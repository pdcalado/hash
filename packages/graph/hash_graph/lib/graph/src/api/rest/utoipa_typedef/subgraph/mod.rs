mod edges;
mod vertices;

use serde::Serialize;
use utoipa::ToSchema;

pub use self::{
    edges::{Edges, KnowledgeGraphRootedEdges, OntologyRootedEdges},
    vertices::{
        KnowledgeGraphVertex, KnowledgeGraphVertices, OntologyVertex, OntologyVertices, Vertex,
        Vertices,
    },
};
use crate::{
    identifier::GraphElementEditionId,
    subgraph::{edges::GraphResolveDepths, query::TimeProjection},
};

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Subgraph {
    roots: Vec<GraphElementEditionId>,
    vertices: Vertices,
    edges: Edges,
    depths: GraphResolveDepths,
    time_projection: TimeProjection,
    resolved_time_projection: TimeProjection,
}

impl From<crate::subgraph::Subgraph> for Subgraph {
    fn from(subgraph: crate::subgraph::Subgraph) -> Self {
        Self {
            roots: subgraph.roots.into_iter().collect(),
            vertices: subgraph.vertices.into(),
            edges: subgraph.edges.into(),
            depths: subgraph.depths,
            time_projection: subgraph.time_projection,
            resolved_time_projection: subgraph.resolved_time_projection,
        }
    }
}
