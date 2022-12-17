use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OntologyEdgeKind {
    /// An ontology type can inherit from another ontology type.
    InheritsFrom,
    /// A [`PropertyType`] or [`DataType`] can reference a [`DataType`] to constrain values.
    ///
    /// [`DataType`]: type_system::DataType
    /// [`PropertyType`]: type_system::PropertyType
    ConstrainsValuesOn,
    /// An [`EntityType`] or [`PropertyType`] can reference a [`PropertyType`] to constrain
    /// properties.
    ///
    /// [`PropertyType`]: type_system::PropertyType
    /// [`EntityType`]: type_system::EntityType
    ConstrainsPropertiesOn,
    /// An [`EntityType`] can reference a link [`EntityType`] to constrain the existence of
    /// certain kinds of links.
    ///
    /// [`EntityType`]: type_system::EntityType
    ConstrainsLinksOn,
    /// An [`EntityType`] can reference an [`EntityType`] to constrain the target entities of
    /// certain kinds of links.
    ///
    /// [`EntityType`]: type_system::EntityType
    ConstrainsLinkDestinationsOn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum KnowledgeGraphEdgeKind {
    /// This link [`Entity`] has another [`Entity`] on its 'left' endpoint.
    ///
    /// The `reverse` of this would be the equivalent of saying an [`Entity`] has an outgoing
    /// `Link` [`Entity`].
    ///
    /// [`Entity`]: crate::knowledge::Entity
    HasLeftEntity,
    /// This link [`Entity`] has another [`Entity`] on its 'right' endpoint.
    ///
    /// [`Entity`]: crate::knowledge::Entity
    HasRightEntity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SharedEdgeKind {
    /// An [`Entity`] is of an [`EntityType`].
    ///
    /// [`Entity`]: crate::knowledge::Entity
    /// [`EntityType`]: type_system::EntityType
    IsOfType,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct EdgeResolveDepths {
    pub incoming: u8,
    pub outgoing: u8,
}

impl EdgeResolveDepths {
    #[expect(
        clippy::useless_let_if_seq,
        reason = "Using a mutable variable is more readable"
    )]
    pub fn update(&mut self, other: Self) -> bool {
        let mut changed = false;
        if other.incoming > self.incoming {
            self.incoming = other.incoming;
            changed = true;
        }
        if other.outgoing > self.outgoing {
            self.outgoing = other.outgoing;
            changed = true;
        }
        changed
    }
}

// TODO: Replace with `EdgeResolveDepths`
//   see https://app.asana.com/0/1201095311341924/1203399511264512/f
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct OutgoingEdgeResolveDepth {
    pub outgoing: u8,
    #[serde(default, skip)]
    #[doc(hidden)]
    /// This is not used yet, but will be used in the future to support incoming edges.
    pub incoming: u8,
}

impl OutgoingEdgeResolveDepth {
    #[expect(
        clippy::useless_let_if_seq,
        reason = "Be consistent with `EdgeResolveDepths`"
    )]
    pub fn update(&mut self, other: Self) -> bool {
        let mut changed = false;
        if other.outgoing > self.outgoing {
            self.outgoing = other.outgoing;
            changed = true;
        }
        changed
    }
}

pub mod resolve_depth {

    use crate::{
        identifier::{knowledge::EntityEditionId, ontology::OntologyTypeEditionId},
        subgraph::edges::{
            Edge, GraphResolveDepths, KnowledgeGraphEdgeKind, KnowledgeGraphOutwardEdges,
            OntologyEdgeKind, OntologyOutwardEdges, OutgoingEdgeResolveDepth, OutwardEdge,
            SharedEdgeKind,
        },
    };

    pub trait EdgeKind {
        type BaseEditionId: Clone + Sync;
        type TargetEditionId: Clone;
    }

    impl EdgeKind for OntologyEdgeKind {
        type BaseEditionId = OntologyTypeEditionId;
        type TargetEditionId = OntologyTypeEditionId;
    }

    impl EdgeKind for SharedEdgeKind {
        type BaseEditionId = EntityEditionId;
        type TargetEditionId = OntologyTypeEditionId;
    }

    impl EdgeKind for KnowledgeGraphEdgeKind {
        type BaseEditionId = EntityEditionId;
        type TargetEditionId = EntityEditionId;
    }

    pub trait ResolveDepth: Sync {
        type EdgeKind: EdgeKind;

        fn create_edge(
            base: <Self::EdgeKind as EdgeKind>::BaseEditionId,
            right_endpoint: <Self::EdgeKind as EdgeKind>::TargetEditionId,
        ) -> Edge;

        fn update_resolve_depth(resolve_depth: &mut GraphResolveDepths) -> bool;
    }

    pub struct InheritsFromResolveDepth(pub OutgoingEdgeResolveDepth);

    macro_rules! define_resolve_depth {
        (
            $name:ident,
            $edge_kind:ident,
            $kind:ident,
            $edge:ident,
            $outward_edges:ident,
            $edge_target:ident,
            $reversed:literal,
            $resolve_depth:ident,
            $parameter:ident
        ) => {
            pub struct $name;

            impl ResolveDepth for $name {
                type EdgeKind = $edge_kind;

                fn create_edge(
                    base: <Self::EdgeKind as EdgeKind>::BaseEditionId,
                    right_endpoint: <Self::EdgeKind as EdgeKind>::TargetEditionId,
                ) -> Edge {
                    Edge::$edge {
                        edition_id: base,
                        outward_edge: $outward_edges::$edge_target(OutwardEdge {
                            kind: $edge_kind::$kind,
                            reversed: $reversed,
                            right_endpoint,
                        }),
                    }
                }

                fn update_resolve_depth(resolve_depth: &mut GraphResolveDepths) -> bool {
                    if resolve_depth.$resolve_depth.$parameter == 0 {
                        false
                    } else {
                        resolve_depth.$resolve_depth.$parameter -= 1;
                        true
                    }
                }
            }
        };
    }

    define_resolve_depth!(
        InheritsFrom,
        OntologyEdgeKind,
        InheritsFrom,
        Ontology,
        OntologyOutwardEdges,
        ToOntology,
        false,
        inherits_from,
        outgoing
    );

    define_resolve_depth!(
        ConstrainsValuesOn,
        OntologyEdgeKind,
        ConstrainsValuesOn,
        Ontology,
        OntologyOutwardEdges,
        ToOntology,
        false,
        constrains_values_on,
        outgoing
    );

    define_resolve_depth!(
        ConstrainsPropertiesOn,
        OntologyEdgeKind,
        ConstrainsPropertiesOn,
        Ontology,
        OntologyOutwardEdges,
        ToOntology,
        false,
        constrains_properties_on,
        outgoing
    );

    define_resolve_depth!(
        ConstrainsLinksOn,
        OntologyEdgeKind,
        ConstrainsLinksOn,
        Ontology,
        OntologyOutwardEdges,
        ToOntology,
        false,
        constrains_links_on,
        outgoing
    );

    define_resolve_depth!(
        ConstrainsLinkDestinationsOn,
        OntologyEdgeKind,
        ConstrainsLinkDestinationsOn,
        Ontology,
        OntologyOutwardEdges,
        ToOntology,
        false,
        constrains_link_destinations_on,
        outgoing
    );

    define_resolve_depth!(
        HasLeftEntity,
        KnowledgeGraphEdgeKind,
        HasLeftEntity,
        KnowledgeGraph,
        KnowledgeGraphOutwardEdges,
        ToKnowledgeGraph,
        false,
        has_left_entity,
        outgoing
    );

    define_resolve_depth!(
        HasRightEntity,
        KnowledgeGraphEdgeKind,
        HasRightEntity,
        KnowledgeGraph,
        KnowledgeGraphOutwardEdges,
        ToKnowledgeGraph,
        false,
        has_right_entity,
        outgoing
    );

    define_resolve_depth!(
        HasOutgoingLink,
        KnowledgeGraphEdgeKind,
        HasLeftEntity,
        KnowledgeGraph,
        KnowledgeGraphOutwardEdges,
        ToKnowledgeGraph,
        true,
        has_left_entity,
        incoming
    );

    define_resolve_depth!(
        HasIncomingLink,
        KnowledgeGraphEdgeKind,
        HasRightEntity,
        KnowledgeGraph,
        KnowledgeGraphOutwardEdges,
        ToKnowledgeGraph,
        true,
        has_right_entity,
        incoming
    );
}

/// TODO: DOC - <https://app.asana.com/0/0/1203438518991188/f>
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GraphResolveDepths {
    pub inherits_from: OutgoingEdgeResolveDepth,
    pub constrains_values_on: OutgoingEdgeResolveDepth,
    pub constrains_properties_on: OutgoingEdgeResolveDepth,
    pub constrains_links_on: OutgoingEdgeResolveDepth,
    pub constrains_link_destinations_on: OutgoingEdgeResolveDepth,
    pub is_of_type: OutgoingEdgeResolveDepth,
    pub has_left_entity: EdgeResolveDepths,
    pub has_right_entity: EdgeResolveDepths,
}

impl GraphResolveDepths {
    #[expect(
        clippy::useless_let_if_seq,
        reason = "Using a mutable variable is more readable"
    )]
    pub fn update(&mut self, other: Self) -> bool {
        let mut changed = false;
        if self.inherits_from.update(other.inherits_from) {
            changed = true;
        }
        if self.constrains_values_on.update(other.constrains_values_on) {
            changed = true;
        }
        if self
            .constrains_properties_on
            .update(other.constrains_properties_on)
        {
            changed = true;
        }
        if self.constrains_links_on.update(other.constrains_links_on) {
            changed = true;
        }
        if self
            .constrains_link_destinations_on
            .update(other.constrains_link_destinations_on)
        {
            changed = true;
        }
        if self.is_of_type.update(other.is_of_type) {
            changed = true;
        }
        if self.has_left_entity.update(other.has_left_entity) {
            changed = true;
        }
        if self.has_right_entity.update(other.has_right_entity) {
            changed = true;
        }
        changed
    }
}
