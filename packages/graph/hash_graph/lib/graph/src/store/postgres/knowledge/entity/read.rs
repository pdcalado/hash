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
        knowledge::{EntityEditionId, EntityId, EntityRecordId, EntityVersion},
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
            resolve_depth::{HasIncomingLink, HasLeftEntity, HasOutgoingLink, HasRightEntity},
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

        // helper method to read the earliest entity in a query result
        // TODO: this is very slow, we should update structural querying to be
        //       able to  get the first timestamp of something efficiently
        async fn read_earliest(
            store: &impl Read<Entity>,
            filter: &Filter<'_, Entity>,
        ) -> Result<impl IntoIterator<Item = Entity, IntoIter: Send>, QueryError> {
            let entities = store.read(filter).await?;
            let mut earliest_entities = Vec::with_capacity(entities.len());
            for entity in entities {
                earliest_entities.push(
                    store
                        .read(&Filter::for_entity_by_entity_id(
                            entity.edition_id().base_id(),
                        ))
                        .await?
                        .into_iter()
                        .min_by_key(|entity| {
                            entity
                                .metadata()
                                .edition_id()
                                .version()
                                .transaction_time()
                                .as_start_bound_timestamp()
                        })
                        .expect(
                            "we got the edition id from the entity in the first place, there must \
                             be at least one version",
                        ),
                );
            }
            Ok(earliest_entities)
        }

        if current_resolve_depth.has_left_entity.incoming > 0 {
            // We want to log the time the link entity was *first* added from this
            // entity. We therefore need to find the timestamp of the first link
            // entity
            self.traverse_edge_by_records::<HasOutgoingLink>(
                subgraph,
                entity_edition_id,
                dependency_context,
                current_resolve_depth,
                read_earliest(
                    self,
                    &Filter::for_outgoing_link_by_source_entity_edition_id(*entity_edition_id),
                )
                .await?,
            )
            .await?;
        }

        if current_resolve_depth.has_right_entity.incoming > 0 {
            // We want to log the time the link entity was *first* added from this
            // entity. We therefore need to find the timestamp of the first link
            // entity
            self.traverse_edge_by_records::<HasIncomingLink>(
                subgraph,
                entity_edition_id,
                dependency_context,
                current_resolve_depth,
                read_earliest(
                    self,
                    &Filter::for_incoming_link_by_source_entity_edition_id(*entity_edition_id),
                )
                .await?,
            )
            .await?;
        }

        if current_resolve_depth.has_left_entity.outgoing > 0 {
            // We want to log the time _this_ link entity was *first* added from the
            // left entity. We therefore need to find the timestamp of this entity
            self.traverse_edge_by_records::<HasLeftEntity>(
                subgraph,
                entity_edition_id,
                dependency_context,
                current_resolve_depth,
                read_earliest(
                    self,
                    &Filter::for_left_entity_by_entity_edition_id(*entity_edition_id),
                )
                .await?,
            )
            .await?;
        }

        if current_resolve_depth.has_right_entity.outgoing > 0 {
            // We want to log the time _this_ link entity was *first* added to the
            // right entity. We therefore need to find the timestamp of this entity
            self.traverse_edge_by_records::<HasRightEntity>(
                subgraph,
                entity_edition_id,
                dependency_context,
                current_resolve_depth,
                read_earliest(
                    self,
                    &Filter::for_right_entity_by_entity_edition_id(*entity_edition_id),
                )
                .await?,
            )
            .await?;
        }

        Ok(())
    }
}
