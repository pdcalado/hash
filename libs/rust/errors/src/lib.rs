use typeshare::typeshare;

use crate::error_code::ErrorCode;

pub mod error_code;

/// The [`Status`] type defines a logical error model that is suitable for different programming
/// environments, including REST APIs and RPC APIs.
#[typeshare]
pub struct Status<D> {
    code: ErrorCode,
    message: String,
    details: D,
}

impl<D> Status<D> {
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
