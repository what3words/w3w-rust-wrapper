use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Error {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ErrorResult {
    pub error: Error,
}
