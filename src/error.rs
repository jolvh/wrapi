use std::fmt;

use http::StatusCode;
use serde_json::Value;

#[derive(Clone, Debug)]
pub enum Error {
    /// API response with possible body
    ResponseError((StatusCode, Option<Value>)),
    /// Generic HTTP client error
    ClientError,
    /// HTTP client failed to decode/deserialize response
    ClientDecodeError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ResponseError((status, body)) => {
                write!(
                    f,
                    "API response error with status {} and body {:?}",
                    status, body
                )
            }
            Error::ClientError => write!(f, "HTTP client error"),
            Error::ClientDecodeError => write!(f, "HTTP client failed to decode response"),
        }
    }
}
