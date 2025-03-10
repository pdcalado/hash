use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::identifier::{
    knowledge::EntityId,
    time::{ProjectedTime, Timestamp},
};

pub mod subgraph;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EntityIdAndTimestamp {
    pub base_id: EntityId,
    pub timestamp: Timestamp<ProjectedTime>,
}
