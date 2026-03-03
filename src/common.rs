use serde_json::Value;
use thiserror::Error;
use tower_lsp::jsonrpc;

pub type PathServerResult<T> = Result<T, PathServerError>;

#[derive(Debug, Error)]
pub enum PathServerError {
    #[error("Encoding error: {0}")]
    // code 1000
    EncodingError(String), // UTF-8/UTF-16 encoding/decoding error
    #[error("IO error: {0}")]
    // code 1001
    IoError(String),
    #[error("Unknown error: {0}")]
    // code 2000
    Unknown(String),
}

impl From<std::string::FromUtf8Error> for PathServerError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        PathServerError::EncodingError(error.to_string())
    }
}

impl From<std::string::FromUtf16Error> for PathServerError {
    fn from(error: std::string::FromUtf16Error) -> Self {
        PathServerError::EncodingError(error.to_string())
    }
}

impl From<PathServerError> for tower_lsp::jsonrpc::Error {
    fn from(err: PathServerError) -> Self {
        match err {
            PathServerError::EncodingError(msg) => jsonrpc::Error {
                code: jsonrpc::ErrorCode::ServerError(1000),
                message: std::borrow::Cow::Borrowed("Encoding error"),
                data: Some(Value::String(msg)),
            },
            PathServerError::IoError(msg) => jsonrpc::Error {
                code: jsonrpc::ErrorCode::ServerError(1001),
                message: std::borrow::Cow::Borrowed("IO error"),
                data: Some(Value::String(msg)),
            },
            PathServerError::Unknown(msg) => jsonrpc::Error {
                code: jsonrpc::ErrorCode::ServerError(2000),
                message: std::borrow::Cow::Borrowed("Unknown error"),
                data: Some(Value::String(msg)),
            },
        }
    }
}
