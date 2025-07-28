use axum::http::StatusCode;

pub fn cache_image_filename(filename: &String) -> String {
    let base = base32::encode(base32::Alphabet::Crockford, filename.as_bytes());

    format!("{base}.webp")
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    tracing::error!("Unhandled internal error: {}", err);
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
