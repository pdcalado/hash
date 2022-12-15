use serde::{Deserialize, Serialize};

use crate::status_code::StatusCode;

pub mod status_code;

/// Defines a logical status and error model that is suitable for different programming
/// environments, including REST APIs and RPC APIs.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Status<D: Serialize> {
    code: StatusCode,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    contents: D,
}

impl<D> Status<D>
where
    D: Serialize + for<'de> Deserialize<'de>,
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
