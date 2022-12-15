use serde::{Deserialize, Serialize};

use crate::error_code::ErrorCode;

pub mod error_code;

/// Defines a logical error model that is suitable for different programming environments, including
/// REST APIs and RPC APIs.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Status<D: Serialize> {
    code: ErrorCode,
    message: String,
    details: D,
}

impl<D> Status<D>
where
    D: Serialize + for<'de> Deserialize<'de>,
{
    pub fn new(code: ErrorCode, message: String, details: D) -> Self {
        Self {
            code,
            message,
            details,
        }
    }

    pub fn code(&self) -> ErrorCode {
        self.code
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn details(&self) -> &D {
        &self.details
    }
}
