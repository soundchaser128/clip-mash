use std::sync::Arc;
use std::time::Duration;

use axum::extract::{Path, State};
use axum::response::sse::{Event, KeepAlive};
use axum::response::{IntoResponse, Sse};
use axum::Json;
use futures::stream::Stream;

use super::AppState;
use crate::server::error::AppError;

#[axum::debug_handler]
pub async fn get_progress_stream(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Sse<impl Stream<Item = Result<Event, AppError>>> {
    use async_stream::try_stream;

    let stream = try_stream! {
        let state = state.clone();
        while let Some(progress) = state.database.progress.get_progress(id.clone()).await? {
            yield Event::default().json_data(progress).unwrap();
            tokio::time::sleep(Duration::from_millis(250)).await;
        }
    };

    Sse::new(stream).keep_alive(KeepAlive::default())
}

#[utoipa::path(
    post,
    path = "/api/progress/info",
    responses(
        (status = 200, description = "The current progress of video creation, or null if it is finished", body = Progress),
    )
)]
#[axum::debug_handler]
pub async fn get_progress_info(
    Path(id): Path<String>,
    state: State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let progress = state.database.progress.get_progress(&id).await?;
    Ok(Json(progress))
}
