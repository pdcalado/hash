use std::ops::{Bound, RangeBounds};

use chrono::{DateTime, Utc};
use derivative::Derivative;
use postgres_protocol::types::{time_from_sql, Range, RangeBound};
use postgres_types::{FromSql, Type};
use serde::{Deserialize, Serialize};
use utoipa::{openapi, openapi::ObjectBuilder, ToSchema};

use crate::identifier::time::Timestamp;

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(
    Debug(bound = ""),
    Clone(bound = ""),
    Copy(bound = ""),
    PartialEq(bound = ""),
    Eq(bound = ""),
    Hash(bound = "")
)]
#[serde(rename_all = "camelCase", tag = "bound", content = "timestamp")]
pub enum TimespanBound<T> {
    Unbound,
    Included(Timestamp<T>),
    Excluded(Timestamp<T>),
}

impl<A> ToSchema for TimespanBound<A> {
    fn schema() -> openapi::Schema {
        openapi::OneOfBuilder::new()
            .item(
                ObjectBuilder::new()
                    .property("bound", ObjectBuilder::new().enum_values(Some(["unbound"])))
                    .required("bound"),
            )
            .item(
                ObjectBuilder::new()
                    .property(
                        "bound",
                        ObjectBuilder::new().enum_values(Some(["included", "excluded"])),
                    )
                    .required("bound")
                    .property("timestamp", Timestamp::<A>::schema())
                    .required("timestamp"),
            )
            .build()
            .into()
    }
}

impl<T> TimespanBound<T> {
    #[must_use]
    #[expect(dead_code, reason = "not used yet")]
    fn from_date_time(bound: TimespanBound<DateTime<Utc>>) -> Self {
        match bound {
            TimespanBound::Unbound => Self::Unbound,
            TimespanBound::Included(t) => Self::Included(Timestamp::from_date_time(t)),
            TimespanBound::Excluded(t) => Self::Excluded(Timestamp::from_date_time(t)),
        }
    }

    #[must_use]
    #[expect(dead_code, reason = "not used yet")]
    fn to_date_time(self) -> TimespanBound<DateTime<Utc>> {
        match self {
            Self::Unbound => TimespanBound::Unbound,
            Self::Included(t) => TimespanBound::Included(Timestamp::to_date_time(t)),
            Self::Excluded(t) => TimespanBound::Excluded(Timestamp::to_date_time(t)),
        }
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
pub struct Timespan<T> {
    from: TimespanBound<T>,
    to: TimespanBound<T>,
}

impl<T> Timespan<T> {
    #[must_use]
    #[expect(dead_code, reason = "not used yet")]
    pub(crate) fn from_date_time(timespan: Timespan<DateTime<Utc>>) -> Self {
        Self {
            from: TimespanBound::from_date_time(timespan.from),
            to: TimespanBound::from_date_time(timespan.to),
        }
    }

    #[must_use]
    #[expect(dead_code, reason = "not used yet")]
    pub(crate) fn to_date_time(&self) -> Timespan<DateTime<Utc>> {
        Timespan {
            from: self.from.to_date_time(),
            to: self.to.to_date_time(),
        }
    }
}

impl<T> Timespan<T> {
    #[must_use]
    pub fn new(timespan: impl RangeBounds<Timestamp<T>>) -> Self {
        Self {
            from: match timespan.start_bound().cloned() {
                Bound::Included(t) => TimespanBound::Included(Timestamp::from(DateTime::from(t))),
                Bound::Excluded(t) => TimespanBound::Excluded(Timestamp::from(DateTime::from(t))),
                Bound::Unbounded => TimespanBound::Unbound,
            },
            to: match timespan.end_bound().cloned() {
                Bound::Included(t) => TimespanBound::Included(Timestamp::from(DateTime::from(t))),
                Bound::Excluded(t) => TimespanBound::Excluded(Timestamp::from(DateTime::from(t))),
                Bound::Unbounded => TimespanBound::Unbound,
            },
        }
    }
}

impl<T> RangeBounds<Timestamp<T>> for Timespan<T> {
    fn start_bound(&self) -> Bound<&Timestamp<T>> {
        match &self.from {
            TimespanBound::Included(t) => Bound::Included(t),
            TimespanBound::Excluded(t) => Bound::Excluded(t),
            TimespanBound::Unbound => Bound::Unbounded,
        }
    }

    fn end_bound(&self) -> Bound<&Timestamp<T>> {
        match &self.to {
            TimespanBound::Included(t) => Bound::Included(t),
            TimespanBound::Excluded(t) => Bound::Excluded(t),
            TimespanBound::Unbound => Bound::Unbounded,
        }
    }
}
fn parse_bound(
    bound: &RangeBound<Option<&[u8]>>,
) -> Result<TimespanBound<DateTime<Utc>>, Box<dyn std::error::Error + Send + Sync>> {
    Ok(match bound {
        RangeBound::Inclusive(None) | RangeBound::Exclusive(None) => {
            unimplemented!("null ranges are not supported")
        }
        RangeBound::Inclusive(Some(bytes)) => {
            let timestamp = DateTime::from_sql(&Type::TIMESTAMPTZ, bytes)?;
            TimespanBound::Included(Timestamp::from(timestamp))
        }
        RangeBound::Exclusive(Some(bytes)) => {
            let timestamp = DateTime::from_sql(&Type::TIMESTAMPTZ, bytes)?;
            TimespanBound::Excluded(Timestamp::from(timestamp))
        }
        RangeBound::Unbounded => TimespanBound::Unbound,
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

impl FromSql<'_> for Timespan<DateTime<Utc>> {
    fn from_sql(_: &Type, buf: &[u8]) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        match postgres_protocol::types::range_from_sql(buf)? {
            Range::Empty => unimplemented!("Empty ranges are not supported"),
            Range::Nonempty(lower, upper) => Ok(Self {
                from: if is_negative_infinity(&lower)? {
                    TimespanBound::Unbound
                } else {
                    parse_bound(&lower)?
                },
                to: if is_positive_infinity(&upper)? {
                    TimespanBound::Unbound
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

#[derive(Derivative, Serialize, Deserialize, ToSchema)]
#[derivative(
    Debug(bound = ""),
    Copy(bound = ""),
    Clone(bound = ""),
    PartialEq(bound = ""),
    Eq(bound = ""),
    Hash(bound = "")
)]
pub struct BoundedTimespan<T> {
    pub from: Timestamp<T>,
    pub to: Option<Timestamp<T>>,
}

impl<T> BoundedTimespan<T> {
    #[must_use]
    pub const fn new(from: Timestamp<T>, to: Option<Timestamp<T>>) -> Self {
        Self { from, to }
    }

    #[must_use]
    pub(crate) fn from_date_time(timespan: BoundedTimespan<DateTime<Utc>>) -> Self {
        Self {
            from: Timestamp::from_date_time(timespan.from),
            to: timespan.to.map(Timestamp::from_date_time),
        }
    }

    #[must_use]
    #[expect(dead_code, reason = "not used yet")]
    pub(crate) fn to_date_time(self) -> BoundedTimespan<DateTime<Utc>> {
        BoundedTimespan {
            from: self.from.to_date_time(),
            to: self.to.map(Timestamp::to_date_time),
        }
    }
}

impl FromSql<'_> for BoundedTimespan<DateTime<Utc>> {
    fn from_sql(_: &Type, buf: &[u8]) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        match postgres_protocol::types::range_from_sql(buf)? {
            Range::Empty => unimplemented!("Empty ranges are not supported"),
            Range::Nonempty(lower, upper) => Ok(Self {
                from: if is_negative_infinity(&lower)? {
                    unreachable!("Unbounded lower bounds are not supported");
                } else {
                    match parse_bound(&lower)? {
                        TimespanBound::Included(t) => t,
                        TimespanBound::Excluded(_) => {
                            unimplemented!("Excluded lower bounds are not supported")
                        }
                        TimespanBound::Unbound => {
                            unreachable!("Unbounded lower bounds are not supported")
                        }
                    }
                },
                to: if is_positive_infinity(&upper)? {
                    None
                } else {
                    match parse_bound(&upper)? {
                        TimespanBound::Included(_) => {
                            unimplemented!("Included upper bounds are not supported")
                        }
                        TimespanBound::Excluded(t) => Some(t),
                        TimespanBound::Unbound => None,
                    }
                },
            }),
        }
    }

    fn accepts(ty: &Type) -> bool {
        matches!(ty, &Type::TSTZ_RANGE)
    }
}
