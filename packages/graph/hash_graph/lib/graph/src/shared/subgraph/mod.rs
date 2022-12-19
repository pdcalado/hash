use std::{collections::HashSet, fmt::Debug, time::SystemTime};

use chrono::DateTime;
use edges::Edges;

use crate::{
    shared::identifier::GraphElementEditionId,
    subgraph::{
        edges::GraphResolveDepths,
        query::{TimeProjection, TimeResolver},
        vertices::Vertices,
    },
};

pub mod edges;
pub mod query;
pub mod vertices;

#[derive(Debug)]
pub struct Subgraph {
    pub roots: HashSet<GraphElementEditionId>,
    pub vertices: Vertices,
    pub edges: Edges,
    pub depths: GraphResolveDepths,
    pub time_projection: TimeProjection,
    pub resolved_time_projection: TimeProjection,
}

impl Subgraph {
    #[must_use]
    pub fn new(depths: GraphResolveDepths, time_projection: TimeProjection) -> Self {
        Self {
            roots: HashSet::new(),
            vertices: Vertices::default(),
            edges: Edges::default(),
            depths,
            resolved_time_projection: time_projection.resolve(DateTime::from(SystemTime::now())),
            time_projection,
        }
    }
}
