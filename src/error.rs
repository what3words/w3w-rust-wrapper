use std::fmt;

use crate::models::error::ErrorResponse;

#[derive(Debug)]
pub enum Error {
    NetworkError(String),
    HttpError(String),
    ApiError(ErrorResponse),
    DecodeError(String),
    UnknownError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NetworkError(msg) => write!(f, "Network error: {}", msg),
            Error::HttpError(msg) => write!(f, "HTTP error: {}", msg),
            Error::ApiError(res) => write!(
                f,
                "What3words error: {} - {}",
                res.error.code, res.error.message
            ),
            Error::DecodeError(msg) => write!(f, "Decode error: {}", msg),
            Error::UnknownError(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        if error.is_request() {
            Error::HttpError(error.to_string())
        } else if error.is_connect() {
            Error::NetworkError(error.to_string())
        } else if error.is_decode() {
            Error::DecodeError(error.to_string())
        } else {
            Error::UnknownError(error.to_string())
        }
    }
}
