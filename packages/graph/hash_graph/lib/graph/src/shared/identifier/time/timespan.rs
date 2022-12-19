use std::{
    fmt,
    hash::{Hash, Hasher},
    ops::{Bound, RangeBounds},
};

use chrono::{DateTime, Utc};
use postgres_protocol::types::{time_from_sql, Range, RangeBound};
use postgres_types::{FromSql, Type};
use serde::{Deserialize, Serialize};

use crate::{identifier::time::Timestamp, subgraph::query::TimeResolver};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "bound", content = "timestamp")]
enum TimespanBound<T> {
    Included(Timestamp<T>),
    Excluded(Timestamp<T>),
}

impl<T> TimespanBound<T> {
    #[expect(clippy::needless_pass_by_value, reason = "wrong positive")]
    fn new(bound: TimespanBound<DateTime<Utc>>) -> Self {
        match bound {
            TimespanBound::Included(t) => Self::Included(Timestamp::from(DateTime::from(t))),
            TimespanBound::Excluded(t) => Self::Excluded(Timestamp::from(DateTime::from(t))),
        }
    }
}

impl<T> fmt::Debug for TimespanBound<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimespanBound::Included(t) => write!(f, "Included({:?})", t),
            TimespanBound::Excluded(t) => write!(f, "Excluded({:?})", t),
        }
    }
}

impl<T> Clone for TimespanBound<T> {
    fn clone(&self) -> Self {
        match self {
            TimespanBound::Included(t) => Self::Included(*t),
            TimespanBound::Excluded(t) => Self::Excluded(*t),
        }
    }
}

impl<T> Copy for TimespanBound<T> {}

impl<T> Hash for TimespanBound<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            TimespanBound::Included(t) => t.hash(state),
            TimespanBound::Excluded(t) => t.hash(state),
        }
    }
}

impl<T> PartialEq for TimespanBound<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TimespanBound::Included(t1), TimespanBound::Included(t2)) => t1.eq(t2),
            (TimespanBound::Excluded(t1), TimespanBound::Excluded(t2)) => t1.eq(t2),
            _ => false,
        }
    }
}

impl<T> Eq for TimespanBound<T> {}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Timespan<T> {
    from: Option<TimespanBound<T>>,
    to: Option<TimespanBound<T>>,
}

impl<T> Timespan<T> {
    #[must_use]
    pub(crate) fn from_date_time(timespan: Timespan<DateTime<Utc>>) -> Self {
        Self {
            from: timespan.from.map(TimespanBound::new),
            to: timespan.to.map(TimespanBound::new),
        }
    }
}

impl<A> TimeResolver for Timespan<A> {
    fn resolve(&self, now: DateTime<Utc>) -> Self {
        Self {
            from: Some(
                self.from
                    .unwrap_or(TimespanBound::Included(Timestamp::from(now))),
            ),
            to: Some(
                self.to
                    .unwrap_or(TimespanBound::Included(Timestamp::from(now))),
            ),
        }
    }
}

impl<T> Timespan<T> {
    #[must_use]
    pub fn new(timespan: impl RangeBounds<Timestamp<T>>) -> Self {
        Self {
            from: match timespan.start_bound().cloned() {
                Bound::Included(t) => {
                    Some(TimespanBound::Included(Timestamp::from(DateTime::from(t))))
                }
                Bound::Excluded(t) => {
                    Some(TimespanBound::Excluded(Timestamp::from(DateTime::from(t))))
                }
                Bound::Unbounded => None,
            },
            to: match timespan.end_bound().cloned() {
                Bound::Included(t) => {
                    Some(TimespanBound::Included(Timestamp::from(DateTime::from(t))))
                }
                Bound::Excluded(t) => {
                    Some(TimespanBound::Excluded(Timestamp::from(DateTime::from(t))))
                }
                Bound::Unbounded => None,
            },
        }
    }

    // TODO: Remove when exposing temporal versions to backend
    //   see https://app.asana.com/0/0/1203444301722133/f
    #[must_use]
    pub(crate) fn as_start_bound_timestamp(&self) -> Timestamp<T> {
        let Bound::Included(timestamp) = self.start_bound() else { unreachable!("invalid bound") };
        timestamp.clone()
    }
}

impl<T> RangeBounds<Timestamp<T>> for Timespan<T> {
    fn start_bound(&self) -> Bound<&Timestamp<T>> {
        match self.from.as_ref() {
            Some(TimespanBound::Included(t)) => Bound::Included(t),
            Some(TimespanBound::Excluded(t)) => Bound::Excluded(t),
            None => Bound::Unbounded,
        }
    }

    fn end_bound(&self) -> Bound<&Timestamp<T>> {
        match self.to.as_ref() {
            Some(TimespanBound::Included(t)) => Bound::Included(t),
            Some(TimespanBound::Excluded(t)) => Bound::Excluded(t),
            None => Bound::Unbounded,
        }
    }
}

impl FromSql<'_> for Timespan<DateTime<Utc>> {
    fn from_sql(_: &Type, buf: &[u8]) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        fn parse_bound(
            bound: &RangeBound<Option<&[u8]>>,
        ) -> Result<Option<TimespanBound<DateTime<Utc>>>, Box<dyn std::error::Error + Send + Sync>>
        {
            Ok(match bound {
                RangeBound::Inclusive(None) | RangeBound::Exclusive(None) => {
                    unimplemented!("null ranges are not supported")
                }
                RangeBound::Inclusive(Some(bytes)) => {
                    let timestamp = DateTime::from_sql(&Type::TIMESTAMPTZ, bytes)?;
                    Some(TimespanBound::Included(Timestamp::from(timestamp)))
                }
                RangeBound::Exclusive(Some(bytes)) => {
                    let timestamp = DateTime::from_sql(&Type::TIMESTAMPTZ, bytes)?;
                    Some(TimespanBound::Excluded(Timestamp::from(timestamp)))
                }
                RangeBound::Unbounded => None,
            })
        }

        fn is_negative_infinity(
            bound: &RangeBound<Option<&[u8]>>,
        ) -> Result<bool, Box<dyn std::error::Error + Sync + Send>> {
            Ok(match bound {
                RangeBound::Inclusive(Some(bytes)) | RangeBound::Exclusive(Some(bytes)) => {
                    time_from_sql(bytes)? == i64::MIN
                }
                _ => false,
            })
        }

        fn is_positive_infinity(
            bound: &RangeBound<Option<&[u8]>>,
        ) -> Result<bool, Box<dyn std::error::Error + Sync + Send>> {
            Ok(match bound {
                RangeBound::Inclusive(Some(bytes)) | RangeBound::Exclusive(Some(bytes)) => {
                    time_from_sql(bytes)? == i64::MAX
                }
                _ => false,
            })
        }

        match postgres_protocol::types::range_from_sql(buf)? {
            Range::Empty => unimplemented!("Empty ranges are not supported"),
            Range::Nonempty(lower, upper) => Ok(Self {
                from: if is_negative_infinity(&lower)? {
                    None
                } else {
                    parse_bound(&lower)?
                },
                to: if is_positive_infinity(&upper)? {
                    None
                } else {
                    parse_bound(&upper)?
                },
            }),
        }
    }

    fn accepts(ty: &Type) -> bool {
        matches!(ty, &Type::TSTZ_RANGE)
    }
}
