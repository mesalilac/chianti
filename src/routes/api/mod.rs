mod channels;
mod images;
mod ping;
mod statistics;
mod tags;
mod videos;
mod watch_history;

use crate::state::AppState;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(ping::ping))
        .routes(routes!(
            watch_history::get_watch_history,
            watch_history::create_watch_history
        ))
        .routes(routes!(videos::get_videos))
        .routes(routes!(videos::get_video))
        .routes(routes!(channels::get_channels))
        .routes(routes!(channels::get_channel))
        .routes(routes!(tags::get_tags))
        .routes(routes!(tags::get_tag))
        .nest("/statistics", statistics::routes())
        .nest("/images", images::routes())
}
