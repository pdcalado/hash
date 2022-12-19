use std::fmt::{Debug, Formatter};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::timestamp;

use crate::{
    identifier::time::{DecisionTime, DecisionTimespan, Timespan, Timestamp, TransactionTime},
    knowledge::Entity,
    ontology::{DataTypeWithMetadata, EntityTypeWithMetadata, PropertyTypeWithMetadata},
    store::{query::Filter, Record},
    subgraph::edges::GraphResolveDepths,
};

/// Structural queries are the main entry point to read data from the Graph.
///
/// They are used to query the graph for a set of vertices and edges that match a set of filters.
/// Alongside the filters, the query can specify the depth of the query, which determines how many
/// edges the query will follow from the root vertices. The root vertices are determined by the
/// filters. For example, if the query is for all entities of a certain type, the root vertices will
/// be the entities of that type.
///
/// # Filters
///
/// [`Filter`]s are used to specify which root vertices to include in the query. They consist of a
/// variety of different types of filters, which are described in the [`Filter`] documentation. At
/// the leaf level, filters are composed of [`RecordPath`]s and [`Parameter`]s, which identify the
/// root vertices to include in the query.
///
/// Each [`RecordPath`] is a sequence of tokens, which are used to traverse the graph. For example,
/// a `StructuralQuery<Entity>` with the path `["type", "version"]` will traverse the graph from an
/// entity to its type to the version. When associating the above path with a [`Parameter`] with the
/// value `1` in an equality filter, the query will return all entities whose type has version `1`
/// as a root vertex.
///
/// Depending on the type of the [`StructuralQuery`], different [`RecordPath`]s are valid. Please
/// see the documentation on the implementation of [`Record::QueryPath`] for the valid paths for
/// each type.
///
/// # Depth
///
/// The depth of a query determines how many edges the query will follow from the root vertices. For
/// an in-depth explanation of the depth of a query, please see the documentation on
/// [`GraphResolveDepths`].
///
/// # Examples
///
/// Typically, a structural will be deserialized from a JSON request. The following examples assume,
/// that the type of the request body is `StructuralQuery<Entity>`.
///
/// This will return all entities with the latest version of the `foo` type:
///
/// ```json
/// {
///   "filter": {
///     "all": [
///       {
///         "equal": [
///           { "path": ["type", "baseUri"] },
///           { "parameter": "foo" }
///         ]
///       },
///       {
///         "equal": [
///           { "path": ["type", "version"] },
///           { "parameter": "latest" }
///         ]
///       }
///     ]
///   },
///   "graphResolveDepths": {
///     "inheritsFrom": {
///       "outgoing": 0
///     },
///     "constrainsValuesOn": {
///       "outgoing": 0
///     },
///     "constrainsPropertiesOn": {
///       "outgoing": 0
///     },
///     "constrainsLinksOn": {
///       "outgoing": 0
///     },
///     "constrainsLinkDestinationsOn": {
///       "outgoing": 0
///     },
///     "isOfType": {
///       "outgoing": 0
///     },
///     "hasLeftEntity": {
///       "incoming": 2,
///       "outgoing": 2
///     },
///     "hasRightEntity": {
///       "incoming": 2,
///       "outgoing": 2
///     }
///   }
/// ```
///
/// This query will return any entity, which was either created by or is owned by the account
/// `12345678-90ab-cdef-1234-567890abcdef`:
///
/// ```json
/// {
///   "filter": {
///     "any": [
///       {
///         "equal": [
///           { "path": ["updatedById"] },
///           { "parameter": "12345678-90ab-cdef-1234-567890abcdef" }
///         ]
///       },
///       {
///         "equal": [
///           { "path": ["ownedById"] },
///           { "parameter": "12345678-90ab-cdef-1234-567890abcdef" }
///         ]
///       }
///     ]
///   },
///   "graphResolveDepths": {
///     "inheritsFrom": {
///       "outgoing": 0
///     },
///     "constrainsValuesOn": {
///       "outgoing": 0
///     },
///     "constrainsPropertiesOn": {
///       "outgoing": 0
///     },
///     "constrainsLinksOn": {
///       "outgoing": 0
///     },
///     "constrainsLinkDestinationsOn": {
///       "outgoing": 0
///     },
///     "isOfType": {
///       "outgoing": 0
///     },
///     "hasLeftEntity": {
///       "incoming": 2,
///       "outgoing": 2
///     },
///     "hasRightEntity": {
///       "incoming": 2,
///       "outgoing": 2
///     }
///   }
/// }
/// ```
///
/// [`RecordPath`]: crate::store::query::QueryPath
/// [`Parameter`]: crate::store::query::Parameter
#[derive(Deserialize, ToSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[aliases(
    DataTypeStructuralQuery = StructuralQuery<'static, DataTypeWithMetadata>,
    PropertyTypeStructuralQuery = StructuralQuery<'static, PropertyTypeWithMetadata>,
    EntityTypeStructuralQuery = StructuralQuery<'static, EntityTypeWithMetadata>,
    EntityStructuralQuery = StructuralQuery<'static, Entity>,
)]
pub struct StructuralQuery<'p, R: Record> {
    #[serde(bound = "'de: 'p, R::QueryPath<'p>: Deserialize<'de>")]
    pub filter: Filter<'p, R>,
    pub graph_resolve_depths: GraphResolveDepths,
    #[serde(default)]
    pub time_projection: TimeProjection,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Projection<K, I> {
    kernel: Kernel<K>,
    image: Image<I>,
}

impl<K: Copy, I: Copy> TimeResolver for Projection<K, I> {
    fn resolve(&self, now: DateTime<Utc>) -> Self {
        Self {
            kernel: self.kernel.resolve(now),
            image: self.image.resolve(now),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
struct Kernel<A> {
    #[serde(rename = "axis")]
    tag: A,
    timestamp: Option<Timestamp<A>>,
}

impl<A: Copy> TimeResolver for Kernel<A> {
    fn resolve(&self, now: DateTime<Utc>) -> Self {
        Self {
            tag: self.tag,
            timestamp: Some(self.timestamp.unwrap_or(Timestamp::from(now))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
struct Image<A> {
    #[serde(rename = "axis")]
    tag: A,
    #[serde(flatten)]
    timespan: Timespan<A>,
}

impl<A: Copy> TimeResolver for Image<A> {
    fn resolve(&self, now: DateTime<Utc>) -> Self {
        Self {
            tag: self.tag,
            timespan: self.timespan.resolve(now),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct TaggedAxis<T, A> {
    #[serde(rename = "axis")]
    tag: T,
    #[serde(flatten)]
    axis: A,
}

pub type DecisionTimeProjection = Projection<TransactionTime, DecisionTime>;
pub type TransactionTimeProjection = Projection<DecisionTime, TransactionTime>;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[serde(untagged)]
pub enum TimeProjection {
    DecisionTime(DecisionTimeProjection),
    TransactionTime(TransactionTimeProjection),
}

impl Default for TimeProjection {
    fn default() -> Self {
        Self::DecisionTime(DecisionTimeProjection {
            kernel: Kernel {
                tag: TransactionTime::Transaction,
                timestamp: None,
            },
            image: Image {
                tag: DecisionTime::Decision,
                timespan: DecisionTimespan::new(..),
            },
        })
    }
}

pub trait TimeResolver {
    fn resolve(&self, now: DateTime<Utc>) -> Self;
}

impl TimeResolver for TimeProjection {
    fn resolve(&self, now: DateTime<Utc>) -> Self {
        match self {
            Self::DecisionTime(projection) => Self::DecisionTime(projection.resolve(now)),
            Self::TransactionTime(projection) => Self::TransactionTime(projection.resolve(now)),
        }
    }
}

// TODO: Derive traits when bounds are generated correctly
//   see https://github.com/rust-lang/rust/issues/26925
impl<'p, R> Debug for StructuralQuery<'p, R>
where
    R: Record<QueryPath<'p>: Debug>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StructuralQuery")
            .field("filter", &self.filter)
            .field("graph_resolve_depths", &self.graph_resolve_depths)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::identifier::time::{
        DecisionTime, DecisionTimespan, DecisionTimestamp, TransactionTime, TransactionTimestamp,
    };

    #[test]
    fn test_time_project_serialization() {
        let timestamp_a =
            DecisionTimestamp::from_str("2020-01-01T00:00:00Z").expect("invalid timestamp");
        let timestamp_b =
            DecisionTimestamp::from_str("2020-01-02T00:00:00Z").expect("invalid timestamp");
        let timestamp_c =
            TransactionTimestamp::from_str("2020-01-03T00:00:00Z").expect("invalid timestamp");

        let projection = TimeProjection::DecisionTime(DecisionTimeProjection {
            kernel: Kernel {
                tag: TransactionTime::Transaction,
                timestamp: Some(timestamp_c),
            },
            image: Image {
                tag: DecisionTime::Decision,
                timespan: DecisionTimespan::new(timestamp_a..),
            },
        });

        println!("{}", serde_json::to_string_pretty(&projection).unwrap());
    }
}
