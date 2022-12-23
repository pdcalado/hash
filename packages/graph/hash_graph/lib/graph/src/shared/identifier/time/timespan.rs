use derivative::Derivative;
use serde::{Deserialize, Serialize};
use utoipa::{openapi, ToSchema};

use crate::identifier::time::Timestamp;

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(
    Debug(bound = ""),
    Clone(bound = ""),
    PartialEq(bound = ""),
    Eq(bound = ""),
    Hash(bound = "")
)]
#[serde(
    rename_all = "camelCase",
    bound = "",
    tag = "bound",
    content = "timestamp"
)]
pub enum TimespanBound<A> {
    Unbounded,
    Included(Timestamp<A>),
    Excluded(Timestamp<A>),
}

impl<A> TimespanBound<A> {
    #[must_use]
    pub const fn cast<B>(&self) -> TimespanBound<B> {
        match self {
            Self::Unbounded => TimespanBound::Unbounded,
            Self::Included(timestamp) => TimespanBound::Included(timestamp.cast()),
            Self::Excluded(timestamp) => TimespanBound::Excluded(timestamp.cast()),
        }
    }
}

impl<A> ToSchema for TimespanBound<A> {
    fn schema() -> openapi::Schema {
        openapi::OneOfBuilder::new()
            .item(
                openapi::ObjectBuilder::new()
                    .property(
                        "bound",
                        openapi::ObjectBuilder::new().enum_values(Some(["unbounded"])),
                    )
                    .required("bound"),
            )
            .item(
                openapi::ObjectBuilder::new()
                    .property(
                        "bound",
                        openapi::ObjectBuilder::new().enum_values(Some(["included", "excluded"])),
                    )
                    .required("bound")
                    .property("timestamp", Timestamp::<A>::schema())
                    .required("timestamp"),
            )
            .build()
            .into()
    }
}

#[derive(Derivative, Serialize, Deserialize, ToSchema)]
#[derivative(
    Debug(bound = ""),
    Clone(bound = ""),
    PartialEq(bound = ""),
    Eq(bound = ""),
    Hash(bound = "")
)]
#[serde(rename_all = "camelCase", bound = "", deny_unknown_fields)]
pub struct Timespan<A> {
    pub start: Option<TimespanBound<A>>,
    pub end: Option<TimespanBound<A>>,
}

#[derive(Derivative, Serialize, Deserialize, ToSchema)]
#[derivative(
    Debug(bound = ""),
    Clone(bound = ""),
    PartialEq(bound = ""),
    Eq(bound = ""),
    Hash(bound = "")
)]
#[serde(rename_all = "camelCase", bound = "", deny_unknown_fields)]
pub struct ResolvedTimespan<A> {
    pub start: TimespanBound<A>,
    pub end: TimespanBound<A>,
}

impl<A> ResolvedTimespan<A> {
    #[must_use]
    pub const fn cast<B>(&self) -> ResolvedTimespan<B> {
        ResolvedTimespan {
            start: self.start.cast(),
            end: self.end.cast(),
        }
    }

    #[must_use]
    pub fn intersect(&self, other: &Self) -> Self {
        Self {
            start: match (&self.start, &other.start) {
                (TimespanBound::Unbounded, _) => other.start.clone(),
                (_, TimespanBound::Unbounded) => self.start.clone(),
                (TimespanBound::Included(lhs), TimespanBound::Included(rhs)) => {
                    TimespanBound::Included(Timestamp::max(*lhs, *rhs))
                }
                (TimespanBound::Excluded(lhs), TimespanBound::Excluded(rhs)) => {
                    TimespanBound::Excluded(Timestamp::max(*lhs, *rhs))
                }
                (TimespanBound::Included(lhs), TimespanBound::Excluded(rhs)) => {
                    if lhs >= rhs {
                        TimespanBound::Excluded(*rhs)
                    } else {
                        TimespanBound::Included(*lhs)
                    }
                }
                (TimespanBound::Excluded(lhs), TimespanBound::Included(rhs)) => {
                    if lhs > rhs {
                        TimespanBound::Excluded(*rhs)
                    } else {
                        TimespanBound::Included(*lhs)
                    }
                }
            },
            end: match (&self.end, &other.end) {
                (TimespanBound::Unbounded, _) => other.end.clone(),
                (_, TimespanBound::Unbounded) => self.end.clone(),
                (TimespanBound::Included(lhs), TimespanBound::Included(rhs)) => {
                    TimespanBound::Included(Timestamp::min(*lhs, *rhs))
                }
                (TimespanBound::Excluded(lhs), TimespanBound::Excluded(rhs)) => {
                    TimespanBound::Excluded(Timestamp::min(*lhs, *rhs))
                }
                (TimespanBound::Included(lhs), TimespanBound::Excluded(rhs)) => {
                    if lhs < rhs {
                        TimespanBound::Included(*lhs)
                    } else {
                        TimespanBound::Excluded(*rhs)
                    }
                }
                (TimespanBound::Excluded(lhs), TimespanBound::Included(rhs)) => {
                    if lhs <= rhs {
                        TimespanBound::Included(*lhs)
                    } else {
                        TimespanBound::Excluded(*rhs)
                    }
                }
            },
        }
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        Self {
            start: match (&self.start, &other.start) {
                (TimespanBound::Unbounded, _) | (_, TimespanBound::Unbounded) => {
                    TimespanBound::Unbounded
                }
                (TimespanBound::Included(lhs), TimespanBound::Included(rhs)) => {
                    TimespanBound::Included(Timestamp::min(*lhs, *rhs))
                }
                (TimespanBound::Excluded(lhs), TimespanBound::Excluded(rhs)) => {
                    TimespanBound::Excluded(Timestamp::min(*lhs, *rhs))
                }
                (TimespanBound::Included(lhs), TimespanBound::Excluded(rhs)) => {
                    if lhs <= rhs {
                        TimespanBound::Included(*lhs)
                    } else {
                        TimespanBound::Excluded(*rhs)
                    }
                }
                (TimespanBound::Excluded(lhs), TimespanBound::Included(rhs)) => {
                    if lhs < rhs {
                        TimespanBound::Excluded(*lhs)
                    } else {
                        TimespanBound::Included(*rhs)
                    }
                }
            },
            end: match (&self.end, &other.end) {
                (TimespanBound::Unbounded, _) | (_, TimespanBound::Unbounded) => {
                    TimespanBound::Unbounded
                }
                (TimespanBound::Included(lhs), TimespanBound::Included(rhs)) => {
                    TimespanBound::Included(Timestamp::max(*lhs, *rhs))
                }
                (TimespanBound::Excluded(lhs), TimespanBound::Excluded(rhs)) => {
                    TimespanBound::Excluded(Timestamp::max(*lhs, *rhs))
                }
                (TimespanBound::Included(lhs), TimespanBound::Excluded(rhs)) => {
                    if lhs >= rhs {
                        TimespanBound::Included(*lhs)
                    } else {
                        TimespanBound::Excluded(*rhs)
                    }
                }
                (TimespanBound::Excluded(lhs), TimespanBound::Included(rhs)) => {
                    if lhs > rhs {
                        TimespanBound::Excluded(*lhs)
                    } else {
                        TimespanBound::Included(*rhs)
                    }
                }
            },
        }
    }
}
