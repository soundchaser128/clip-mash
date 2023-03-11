use std::io;

use axum::{
    response::{IntoResponse, Response},
    Json,
};
use reqwest::StatusCode;
use serde_json::json;

type StdError = Box<dyn std::error::Error>;

#[derive(Debug)]
pub enum AppError {
    Generic(StdError),
    Io(io::Error),
}

impl From<StdError> for AppError {
    fn from(value: StdError) -> Self {
        AppError::Generic(value)
    }
}

impl From<io::Error> for AppError {
    fn from(value: io::Error) -> Self {
        AppError::Io(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("request failed: {:?}", self);
        let error_message = match self {
            AppError::Generic(e) => e.to_string(),
            AppError::Io(e) => format!("io error: {e}"),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
