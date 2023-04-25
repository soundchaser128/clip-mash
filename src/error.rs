use std::io;

use axum::{
    response::{IntoResponse, Response},
    Json, extract::multipart::MultipartError,
};
use reqwest::StatusCode;
use serde_json::json;

type StdError = Box<dyn std::error::Error>;

#[derive(Debug)]
pub enum AppError {
    Generic(StdError),
    Io(io::Error),
    Report(color_eyre::Report),
    Multipart(MultipartError),
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

impl From<color_eyre::Report> for AppError {
    fn from(value: color_eyre::Report) -> Self {
        AppError::Report(value)
    }
}

impl From<MultipartError> for AppError {
    fn from(value: MultipartError) -> Self {
        AppError::Multipart(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("request failed: {:?}", self);
        let error_message = match self {
            AppError::Generic(e) => e.to_string(),
            AppError::Io(e) => format!("io error: {e}"),
            AppError::Report(e) => e.to_string(),
            AppError::Multipart(e) => format!("mulitpart error: {e}"),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
