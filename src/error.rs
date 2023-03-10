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
}

impl From<StdError> for AppError {
    fn from(value: StdError) -> Self {
        AppError::Generic(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("request failed: {:?}", self);
        let error_message = match self {
            AppError::Generic(e) => e.to_string(),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
