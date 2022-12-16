use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::status_code::StatusCode;

pub mod payload;
pub mod status_code;

/// Defines a logical status and error model that is suitable for different programming
/// environments, including REST APIs and RPC APIs.
#[derive(Clone, Eq, Debug, PartialEq, Serialize, Deserialize)]
pub struct Status<D>
where
    D: Send + Sync + Debug,
{
    code: StatusCode,
    /// A developer-facing description of the status.
    ///
    /// Where possible, this should provide guiding advice for debugging and/or handling the error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    contents: D,
}

impl<D> Status<D>
where
    D: Send + Sync + Debug + Serialize + for<'de> Deserialize<'de>,
{
    pub fn new(code: StatusCode, message: Option<String>, contents: D) -> Self {
        Self {
            code,
            message,
            contents,
        }
    }

    pub fn code(&self) -> StatusCode {
        self.code
    }

    pub fn message(&self) -> &Option<String> {
        &self.message
    }

    pub fn contents(&self) -> &D {
        &self.contents
    }
}

// impl<D> Display for Status<D>
// where
//     D: Send + Sync + Debug + Serialize + for<'de> Deserialize<'de>,
// {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }
//
// impl<D> Context for Status<D> where D: Send + Sync + Debug + Serialize + for<'de>
// Deserialize<'de> {}
