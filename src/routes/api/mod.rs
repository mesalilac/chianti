mod channel_avater;
mod ping;
mod stats;
mod video_thumbnail;
mod watch_history;

use crate::state::AppState;
use axum::{
    Router,
    routing::{get, post},
};

use channel_avater::get_channel_avater;
use ping::ping;
use stats::stats_routes;
use video_thumbnail::get_video_thumbnail;
use watch_history::create_watch_history;

pub fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/avater/{channel_id}", get(get_channel_avater))
        .route("/thumbnail/{video_id}", get(get_video_thumbnail))
        .route("/ping", get(ping))
        .route("/watch_history", post(create_watch_history))
        .nest("/stats", stats_routes())
}
