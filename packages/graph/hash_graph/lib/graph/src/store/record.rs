use std::{
    collections::hash_map::{RandomState, RawEntryMut},
    hash::Hash,
};

use crate::{
    identifier::{time::TimeAxis, GraphElementVertexId},
    store::query::{Filter, QueryPath},
    subgraph::Subgraph,
};

/// A record stored in the [`store`].
///
/// [`store`]: crate::store
pub trait Record: Sized + Send {
    type EditionId: Clone + PartialEq + Eq + Hash + Send + Sync;
    type VertexId: Clone + PartialEq + Eq + Hash + Send + Sync + Into<GraphElementVertexId>;
    type QueryPath<'p>: QueryPath + Send + Sync;

    fn edition_id(&self) -> &Self::EditionId;

    fn vertex_id(&self, time_axis: TimeAxis) -> Self::VertexId;

    fn create_filter_for_vertex_id(vertex_id: &Self::VertexId) -> Filter<Self>;

    fn subgraph_entry<'s>(
        subgraph: &'s mut Subgraph,
        vertex_id: &Self::VertexId,
    ) -> RawEntryMut<'s, Self::VertexId, Self, RandomState>;

    fn insert_into_subgraph(self, subgraph: &mut Subgraph) -> &Self {
        let vertex_id = self.vertex_id(subgraph.resolved_time_projection.time_axis());
        Self::subgraph_entry(subgraph, &vertex_id)
            .or_insert(vertex_id, self)
            .1
    }

    fn insert_into_subgraph_as_root(self, subgraph: &mut Subgraph) -> &Self {
        let vertex_id = self.vertex_id(subgraph.resolved_time_projection.time_axis());
        subgraph.roots.insert(vertex_id.clone().into());
        Self::subgraph_entry(subgraph, &vertex_id)
            .or_insert(vertex_id, self)
            .1
    }
}
