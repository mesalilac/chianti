use crate::state::AppState;
use crate::utils::build_cache_image_filename;
use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub async fn get_channel_avater(
    State(state): State<AppState>,
    Path(channel_id): Path<String>,
) -> impl IntoResponse {
    let avater_file_path = state
        .channel_avaters_dir
        .join(build_cache_image_filename(&channel_id));

    let Ok(file) = tokio::fs::File::open(&avater_file_path).await else {
        return (StatusCode::NOT_FOUND).into_response();
    };

    let Some(content_type) = mime_guess::from_path(&avater_file_path).first_raw() else {
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    };

    let stream = tokio_util::io::ReaderStream::new(file);
    let body = Body::from_stream(stream);

    match Response::builder()
        .header("Content-Type", content_type)
        .body(body)
    {
        Ok(response) => response,
        Err(err) => {
            tracing::error!("Failed to create response: {err}");
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}
