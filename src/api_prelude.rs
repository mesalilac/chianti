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
use diesel::prelude::*;
pub use serde::{Deserialize, Serialize};
pub use ts_rs::TS;
pub use utils::internal_error;

use diesel::sql_types::{BigInt, Text};

define_sql_function! {
    #[sql_name = "strftime"]
    fn strftime(fmt: Text, ts: BigInt, modifier: Text) -> Text
}

pub type ApiErr = (StatusCode, String);
pub type ApiResult<T> = Result<T, ApiErr>;

pub use crate::apply_sort;
pub use crate::day_unix;
pub use crate::month_unix;
pub use crate::year_unix;

#[derive(Debug, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(utoipa::ToSchema, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub total: i64,
}

#[derive(utoipa::ToSchema, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ChannelResponse {
    #[serde(flatten)]
    pub channel: models::Channel,
    pub avatar_endpoint: String,
}

#[derive(utoipa::ToSchema, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct VideoResponse {
    #[serde(flatten)]
    pub video: models::Video,
    pub thumbnail_endpoint: String,
    pub tags: Vec<String>,
    pub channel: Option<ChannelResponse>,
}

#[derive(utoipa::ToSchema, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ChannelWithVideosResponse {
    #[serde(flatten)]
    pub channel: ChannelResponse,
    pub videos: Vec<VideoResponse>,
}

#[derive(utoipa::ToSchema, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct WatchHistoryResponse {
    #[serde(flatten)]
    pub watch_history: models::WatchHistory,
    pub video: VideoResponse,
}
