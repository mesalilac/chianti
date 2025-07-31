mod channel_avater;
mod ping;
mod stats;
mod video_thumbnail;
mod watch_history;

use crate::state::AppState;
use utoipa_axum::{router::OpenApiRouter, routes};

use stats::stats_routes;

pub fn api_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(ping::ping))
        .routes(routes!(channel_avater::get_channel_avater))
        .routes(routes!(video_thumbnail::get_video_thumbnail))
        .routes(routes!(watch_history::create_watch_history))
        .nest("/statistics", stats_routes())
}
