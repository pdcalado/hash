use crate::error_code::ErrorCode;

impl ErrorCode {
    pub fn to_http_code(&self) -> u16 {
        match self {
            ErrorCode::Ok => 200,
            ErrorCode::Cancelled => 499,
            ErrorCode::Unknown => 500,
            ErrorCode::InvalidArgument => 400,
            ErrorCode::DeadlineExceeded => 504,
            ErrorCode::NotFound => 404,
            ErrorCode::AlreadyExists => 409,
            ErrorCode::PermissionDenied => 403,
            ErrorCode::Unauthenticated => 401,
            ErrorCode::ResourceExhausted => 429,
            ErrorCode::FailedPrecondition => 400,
            ErrorCode::Aborted => 409,
            ErrorCode::OutOfRange => 400,
            ErrorCode::Unimplemented => 501,
            ErrorCode::Internal => 500,
            ErrorCode::Unavailable => 503,
            ErrorCode::DataLoss => 500,
        }
    }
}
