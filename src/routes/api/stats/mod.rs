use crate::state::AppState;
use axum::{Router, routing::get};

mod overview;
use overview::get_overview;

pub fn stats_routes() -> Router<AppState> {
    Router::new().route("/overview", get(get_overview))
}
