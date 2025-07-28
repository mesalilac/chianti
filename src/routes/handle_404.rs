use axum::http::StatusCode;

pub async fn handle_404() -> (StatusCode, String) {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found".to_string(),
    )
}
