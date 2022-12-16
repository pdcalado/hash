use async_trait::async_trait;
use error_stack::{IntoReport, Result, ResultExt};
use tokio_postgres::GenericClient;
use type_system::{DataTypeReference, PropertyType, PropertyTypeReference};

use crate::{
    identifier::ontology::OntologyTypeEditionId,
    ontology::{
        DataTypeWithMetadata, OntologyElementMetadata, OntologyTypeWithMetadata,
        PropertyTypeWithMetadata,
    },
    provenance::{OwnedById, UpdatedById},
    store::{
        crud::Read,
        postgres::{DependencyContext, DependencyStatus},
        query::Filter,
        AsClient, InsertionError, PostgresStore, PropertyTypeStore, QueryError, UpdateError,
    },
    subgraph::{
        edges::{
            Edge, GraphResolveDepths, OntologyEdgeKind, OntologyOutwardEdges,
            OutgoingEdgeResolveDepth, OutwardEdge,
        },
        Subgraph,
    },
};

#[async_trait]
impl<C: AsClient> Read<PropertyTypeWithMetadata> for PostgresStore<C> {
    async fn read(
        &self,
        filter: &Filter<PropertyTypeWithMetadata>,
    ) -> Result<Vec<PropertyTypeWithMetadata>, QueryError> {
        self.read_ontology_type(filter).await
    }

    #[tracing::instrument(level = "trace", skip(self, dependency_context, subgraph))]
    async fn traverse(
        &self,
        property_type_id: &OntologyTypeEditionId,
        dependency_context: &mut DependencyContext,
        subgraph: &mut Subgraph,
        current_resolve_depth: GraphResolveDepths,
    ) -> Result<(), QueryError> {
        let dependency_status = dependency_context
            .ontology_dependency_map
            .insert(property_type_id, current_resolve_depth);

        let property_type = match dependency_status {
            DependencyStatus::Unresolved => {
                subgraph
                    .get_or_read::<PropertyTypeWithMetadata>(self, property_type_id)
                    .await?
            }
            DependencyStatus::Resolved => return Ok(()),
        };

        // Collecting references before traversing further to avoid having a shared
        // reference to the subgraph when borrowing it mutably
        let data_type_ref_uris =
            (current_resolve_depth.constrains_values_on.outgoing > 0).then(|| {
                property_type
                    .inner()
                    .data_type_references()
                    .into_iter()
                    .map(DataTypeReference::uri)
                    .cloned()
                    .collect::<Vec<_>>()
            });

        let property_type_ref_uris = (current_resolve_depth.constrains_properties_on.outgoing > 0)
            .then(|| {
                property_type
                    .inner()
                    .property_type_references()
                    .into_iter()
                    .map(PropertyTypeReference::uri)
                    .cloned()
                    .collect::<Vec<_>>()
            });

        if let Some(data_type_ref_uris) = data_type_ref_uris {
            for data_type_ref in data_type_ref_uris {
                subgraph.edges.insert(Edge::Ontology {
                    edition_id: property_type_id.clone(),
                    outward_edge: OntologyOutwardEdges::ToOntology(OutwardEdge {
                        kind: OntologyEdgeKind::ConstrainsValuesOn,
                        reversed: false,
                        right_endpoint: OntologyTypeEditionId::from(&data_type_ref),
                    }),
                });

                Read::<DataTypeWithMetadata>::traverse(
                    self,
                    &OntologyTypeEditionId::from(&data_type_ref),
                    dependency_context,
                    subgraph,
                    GraphResolveDepths {
                        constrains_values_on: OutgoingEdgeResolveDepth {
                            outgoing: current_resolve_depth.constrains_values_on.outgoing - 1,
                            ..current_resolve_depth.constrains_values_on
                        },
                        ..current_resolve_depth
                    },
                )
                .await?;
            }
        }

        if let Some(property_type_ref_uris) = property_type_ref_uris {
            for property_type_ref_uri in property_type_ref_uris {
                subgraph.edges.insert(Edge::Ontology {
                    edition_id: property_type_id.clone(),
                    outward_edge: OntologyOutwardEdges::ToOntology(OutwardEdge {
                        kind: OntologyEdgeKind::ConstrainsPropertiesOn,
                        reversed: false,
                        right_endpoint: OntologyTypeEditionId::from(&property_type_ref_uri),
                    }),
                });

                Read::<PropertyTypeWithMetadata>::traverse(
                    self,
                    &OntologyTypeEditionId::from(&property_type_ref_uri),
                    dependency_context,
                    subgraph,
                    GraphResolveDepths {
                        constrains_properties_on: OutgoingEdgeResolveDepth {
                            outgoing: current_resolve_depth.constrains_properties_on.outgoing - 1,
                            ..current_resolve_depth.constrains_properties_on
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

#[async_trait]
impl<C: AsClient> PropertyTypeStore for PostgresStore<C> {
    #[tracing::instrument(level = "info", skip(self, property_type))]
    async fn create_property_type(
        &mut self,
        property_type: PropertyType,
        owned_by_id: OwnedById,
        updated_by_id: UpdatedById,
    ) -> Result<OntologyElementMetadata, InsertionError> {
        let transaction = PostgresStore::new(
            self.as_mut_client()
                .transaction()
                .await
                .into_report()
                .change_context(InsertionError)?,
        );

        // This clone is currently necessary because we extract the references as we insert them.
        // We can only insert them after the type has been created, and so we currently extract them
        // after as well. See `insert_property_type_references` taking `&property_type`
        let (version_id, metadata) = transaction
            .create(property_type.clone(), owned_by_id, updated_by_id)
            .await?;

        transaction
            .insert_property_type_references(&property_type, version_id)
            .await
            .change_context(InsertionError)
            .attach_printable_lazy(|| {
                format!(
                    "could not insert references for property type: {}",
                    property_type.id()
                )
            })
            .attach_lazy(|| property_type.clone())?;

        transaction
            .client
            .commit()
            .await
            .into_report()
            .change_context(InsertionError)?;

        Ok(metadata)
    }

    async fn update_property_type(
        &mut self,
        property_type: PropertyType,
        updated_by: UpdatedById,
    ) -> Result<OntologyElementMetadata, UpdateError> {
        let transaction = PostgresStore::new(
            self.as_mut_client()
                .transaction()
                .await
                .into_report()
                .change_context(UpdateError)?,
        );

        // TODO - address potential race condition
        //  https://app.asana.com/0/1202805690238892/1203201674100967/f
        let previous_owned_by_id = Read::<PropertyTypeWithMetadata>::read_one(
            &transaction,
            &Filter::for_base_uri(property_type.id().base_uri()),
        )
        .await
        .change_context(UpdateError)?
        .metadata()
        .owned_by_id();

        // This clone is currently necessary because we extract the references as we insert them.
        // We can only insert them after the type has been created, and so we currently extract them
        // after as well. See `insert_property_type_references` taking `&property_type`
        let (version_id, metadata) = transaction
            .update::<PropertyType>(property_type.clone(), previous_owned_by_id, updated_by)
            .await?;

        transaction
            .insert_property_type_references(&property_type, version_id)
            .await
            .change_context(UpdateError)
            .attach_printable_lazy(|| {
                format!(
                    "could not insert references for property type: {}",
                    property_type.id()
                )
            })
            .attach_lazy(|| property_type.clone())?;

        transaction
            .client
            .commit()
            .await
            .into_report()
            .change_context(UpdateError)?;

        Ok(metadata)
    }
}
