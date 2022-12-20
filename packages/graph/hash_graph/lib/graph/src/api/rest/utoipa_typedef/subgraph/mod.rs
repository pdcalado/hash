mod edges;
mod vertices;

use serde::Serialize;
use utoipa::{
    openapi::{OneOfBuilder, Schema},
    ToSchema,
};

pub use self::{
    edges::{
        Edges, KnowledgeGraphOutwardEdges, KnowledgeGraphRootedEdges, OntologyOutwardEdges,
        OntologyRootedEdges,
    },
    vertices::{
        KnowledgeGraphVertex, KnowledgeGraphVertices, OntologyVertex, OntologyVertices, Vertex,
        Vertices,
    },
};
use crate::{
    api::rest::utoipa_typedef::EntityIdAndTimestamp, identifier::ontology::OntologyTypeEditionId,
    subgraph::edges::GraphResolveDepths,
};

#[derive(Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(untagged)]
pub enum GraphElementEditionId {
    Ontology(OntologyTypeEditionId),
    KnowledgeGraph(EntityIdAndTimestamp),
}

// WARNING: This MUST be kept up to date with the enum variants.
//   We have to do this because utoipa doesn't understand serde untagged:
//   https://github.com/juhaku/utoipa/issues/320
impl ToSchema for GraphElementEditionId {
    fn schema() -> Schema {
        OneOfBuilder::new()
            .item(OntologyTypeEditionId::schema())
            .item(EntityIdAndTimestamp::schema())
            .into()
    }
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Subgraph {
    roots: Vec<GraphElementEditionId>,
    vertices: Vertices,
    edges: Edges,
    depths: GraphResolveDepths,
}

impl From<crate::subgraph::Subgraph> for Subgraph {
    fn from(subgraph: crate::subgraph::Subgraph) -> Self {
        let vertices = subgraph.vertices.into();
        let edges = Edges::from_vertices_and_store_edges(subgraph.edges, &vertices);
        Self {
            roots: subgraph
                .roots
                .into_iter()
                .map(|id| match id {
                    crate::identifier::GraphElementEditionId::Ontology(id) => {
                        GraphElementEditionId::Ontology(id)
                    }
                    crate::identifier::GraphElementEditionId::KnowledgeGraph(id) => {
                        GraphElementEditionId::KnowledgeGraph(EntityIdAndTimestamp {
                            base_id: id.base_id(),
                            timestamp: id.version().transaction_time().from,
                        })
                    }
                })
                .collect(),
            vertices,
            edges,
            depths: subgraph.depths,
        }
    }
}
