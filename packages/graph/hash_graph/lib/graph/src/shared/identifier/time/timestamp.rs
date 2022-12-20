use core::fmt;
use std::{any::type_name, error::Error, marker::PhantomData, str::FromStr};

use chrono::{DateTime, Utc};
use derivative::Derivative;
use postgres_types::{private::BytesMut, FromSql, ToSql, Type};
use serde::{Deserialize, Serialize};
use utoipa::{openapi, ToSchema};

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(
    Copy(bound = ""),
    Clone(bound = ""),
    PartialEq(bound = ""),
    Eq(bound = ""),
    Hash(bound = ""),
    PartialOrd(bound = ""),
    Ord(bound = "")
)]
#[serde(transparent, bound = "")]
pub struct Timestamp<A> {
    #[serde(skip)]
    axis: PhantomData<A>,
    date_time: DateTime<Utc>,
}

impl<A> fmt::Debug for Timestamp<A> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("timestamp")
            .field("axis", &type_name::<A>())
            .field("date_time", &self.date_time)
            .finish()
    }
}

impl<A> Timestamp<A> {
    #[must_use]
    pub fn from_date_time(timestamp: Timestamp<DateTime<Utc>>) -> Self {
        Self::from(timestamp.date_time)
    }

    #[must_use]
    pub fn to_date_time(self) -> Timestamp<DateTime<Utc>> {
        Timestamp::from(self.date_time)
    }
}

impl<A> From<DateTime<Utc>> for Timestamp<A> {
    fn from(timestamp: DateTime<Utc>) -> Self {
        Self {
            axis: PhantomData,
            date_time: timestamp,
        }
    }
}

impl<A> From<Timestamp<A>> for DateTime<Utc> {
    fn from(timestamp: Timestamp<A>) -> Self {
        timestamp.date_time
    }
}

impl<A> FromStr for Timestamp<A> {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(DateTime::from_str(s)?))
    }
}

impl<'a> FromSql<'a> for Timestamp<DateTime<Utc>> {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        Ok(Self::from(DateTime::from_sql(ty, raw)?))
    }

    fn accepts(ty: &Type) -> bool {
        <DateTime<Utc> as FromSql>::accepts(ty)
    }
}

impl<A> ToSql for Timestamp<A> {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn Error + Sync + Send>> {
        self.date_time.to_sql(ty, out)
    }

    fn accepts(ty: &Type) -> bool {
        <DateTime<Utc> as ToSql>::accepts(ty)
    }

    fn to_sql_checked(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn Error + Sync + Send>> {
        self.date_time.to_sql_checked(ty, out)
    }
}

// Utoipa is not able to generate a schema for generic structs, this has to be kept in sync with
// the definition of `DecisionTimeProjection`.
impl<A> ToSchema for Timestamp<A> {
    fn schema() -> openapi::Schema {
        openapi::schema::ObjectBuilder::new()
            .schema_type(openapi::SchemaType::String)
            .format(Some(openapi::SchemaFormat::KnownFormat(
                openapi::KnownFormat::DateTime,
            )))
            .into()
    }
}
