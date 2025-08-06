mod channel_avater;
mod channels;
mod ping;
mod statistics;
mod video_thumbnail;
mod videos;
mod watch_history;

use crate::state::AppState;
use utoipa_axum::{router::OpenApiRouter, routes};

use statistics::statistics_routes;

pub fn api_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(ping::ping))
        .routes(routes!(channel_avater::get_channel_avater))
        .routes(routes!(video_thumbnail::get_video_thumbnail))
        .routes(routes!(
            watch_history::get_watch_history,
            watch_history::create_watch_history
        ))
        .routes(routes!(videos::get_videos))
        .routes(routes!(channels::get_channels))
        .nest("/statistics", statistics_routes())
}
