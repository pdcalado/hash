use std::{collections::HashSet, fmt::Debug};

use edges::Edges;

use crate::{
    identifier::time::{ResolvedTimeProjection, TimeProjection},
    shared::identifier::GraphElementVertexId,
    subgraph::{edges::GraphResolveDepths, vertices::Vertices},
};

pub mod edges;
pub mod query;
pub mod vertices;

#[derive(Debug)]
pub struct Subgraph {
    pub roots: HashSet<GraphElementVertexId>,
    pub vertices: Vertices,
    pub edges: Edges,
    pub depths: GraphResolveDepths,
    pub time_projection: TimeProjection,
    pub resolved_time_projection: ResolvedTimeProjection,
}

impl Subgraph {
    #[must_use]
    pub fn new(
        depths: GraphResolveDepths,
        time_projection: TimeProjection,
        resolved_time_projection: ResolvedTimeProjection,
    ) -> Self {
        Self {
            roots: HashSet::new(),
            vertices: Vertices::default(),
            edges: Edges::default(),
            depths,
            time_projection,
            resolved_time_projection,
        }
    }
}
