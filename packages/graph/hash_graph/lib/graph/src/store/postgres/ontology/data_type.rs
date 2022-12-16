use async_trait::async_trait;
use error_stack::{IntoReport, Result, ResultExt};
use tokio_postgres::GenericClient;
use type_system::DataType;

use crate::{
    identifier::ontology::OntologyTypeEditionId,
    ontology::{DataTypeWithMetadata, OntologyElementMetadata, OntologyTypeWithMetadata},
    provenance::{OwnedById, UpdatedById},
    store::{
        crud::Read,
        postgres::{DependencyContext, DependencyStatus},
        query::Filter,
        AsClient, DataTypeStore, InsertionError, PostgresStore, QueryError, UpdateError,
    },
    subgraph::{edges::GraphResolveDepths, Subgraph},
};

#[async_trait]
impl<C: AsClient> Read<DataTypeWithMetadata> for PostgresStore<C> {
    async fn read(
        &self,
        filter: &Filter<DataTypeWithMetadata>,
    ) -> Result<Vec<DataTypeWithMetadata>, QueryError> {
        self.read_ontology_type(filter).await
    }

    #[tracing::instrument(level = "trace", skip(self, dependency_context, subgraph))]
    async fn traverse(
        &self,
        data_type_id: &OntologyTypeEditionId,
        dependency_context: &mut DependencyContext,
        subgraph: &mut Subgraph,
        current_resolve_depth: GraphResolveDepths,
    ) -> Result<(), QueryError> {
        let dependency_status = dependency_context
            .ontology_dependency_map
            .insert(data_type_id, current_resolve_depth);

        let _data_type = match dependency_status {
            DependencyStatus::Unresolved => {
                subgraph
                    .get_or_read::<DataTypeWithMetadata>(self, data_type_id)
                    .await?
            }
            DependencyStatus::Resolved => return Ok(()),
        };

        // TODO: data types currently have no references to other types, so we don't need to do
        //       anything here
        //   see https://app.asana.com/0/1200211978612931/1202464168422955/f

        Ok(())
    }
}

#[async_trait]
impl<C: AsClient> DataTypeStore for PostgresStore<C> {
    #[tracing::instrument(level = "info", skip(self, data_type))]
    async fn create_data_type(
        &mut self,
        data_type: DataType,
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

        let (_, metadata) = transaction
            .create(data_type, owned_by_id, updated_by_id)
            .await?;

        transaction
            .client
            .commit()
            .await
            .into_report()
            .change_context(InsertionError)?;

        Ok(metadata)
    }

    async fn update_data_type(
        &mut self,
        data_type: DataType,
        updated_by_id: UpdatedById,
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
        let previous_owned_by_id = Read::<DataTypeWithMetadata>::read_one(
            &transaction,
            &Filter::for_base_uri(data_type.id().base_uri()),
        )
        .await
        .change_context(UpdateError)?
        .metadata()
        .owned_by_id();

        let (_, metadata) = transaction
            .update::<DataType>(data_type, previous_owned_by_id, updated_by_id)
            .await?;

        transaction
            .client
            .commit()
            .await
            .into_report()
            .change_context(UpdateError)?;

        Ok(metadata)
    }
}
