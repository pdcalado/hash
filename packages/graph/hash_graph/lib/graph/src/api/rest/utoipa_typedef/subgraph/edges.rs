use std::collections::{hash_map::Entry, HashMap};

use serde::Serialize;
use type_system::uri::BaseUri;
use utoipa::{
    openapi::{Array, ObjectBuilder, OneOfBuilder, Ref, Schema},
    ToSchema,
};

use crate::{
    identifier::{
        knowledge::{EntityId, EntityIdAndTimestamp, EntityVersion},
        ontology::{OntologyTypeEditionId, OntologyTypeVersion},
    },
    subgraph::edges::{KnowledgeGraphEdgeKind, OntologyOutwardEdges, OutwardEdge, SharedEdgeKind},
};

#[derive(Debug, Hash, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum KnowledgeGraphOutwardEdges {
    ToKnowledgeGraph(OutwardEdge<KnowledgeGraphEdgeKind, EntityIdAndTimestamp>),
    ToOntology(OutwardEdge<SharedEdgeKind, OntologyTypeEditionId>),
}

// WARNING: This MUST be kept up to date with the enum variants.
//   We have to do this because utoipa doesn't understand serde untagged:
//   https://github.com/juhaku/utoipa/issues/320
impl ToSchema for KnowledgeGraphOutwardEdges {
    fn schema() -> Schema {
        OneOfBuilder::new()
            .item(<OutwardEdge<KnowledgeGraphEdgeKind, EntityIdAndTimestamp>>::schema())
            .item(<OutwardEdge<SharedEdgeKind, OntologyTypeEditionId>>::schema())
            .into()
    }
}

#[derive(Default, Debug, Serialize, ToSchema)]
#[serde(transparent)]
pub struct KnowledgeGraphRootedEdges(
    pub HashMap<EntityId, HashMap<EntityVersion, Vec<KnowledgeGraphOutwardEdges>>>,
);

#[derive(Default, Debug, Serialize, ToSchema)]
#[serde(transparent)]
pub struct OntologyRootedEdges(
    pub HashMap<BaseUri, HashMap<OntologyTypeVersion, Vec<OntologyOutwardEdges>>>,
);

#[derive(Serialize)]
pub struct Edges {
    #[serde(flatten)]
    pub ontology: OntologyRootedEdges,
    #[serde(flatten)]
    pub knowledge_graph: KnowledgeGraphRootedEdges,
}

impl From<crate::subgraph::edges::Edges> for Edges {
    fn from(edges: crate::subgraph::edges::Edges) -> Self {
        Self {
            ontology: OntologyRootedEdges(edges.ontology.into_iter().fold(
                HashMap::new(),
                |mut map, (id, edges)| {
                    let edges = edges.into_iter().collect();
                    match map.entry(id.base_id().clone()) {
                        Entry::Occupied(entry) => {
                            entry.into_mut().insert(id.version(), edges);
                        }
                        Entry::Vacant(entry) => {
                            entry.insert(HashMap::from([(id.version(), edges)]));
                        }
                    }
                    map
                },
            )),
            knowledge_graph: KnowledgeGraphRootedEdges(edges.knowledge_graph.into_iter().fold(
                HashMap::new(),
                |mut map, (id, edges)| {
                    let edges = edges.into_iter().map(|edge| {
                        match edge {
                            crate::subgraph::edges::KnowledgeGraphOutwardEdges::ToOntology(edge) => KnowledgeGraphOutwardEdges::ToOntology(edge),
                            crate::subgraph::edges::KnowledgeGraphOutwardEdges::ToKnowledgeGraph(edge) => {
                                KnowledgeGraphOutwardEdges::ToKnowledgeGraph(OutwardEdge {
                                    kind: edge.kind,
                                    reversed: edge.reversed,
                                    right_endpoint: EntityIdAndTimestamp::new(
                                         edge.right_endpoint.base_id(),
                                         edge.right_endpoint.version().transaction_time().as_start_bound_timestamp())

                                })
                            }
                        }
                    }).collect();
                    match map.entry(id.base_id()) {
                        Entry::Occupied(entry) => {
                            entry.into_mut().insert(id.version(), edges);
                        }
                        Entry::Vacant(entry) => {
                            entry.insert(HashMap::from([(id.version(), edges)]));
                        }
                    }
                    map
                },
            )),
        }
    }
}

// Utoipa generates `Edges` as an empty object if we don't manually do it, and we can't use
// allOf because the generator can't handle it
impl ToSchema for Edges {
    fn schema() -> Schema {
        ObjectBuilder::new()
            .additional_properties(Some(Schema::from(
                ObjectBuilder::new().additional_properties(Some(Array::new(
                    OneOfBuilder::new()
                        .item(Ref::from_schema_name("OntologyOutwardEdges"))
                        .item(Ref::from_schema_name("KnowledgeGraphOutwardEdges")),
                ))),
            )))
            .into()
    }
}
