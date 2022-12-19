use std::{
    any::type_name,
    error::Error,
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    str::FromStr,
};

use chrono::{DateTime, Utc};
use postgres_types::{private::BytesMut, FromSql, ToSql, Type};
use serde::{Deserialize, Serialize, Serializer};

pub struct Timestamp<T>(DateTime<Utc>, PhantomData<T>);

impl<T> Timestamp<T> {
    pub fn new(timestamp: Timestamp<DateTime<Utc>>) -> Self {
        Self(timestamp.0, PhantomData)
    }
}

impl<T> From<DateTime<Utc>> for Timestamp<T> {
    fn from(timestamp: DateTime<Utc>) -> Self {
        Self(timestamp, PhantomData)
    }
}

impl<T> From<Timestamp<T>> for DateTime<Utc> {
    fn from(timestamp: Timestamp<T>) -> Self {
        timestamp.0
    }
}

impl<T> fmt::Debug for Timestamp<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}stamp({:?})", type_name::<T>(), self.0)
    }
}

impl<T> Clone for Timestamp<T> {
    fn clone(&self) -> Self {
        Self(self.0, PhantomData)
    }
}

impl<T> Copy for Timestamp<T> {}

impl<T> Hash for Timestamp<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl<T> PartialEq for Timestamp<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T> Eq for Timestamp<T> {}

impl<T> PartialOrd for Timestamp<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T> Ord for Timestamp<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T> Serialize for Timestamp<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for Timestamp<T> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        DateTime::deserialize(deserializer).map(|timestamp| Self(timestamp, PhantomData))
    }
}

impl<T> FromStr for Timestamp<T> {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(DateTime::from_str(s)?, PhantomData))
    }
}

impl<'a> FromSql<'a> for Timestamp<DateTime<Utc>> {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        Ok(Self(DateTime::from_sql(ty, raw)?, PhantomData))
    }

    fn accepts(ty: &Type) -> bool {
        <DateTime<Utc> as FromSql>::accepts(ty)
    }
}

impl<T> ToSql for Timestamp<T> {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn Error + Sync + Send>> {
        self.0.to_sql(ty, out)
    }

    fn accepts(ty: &Type) -> bool {
        <DateTime<Utc> as ToSql>::accepts(ty)
    }

    fn to_sql_checked(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn Error + Sync + Send>> {
        self.0.to_sql_checked(ty, out)
    }
}
