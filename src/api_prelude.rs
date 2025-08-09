/// Prelude for the api routes
pub use crate::database::models;
pub use crate::schema;
pub use crate::state::AppState;
pub use crate::utils;
pub use axum::{
    Json,
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
pub use axum_extra::extract::Query;
pub use serde::{Deserialize, Serialize};
pub use ts_rs::TS;
pub use utils::internal_error;

pub type ApiErr = (StatusCode, String);
pub type ApiResult<T> = Result<T, ApiErr>;

#[derive(utoipa::ToSchema, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct VideoResponse {
    #[serde(flatten)]
    pub video: models::Video,
    pub thumbnail_endpoint: String,
    pub tags: Vec<String>,
    pub channel: Option<models::Channel>,
}

#[derive(utoipa::ToSchema, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ChannelResponse {
    #[serde(flatten)]
    pub channel: models::Channel,
    pub videos: Option<Vec<VideoResponse>>,
}

#[derive(utoipa::ToSchema, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct WatchHistoryResponse {
    #[serde(flatten)]
    pub watch_history: models::WatchHistory,
    pub video: models::Video,
    pub channel: models::Channel,
}
