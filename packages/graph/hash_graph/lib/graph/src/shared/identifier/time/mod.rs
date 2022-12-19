mod timespan;
mod timestamp;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub use self::{timespan::Timespan, timestamp::Timestamp};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum DecisionTime {
    Decision,
}

pub type DecisionTimestamp = Timestamp<DecisionTime>;
pub type DecisionTimespan = Timespan<DecisionTime>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum TransactionTime {
    Transaction,
}
pub type TransactionTimestamp = Timestamp<TransactionTime>;
pub type TransactionTimespan = Timespan<TransactionTime>;
