mod timespan;
mod timestamp;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub use self::{
    timespan::{BoundedTimespan, Timespan, TimespanBound},
    timestamp::Timestamp,
};

/// Time axis for the decision time.
///
/// This is used as the generic argument to time-related structs and can be used as tag value.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum DecisionTime {
    #[default]
    Decision,
}

pub type DecisionTimestamp = Timestamp<DecisionTime>;
pub type DecisionTimespanBound = TimespanBound<DecisionTime>;
pub type DecisionTimespan = Timespan<DecisionTime>;
pub type BoundedDecisionTimespan = BoundedTimespan<DecisionTime>;

/// Time axis for the transaction time.
///
/// This is used as the generic argument to time-related structs and can be used as tag value.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum TransactionTime {
    #[default]
    Transaction,
}

pub type TransactionTimestamp = Timestamp<TransactionTime>;
pub type TransactionTimespanBound = TimespanBound<TransactionTime>;
pub type TransactionTimespan = Timespan<TransactionTime>;
pub type BoundedTransactionTimespan = BoundedTimespan<TransactionTime>;
