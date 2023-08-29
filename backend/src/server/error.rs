use std::collections::HashMap;
use std::{fmt, io};

use axum::response::{IntoResponse, Response};
use axum::Json;
use reqwest::StatusCode;
use serde_json::json;
use tracing::error;

#[derive(Debug)]
pub enum AppError {
    Io(io::Error),
    Report(color_eyre::Report),
    StatusCode(StatusCode),
    Validation(HashMap<&'static str, &'static str>),
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

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        error!("request failed: {:?}", self);
        let status_code = match &self {
            AppError::StatusCode(code) => *code,
            AppError::Validation(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let error_message = match self {
            AppError::Io(e) => json!(format!("io error: {e}")),
            AppError::Report(e) => json!(e.to_string()),
            AppError::StatusCode(s) => json!(s.to_string()),
            AppError::Validation(map) => json!(map),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status_code, body).into_response()
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "io error: {}", e),
            AppError::Report(e) => write!(f, "{}", e),
            AppError::StatusCode(s) => write!(f, "status code: {}", s),
            AppError::Validation(map) => write!(f, "validation error: {:?}", map),
        }
    }
}

impl std::error::Error for AppError {}
