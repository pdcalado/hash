//! Store interface for CRUD operations.
//!
//! The traits defined in this module are used in [`Store`] to create, read, update, and delete
//! entries. They form a unified access to the [`Store`], so it's possible to add operations to the
//! [`Store`] without changing the [`Store`] implementation.
//!
//! [`Store`]: crate::store::Store

use async_trait::async_trait;
use error_stack::{ensure, Report, Result};

use crate::{
    store::{postgres::DependencyContext, query::Filter, QueryError, Record},
    subgraph::{
        edges::{
            resolve_depth::{EdgeKind, ResolveDepth},
            GraphResolveDepths,
        },
        query::StructuralQuery,
        Subgraph,
    },
};

/// Read access to a [`Store`].
///
/// [`Store`]: crate::store::Store
// TODO: Use queries, which are passed to the query-endpoint
//   see https://app.asana.com/0/1202805690238892/1202979057056097/f
#[async_trait]
pub trait Read<R: Record + Send + Sync>: Sync {
    // TODO: Return a stream of `R` instead
    //   see https://app.asana.com/0/1202805690238892/1202923536131158/f
    /// Returns a value from the [`Store`] specified by the passed `query`.
    ///
    /// [`Store`]: crate::store::Store
    async fn read(&self, query: &Filter<R>) -> Result<Vec<R>, QueryError>;

    #[tracing::instrument(level = "info", skip(self, query))]
    async fn read_one(&self, query: &Filter<R>) -> Result<R, QueryError> {
        let mut records = self.read(query).await?;
        ensure!(
            records.len() <= 1,
            Report::new(QueryError).attach_printable(format!(
                "Expected exactly one record to be returned from the query but {} were returned",
                records.len(),
            ))
        );
        records.pop().ok_or_else(|| {
            Report::new(QueryError).attach_printable(
                "Expected exactly one record to be returned from the query but none was returned",
            )
        })
    }

    async fn read_by_query(&self, query: &StructuralQuery<R>) -> Result<Subgraph, QueryError> {
        let StructuralQuery {
            ref filter,
            graph_resolve_depths,
        } = *query;

        let mut subgraph = Subgraph::new(graph_resolve_depths);
        let mut dependency_context = DependencyContext::default();

        for record in self.read(filter).await? {
            let edition_id = record.edition_id().clone();
            // Insert the vertex into the subgraph to avoid another lookup when traversing it
            subgraph.insert(record);

            self.traverse(
                &edition_id,
                &mut dependency_context,
                &mut subgraph,
                graph_resolve_depths,
            )
            .await?;

            subgraph.roots.insert(edition_id.into());
        }

        Ok(subgraph)
    }

    async fn traverse(
        &self,
        edition_id: &R::EditionId,
        dependency_context: &mut DependencyContext,
        subgraph: &mut Subgraph,
        resolve_depth: GraphResolveDepths,
    ) -> Result<(), QueryError>;

    async fn traverse_edge_by_filter<'a, E: ResolveDepth>(
        &self,
        subgraph: &mut Subgraph,
        edition_id: &'a <E::EdgeKind as EdgeKind>::BaseEditionId,
        dependency_context: &mut DependencyContext,
        mut resolve_depth: GraphResolveDepths,
        filter: impl FnOnce(&'a <E::EdgeKind as EdgeKind>::BaseEditionId) -> Filter<'a, R> + Send,
    ) -> Result<(), QueryError>
    where
        E::EdgeKind: EdgeKind<TargetEditionId = R::EditionId>,
    {
        if !E::update_resolve_depth(&mut resolve_depth) {
            return Ok(());
        }

        self.traverse_edge_by_records::<E>(
            subgraph,
            edition_id,
            dependency_context,
            resolve_depth,
            self.read(&filter(edition_id)).await?,
        )
        .await
    }

    async fn traverse_edge_by_records<'a, E: ResolveDepth>(
        &self,
        subgraph: &mut Subgraph,
        edition_id: &'a <E::EdgeKind as EdgeKind>::BaseEditionId,
        dependency_context: &mut DependencyContext,
        resolve_depth: GraphResolveDepths,
        records: impl IntoIterator<Item = R, IntoIter: Send> + Send,
    ) -> Result<(), QueryError>
    where
        E::EdgeKind: EdgeKind<TargetEditionId = R::EditionId>,
    {
        for record in records {
            let endpoint_edition_id = record.edition_id().clone();
            subgraph.insert(record);

            self.traverse(
                &endpoint_edition_id,
                dependency_context,
                subgraph,
                resolve_depth,
            )
            .await?;

            subgraph
                .edges
                .insert(E::create_edge(edition_id.clone(), endpoint_edition_id));
        }
        Ok(())
    }
}

// TODO: Add remaining CRUD traits (but probably don't implement the `D`-part)
