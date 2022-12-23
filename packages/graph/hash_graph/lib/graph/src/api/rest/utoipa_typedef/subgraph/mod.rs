mod edges;
mod vertices;

use serde::Serialize;
use utoipa::ToSchema;

pub use self::{
    edges::{Edges, KnowledgeGraphOutwardEdges, KnowledgeGraphRootedEdges, OntologyRootedEdges},
    vertices::{
        KnowledgeGraphVertex, KnowledgeGraphVertices, OntologyVertex, OntologyVertices, Vertex,
        Vertices,
    },
};
use crate::{
    identifier::{
        time::{ResolvedTimeProjection, TimeProjection},
        GraphElementVertexId,
    },
    subgraph::edges::GraphResolveDepths,
};

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Subgraph {
    roots: Vec<GraphElementVertexId>,
    vertices: Vertices,
    edges: Edges,
    depths: GraphResolveDepths,
    time_projection: TimeProjection,
    resolved_time_projection: ResolvedTimeProjection,
}

impl From<crate::subgraph::Subgraph> for Subgraph {
    fn from(subgraph: crate::subgraph::Subgraph) -> Self {
        let vertices = subgraph.vertices.into();
        let edges = Edges::from_vertices_and_store_edges(subgraph.edges, &vertices);
        Self {
            roots: subgraph.roots.into_iter().collect(),
            vertices,
            edges,
            depths: subgraph.depths,
            time_projection: subgraph.time_projection,
            resolved_time_projection: subgraph.resolved_time_projection,
        }
    }
}
