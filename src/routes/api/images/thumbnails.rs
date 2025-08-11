use crate::api_prelude::*;

/// Returns video thumbnail
#[utoipa::path(
    get,
    path = "/thumbnails/{id}",
    tag = "Images",
    responses(
        (status = OK, description = "Image was found on disk", content_type = "image/webp", body = Vec<u8>),
        (status = NOT_FOUND, description = "Image not found on disk"),
    )
)]
pub async fn get_video_thumbnail(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<impl IntoResponse> {
    let thumbnail_file_path = state
        .video_thumbnails_dir
        .join(utils::build_thumbnail_cache_image_filename(&id));

    let Ok(file) = tokio::fs::File::open(&thumbnail_file_path).await else {
        return Err((StatusCode::NOT_FOUND, "Image not found on disk".to_string()));
    };

    let content_type = mime_guess::from_path(&thumbnail_file_path)
        .first_raw()
        .unwrap_or("application/octet-stream");

    let stream = tokio_util::io::ReaderStream::new(file);
    let body = Body::from_stream(stream);

    Response::builder()
        .header("Content-Type", content_type)
        .body(body)
        .map_err(internal_error)
}
