use crate::api_prelude::*;

/// Returns video thumbnail
#[utoipa::path(
    get,
    path = "/thumbnails/{video_id}",
    tag = "Video",
    responses(
        (status = OK, description = "Image was found on disk", content_type = "image/webp", body = Vec<u8>),
        (status = NOT_FOUND, description = "Image not found on disk"),
    )
)]
pub async fn get_video_thumbnail(
    State(state): State<AppState>,
    Path(video_id): Path<String>,
) -> impl IntoResponse {
    let thumbnail_file_path = state
        .video_thumbnails_dir
        .join(utils::build_thumbnail_cache_image_filename(&video_id));

    let Ok(file) = tokio::fs::File::open(&thumbnail_file_path).await else {
        return (StatusCode::NOT_FOUND).into_response();
    };

    let Some(content_type) = mime_guess::from_path(&thumbnail_file_path).first_raw() else {
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
