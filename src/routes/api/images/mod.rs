use crate::state::AppState;
use utoipa_axum::{router::OpenApiRouter, routes};

mod avaters;
mod thumbnails;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(avaters::get_channel_avater))
        .routes(routes!(thumbnails::get_video_thumbnail))
}
