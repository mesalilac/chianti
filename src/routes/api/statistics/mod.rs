use crate::state::AppState;
use utoipa_axum::{router::OpenApiRouter, routes};

mod overview;

pub fn statistics_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(overview::get_overview))
}
