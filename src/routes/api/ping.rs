use axum::http::StatusCode;

/// Returns server status
///
/// Cheap api call to check if server is online
#[utoipa::path(
    get,
    path = "/ping",
    tag = "Utility",
    responses(
        (status = OK, description = "Server is online", body = ())
    )
)]
pub async fn ping() -> StatusCode {
    StatusCode::OK
}
