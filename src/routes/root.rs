use axum::response::Redirect;

pub async fn root() -> Redirect {
    Redirect::to("/docs")
}
