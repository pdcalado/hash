use std::str::FromStr;

use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
use tokio_postgres::types::ToSql;
use utoipa::{openapi, ToSchema};

use crate::{
    identifier::{
        account::AccountId,
        time::{
            DecisionTimeVersionTimespan, ProjectedTime, TimeAxis, TransactionTimeVersionTimespan,
            VersionTimespan,
        },
    },
    knowledge::EntityUuid,
    provenance::OwnedById,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct EntityId {
    owned_by_id: OwnedById,
    entity_uuid: EntityUuid,
}

impl EntityId {
    #[must_use]
    pub const fn new(owned_by_id: OwnedById, entity_uuid: EntityUuid) -> Self {
        Self {
            owned_by_id,
            entity_uuid,
        }
    }

    #[must_use]
    pub const fn owned_by_id(&self) -> OwnedById {
        self.owned_by_id
    }

    #[must_use]
    pub const fn entity_uuid(&self) -> EntityUuid {
        self.entity_uuid
    }
}

impl Serialize for EntityId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}%{}", self.owned_by_id, self.entity_uuid))
    }
}

impl<'de> Deserialize<'de> for EntityId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // We can be more efficient than this, we know the byte sizes of all the elements
        let as_string = String::deserialize(deserializer)?;
        let mut parts = as_string.split('%');

        Ok(Self {
            owned_by_id: OwnedById::new(AccountId::new(
                uuid::Uuid::from_str(parts.next().ok_or_else(|| {
                    D::Error::custom("failed to find first component of `%` delimited string")
                })?)
                .map_err(|err| D::Error::custom(err.to_string()))?,
            )),
            entity_uuid: EntityUuid::new(
                uuid::Uuid::from_str(parts.next().ok_or_else(|| {
                    D::Error::custom("failed to find second component of `%` delimited string")
                })?)
                .map_err(|err| D::Error::custom(err.to_string()))?,
            ),
        })
    }
}

impl ToSchema for EntityId {
    fn schema() -> openapi::Schema {
        openapi::Schema::Object(openapi::schema::Object::with_type(
            openapi::SchemaType::String,
        ))
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EntityVersion {
    decision_time: DecisionTimeVersionTimespan,
    transaction_time: TransactionTimeVersionTimespan,
}

impl EntityVersion {
    #[must_use]
    pub const fn new(
        decision_time: DecisionTimeVersionTimespan,
        transaction_time: TransactionTimeVersionTimespan,
    ) -> Self {
        Self {
            decision_time,
            transaction_time,
        }
    }

    #[must_use]
    pub const fn decision_time(&self) -> DecisionTimeVersionTimespan {
        self.decision_time
    }

    #[must_use]
    pub const fn transaction_time(&self) -> TransactionTimeVersionTimespan {
        self.transaction_time
    }

    #[must_use]
    pub fn projected_time(&self, axis: TimeAxis) -> VersionTimespan<ProjectedTime> {
        match axis {
            TimeAxis::DecisionTime => self.decision_time().cast(),
            TimeAxis::TransactionTime => self.transaction_time().cast(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, ToSql, ToSchema)]
#[postgres(transparent)]
#[repr(transparent)]
pub struct EntityRecordId(i64);

impl EntityRecordId {
    #[must_use]
    pub const fn new(id: i64) -> Self {
        Self(id)
    }

    #[must_use]
    pub const fn as_i64(&self) -> i64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EntityEditionId {
    base_id: EntityId,
    record_id: EntityRecordId,
}

impl EntityEditionId {
    #[must_use]
    pub const fn new(entity_id: EntityId, record_id: EntityRecordId) -> Self {
        Self {
            base_id: entity_id,
            record_id,
        }
    }

    #[must_use]
    pub const fn base_id(&self) -> EntityId {
        self.base_id
    }

    #[must_use]
    pub const fn record_id(&self) -> EntityRecordId {
        self.record_id
    }
}
