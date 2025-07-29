mod channel_avater;
mod handle_404;
mod ping;
mod root;
mod video_thumbnail;
mod watch_history;

use crate::state::AppState;
use axum::{
    Router,
    routing::{get, post},
};

pub use channel_avater::get_channel_avater;
pub use handle_404::handle_404;
pub use ping::ping;
pub use root::root;
pub use video_thumbnail::get_video_thumbnail;
pub use watch_history::create_watch_history;

pub fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/avater/{channel_id}", get(get_channel_avater))
        .route("/thumbnail/{video_id}", get(get_video_thumbnail))
        .route("/ping", get(ping))
        .route("/watch_history", post(create_watch_history))
}
