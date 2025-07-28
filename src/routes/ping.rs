use axum::http::StatusCode;

/// check if the server is online
pub async fn ping() -> (StatusCode, String) {
    (StatusCode::OK, "pong".to_string())
}
