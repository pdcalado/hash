use std::str::FromStr;

use async_trait::async_trait;
use error_stack::{IntoReport, Result, ResultExt};
use futures::{StreamExt, TryStreamExt};
use tokio_postgres::GenericClient;
use type_system::uri::VersionedUri;
use uuid::Uuid;

use crate::{
    identifier::{
        account::AccountId,
        knowledge::{
            EntityEditionId, EntityId, EntityIdAndTimestamp, EntityRecordId, EntityVersion,
        },
        ontology::OntologyTypeEditionId,
        DecisionTimespan, TransactionTimespan,
    },
    knowledge::{Entity, EntityProperties, EntityQueryPath, EntityUuid, LinkData},
    ontology::{EntityTypeQueryPath, EntityTypeWithMetadata},
    provenance::{OwnedById, ProvenanceMetadata, UpdatedById},
    store::{
        crud::Read,
        postgres::{
            query::{Distinctness, SelectCompiler},
            DependencyContext, DependencyStatus,
        },
        query::Filter,
        AsClient, PostgresStore, QueryError, Record,
    },
    subgraph::{
        edges::{
            Edge, EdgeResolveDepths, GraphResolveDepths, KnowledgeGraphEdgeKind,
            KnowledgeGraphOutwardEdges, OutgoingEdgeResolveDepth, OutwardEdge, SharedEdgeKind,
        },
        Subgraph,
    },
};

#[async_trait]
impl<C: AsClient> Read<Entity> for PostgresStore<C> {
    #[tracing::instrument(level = "info", skip(self))]
    async fn read(&self, filter: &Filter<Entity>) -> Result<Vec<Entity>, QueryError> {
        // We can't define these inline otherwise we'll drop while borrowed
        let left_entity_uuid_path = EntityQueryPath::LeftEntity(Box::new(EntityQueryPath::Uuid));
        let left_owned_by_id_query_path =
            EntityQueryPath::LeftEntity(Box::new(EntityQueryPath::OwnedById));
        let right_entity_uuid_path = EntityQueryPath::RightEntity(Box::new(EntityQueryPath::Uuid));
        let right_owned_by_id_query_path =
            EntityQueryPath::RightEntity(Box::new(EntityQueryPath::OwnedById));

        let mut compiler = SelectCompiler::new();

        let owned_by_id_index = compiler.add_selection_path(&EntityQueryPath::OwnedById);
        let entity_uuid_index = compiler.add_selection_path(&EntityQueryPath::Uuid);
        let record_id_index = compiler.add_distinct_selection_with_ordering(
            &EntityQueryPath::RecordId,
            Distinctness::Distinct,
            None,
        );
        let decision_time_index = compiler.add_selection_path(&EntityQueryPath::DecisionTime);
        let transaction_time_index = compiler.add_selection_path(&EntityQueryPath::TransactionTime);

        let type_id_index =
            compiler.add_selection_path(&EntityQueryPath::Type(EntityTypeQueryPath::VersionedUri));

        let properties_index = compiler.add_selection_path(&EntityQueryPath::Properties(None));

        let left_entity_uuid_index = compiler.add_selection_path(&left_entity_uuid_path);
        let left_entity_owned_by_id_index =
            compiler.add_selection_path(&left_owned_by_id_query_path);
        let right_entity_uuid_index = compiler.add_selection_path(&right_entity_uuid_path);
        let right_entity_owned_by_id_index =
            compiler.add_selection_path(&right_owned_by_id_query_path);
        let left_to_right_order_index =
            compiler.add_selection_path(&EntityQueryPath::LeftToRightOrder);
        let right_to_left_order_index =
            compiler.add_selection_path(&EntityQueryPath::RightToLeftOrder);

        let updated_by_id_index = compiler.add_selection_path(&EntityQueryPath::UpdatedById);

        let archived_index = compiler.add_selection_path(&EntityQueryPath::Archived);

        compiler.add_filter(filter);
        let (statement, parameters) = compiler.compile();

        self.as_client()
            .query_raw(&statement, parameters.iter().copied())
            .await
            .into_report()
            .change_context(QueryError)?
            .map(|row| row.into_report().change_context(QueryError))
            .and_then(|row| async move {
                let properties: EntityProperties =
                    serde_json::from_value(row.get(properties_index))
                        .into_report()
                        .change_context(QueryError)?;
                let entity_type_uri = VersionedUri::from_str(row.get(type_id_index))
                    .into_report()
                    .change_context(QueryError)?;

                let link_data = {
                    let left_owned_by_id: Option<AccountId> =
                        row.get(left_entity_owned_by_id_index);
                    let left_entity_uuid: Option<Uuid> = row.get(left_entity_uuid_index);
                    let right_owned_by_id: Option<AccountId> =
                        row.get(right_entity_owned_by_id_index);
                    let right_entity_uuid: Option<Uuid> = row.get(right_entity_uuid_index);
                    match (
                        left_owned_by_id,
                        left_entity_uuid,
                        right_owned_by_id,
                        right_entity_uuid,
                    ) {
                        (
                            Some(left_owned_by_id),
                            Some(left_entity_uuid),
                            Some(right_owned_by_id),
                            Some(right_entity_uuid),
                        ) => Some(LinkData::new(
                            EntityId::new(
                                OwnedById::new(left_owned_by_id),
                                EntityUuid::new(left_entity_uuid),
                            ),
                            EntityId::new(
                                OwnedById::new(right_owned_by_id),
                                EntityUuid::new(right_entity_uuid),
                            ),
                            row.get(left_to_right_order_index),
                            row.get(right_to_left_order_index),
                        )),
                        (None, None, None, None) => None,
                        _ => unreachable!(
                            "It's not possible to have a link entity with the left entityId or \
                             right entityId unspecified"
                        ),
                    }
                };

                let owned_by_id = OwnedById::new(row.get(owned_by_id_index));
                let entity_uuid = EntityUuid::new(row.get(entity_uuid_index));
                let updated_by_id = UpdatedById::new(row.get(updated_by_id_index));

                Ok(Entity::new(
                    properties,
                    link_data,
                    EntityEditionId::new(
                        EntityId::new(owned_by_id, entity_uuid),
                        EntityRecordId::new(row.get(record_id_index)),
                        EntityVersion::new(
                            DecisionTimespan::new(row.get(decision_time_index)),
                            TransactionTimespan::new(row.get(transaction_time_index)),
                        ),
                    ),
                    entity_type_uri,
                    ProvenanceMetadata::new(updated_by_id),
                    // TODO: only the historic table would have an `archived` field.
                    //   Consider what we should do about that.
                    row.get(archived_index),
                ))
            })
            .try_collect()
            .await
    }

    #[tracing::instrument(level = "info", skip(self, dependency_context, subgraph))]
    async fn traverse(
        &self,
        entity_edition_id: &EntityEditionId,
        dependency_context: &mut DependencyContext,
        subgraph: &mut Subgraph,
        current_resolve_depth: GraphResolveDepths,
    ) -> Result<(), QueryError> {
        let dependency_status = dependency_context
            .knowledge_dependency_map
            .insert(entity_edition_id, current_resolve_depth);

        let entity = match dependency_status {
            DependencyStatus::Unresolved => {
                subgraph
                    .get_or_read::<Entity>(self, entity_edition_id)
                    .await?
            }
            DependencyStatus::Resolved => return Ok(()),
        };

        if current_resolve_depth.is_of_type.outgoing > 0 {
            let entity_type_id = OntologyTypeEditionId::from(entity.metadata().entity_type_id());
            subgraph.edges.insert(Edge::KnowledgeGraph {
                edition_id: *entity_edition_id,
                outward_edge: KnowledgeGraphOutwardEdges::ToOntology(OutwardEdge {
                    kind: SharedEdgeKind::IsOfType,
                    reversed: false,
                    right_endpoint: entity_type_id.clone(),
                }),
            });

            <Self as Read<EntityTypeWithMetadata>>::traverse(
                self,
                &entity_type_id,
                dependency_context,
                subgraph,
                GraphResolveDepths {
                    is_of_type: OutgoingEdgeResolveDepth {
                        outgoing: current_resolve_depth.is_of_type.outgoing - 1,
                        ..current_resolve_depth.is_of_type
                    },
                    ..current_resolve_depth
                },
            )
            .await?;
        }

        if current_resolve_depth.has_left_entity.incoming > 0 {
            for outgoing_link_entity in self
                .read(&Filter::for_outgoing_link_by_source_entity_edition_id(
                    *entity_edition_id,
                ))
                .await?
            {
                // We want to log the time the link entity was *first* added from this
                // entity. We therefore need to find the timestamp of the first link
                // entity
                // TODO: this is very slow, we should update structural querying to be
                //       able to  get the first timestamp of something efficiently
                let mut all_outgoing_link_lower_decision_timestamps: Vec<_> = self
                    .read(&Filter::for_entity_by_entity_id(
                        outgoing_link_entity.edition_id().base_id(),
                    ))
                    .await?
                    .into_iter()
                    .map(|entity| {
                        entity
                            .metadata()
                            .edition_id()
                            .version()
                            .transaction_time()
                            .as_start_bound_timestamp()
                    })
                    .collect();

                all_outgoing_link_lower_decision_timestamps.sort();

                let earliest_timestamp = all_outgoing_link_lower_decision_timestamps
                    .into_iter()
                    .next()
                    .expect(
                        "we got the edition id from the entity in the first place, there must be \
                         at least one version",
                    );

                subgraph.edges.insert(Edge::KnowledgeGraph {
                    edition_id: *entity_edition_id,
                    outward_edge: KnowledgeGraphOutwardEdges::ToKnowledgeGraph(OutwardEdge {
                        // (HasLeftEntity, reversed=true) is equivalent to an
                        // outgoing link `Entity`
                        kind: KnowledgeGraphEdgeKind::HasLeftEntity,
                        reversed: true,
                        right_endpoint: EntityIdAndTimestamp::new(
                            outgoing_link_entity.edition_id().base_id(),
                            earliest_timestamp,
                        ),
                    }),
                });

                let outgoing_link_entity_edition_id = *outgoing_link_entity.edition_id();
                subgraph.insert(outgoing_link_entity);

                Read::<Entity>::traverse(
                    self,
                    &outgoing_link_entity_edition_id,
                    dependency_context,
                    subgraph,
                    GraphResolveDepths {
                        has_left_entity: EdgeResolveDepths {
                            incoming: current_resolve_depth.has_left_entity.incoming - 1,
                            ..current_resolve_depth.has_left_entity
                        },
                        ..current_resolve_depth
                    },
                )
                .await?;
            }
        }

        if current_resolve_depth.has_right_entity.incoming > 0 {
            for incoming_link_entity in self
                .read(&Filter::for_incoming_link_by_source_entity_edition_id(
                    *entity_edition_id,
                ))
                .await?
            {
                // We want to log the time the link entity was *first* added from this
                // entity. We therefore need to find the timestamp of the first link
                // entity
                // TODO: this is very slow, we should update structural querying to be
                //       able to get the first timestamp of something efficiently
                let mut all_incoming_link_lower_decision_timestamps: Vec<_> = self
                    .read(&Filter::for_entity_by_entity_id(
                        incoming_link_entity.edition_id().base_id(),
                    ))
                    .await?
                    .into_iter()
                    .map(|entity| {
                        entity
                            .metadata()
                            .edition_id()
                            .version()
                            .transaction_time()
                            .as_start_bound_timestamp()
                    })
                    .collect();

                all_incoming_link_lower_decision_timestamps.sort();

                let earliest_timestamp = all_incoming_link_lower_decision_timestamps
                    .into_iter()
                    .next()
                    .expect(
                        "we got the edition id from the entity in the first place, there must be \
                         at least one version",
                    );

                subgraph.edges.insert(Edge::KnowledgeGraph {
                    edition_id: *entity_edition_id,
                    outward_edge: KnowledgeGraphOutwardEdges::ToKnowledgeGraph(OutwardEdge {
                        // (HasRightEntity, reversed=true) is equivalent to an
                        // incoming link `Entity`
                        kind: KnowledgeGraphEdgeKind::HasRightEntity,
                        reversed: true,
                        right_endpoint: EntityIdAndTimestamp::new(
                            incoming_link_entity.edition_id().base_id(),
                            earliest_timestamp,
                        ),
                    }),
                });

                let incoming_link_entity_edition_id = *incoming_link_entity.edition_id();
                subgraph.insert(incoming_link_entity);

                Read::<Entity>::traverse(
                    self,
                    &incoming_link_entity_edition_id,
                    dependency_context,
                    subgraph,
                    GraphResolveDepths {
                        has_right_entity: EdgeResolveDepths {
                            incoming: current_resolve_depth.has_right_entity.incoming - 1,
                            ..current_resolve_depth.has_right_entity
                        },
                        ..current_resolve_depth
                    },
                )
                .await?;
            }
        }

        if current_resolve_depth.has_left_entity.outgoing > 0 {
            for left_entity in self
                .read(&Filter::for_left_entity_by_entity_edition_id(
                    *entity_edition_id,
                ))
                .await?
            {
                // We want to log the time _this_ link entity was *first* added from the
                // left entity. We therefore need to find the timestamp of this entity
                // TODO: this is very slow, we should update structural querying to be
                //       able to get the first timestamp of something efficiently
                let mut all_self_lower_decision_timestamps: Vec<_> = self
                    .read(&Filter::for_entity_by_entity_id(
                        entity_edition_id.base_id(),
                    ))
                    .await?
                    .into_iter()
                    .map(|entity| {
                        entity
                            .edition_id()
                            .version()
                            .transaction_time()
                            .as_start_bound_timestamp()
                    })
                    .collect();

                all_self_lower_decision_timestamps.sort();

                let earliest_timestamp = all_self_lower_decision_timestamps
                    .into_iter()
                    .next()
                    .expect(
                        "we got the edition id from the entity in the first place, there must be \
                         at least one version",
                    );

                subgraph.edges.insert(Edge::KnowledgeGraph {
                    edition_id: *entity_edition_id,
                    outward_edge: KnowledgeGraphOutwardEdges::ToKnowledgeGraph(OutwardEdge {
                        // (HasLeftEndpoint, reversed=true) is equivalent to an
                        // outgoing `Link` `Entity`
                        kind: KnowledgeGraphEdgeKind::HasLeftEntity,
                        reversed: false,
                        right_endpoint: EntityIdAndTimestamp::new(
                            left_entity.metadata().edition_id().base_id(),
                            earliest_timestamp,
                        ),
                    }),
                });

                let left_entity_edition_id = *left_entity.edition_id();
                subgraph.insert(left_entity);

                Read::<Entity>::traverse(
                    self,
                    &left_entity_edition_id,
                    dependency_context,
                    subgraph,
                    GraphResolveDepths {
                        has_left_entity: EdgeResolveDepths {
                            outgoing: current_resolve_depth.has_left_entity.outgoing - 1,
                            ..current_resolve_depth.has_left_entity
                        },
                        ..current_resolve_depth
                    },
                )
                .await?;
            }
        }

        if current_resolve_depth.has_right_entity.outgoing > 0 {
            for right_entity in self
                .read(&Filter::for_right_entity_by_entity_edition_id(
                    *entity_edition_id,
                ))
                .await?
            {
                // We want to log the time _this_ link entity was *first* added to the
                // right entity. We therefore need to find the timestamp of this entity
                // TODO: this is very slow, we should update structural querying to be
                //       able to  get the first timestamp of something efficiently
                let mut all_self_lower_decision_timestamps: Vec<_> = self
                    .read(&Filter::for_entity_by_entity_id(
                        entity_edition_id.base_id(),
                    ))
                    .await?
                    .into_iter()
                    .map(|entity| {
                        entity
                            .metadata()
                            .edition_id()
                            .version()
                            .transaction_time()
                            .as_start_bound_timestamp()
                    })
                    .collect();

                all_self_lower_decision_timestamps.sort();

                let earliest_timestamp = all_self_lower_decision_timestamps
                    .into_iter()
                    .next()
                    .expect(
                        "we got the edition id from the entity in the first place, there must be \
                         at least one version",
                    );

                subgraph.edges.insert(Edge::KnowledgeGraph {
                    edition_id: *entity_edition_id,
                    outward_edge: KnowledgeGraphOutwardEdges::ToKnowledgeGraph(OutwardEdge {
                        // (HasLeftEndpoint, reversed=true) is equivalent to an
                        // outgoing `Link` `Entity`
                        kind: KnowledgeGraphEdgeKind::HasRightEntity,
                        reversed: false,
                        right_endpoint: EntityIdAndTimestamp::new(
                            right_entity.metadata().edition_id().base_id(),
                            earliest_timestamp,
                        ),
                    }),
                });

                let right_entity_edition_id = *right_entity.edition_id();
                subgraph.insert(right_entity);

                Read::<Entity>::traverse(
                    self,
                    &right_entity_edition_id,
                    dependency_context,
                    subgraph,
                    GraphResolveDepths {
                        has_right_entity: EdgeResolveDepths {
                            outgoing: current_resolve_depth.has_right_entity.outgoing - 1,
                            ..current_resolve_depth.has_right_entity
                        },
                        ..current_resolve_depth
                    },
                )
                .await?;
            }
        }

        Ok(())
    }
}
