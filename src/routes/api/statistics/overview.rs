use crate::database::models::{Channel, Tag, Video};
use crate::state::AppState;
use crate::utils::internal_error;
use crate::{database::models::WatchHistory, schema};
use axum::{Json, extract::State, http::StatusCode};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(utoipa::ToSchema, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct OverviewResponse {
    #[ts(type = "number")]
    pub total_watch_time_seconds: i64,
    #[ts(type = "number")]
    pub total_videos_watched: i64,
    #[ts(type = "number")]
    pub total_unique_videos_watched: i64,
    #[ts(type = "number")]
    pub total_channels: i64,
    #[ts(type = "number")]
    pub total_tags: i64,
    #[ts(type = "number")]
    pub average_watch_time_per_session_seconds: i64,
}

/// Returns stats overview
///
/// Quick overview of general stats
#[utoipa::path(
    get,
    path = "/overview",
    tag = "Statistics",
    responses(
        (status = OK, body = OverviewResponse)
    )
)]
pub async fn get_overview(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<OverviewResponse>), (StatusCode, String)> {
    use schema::channels::dsl as channels_dsl;
    use schema::tags::dsl as tags_dsl;
    use schema::videos::dsl as videos_dsl;
    use schema::watch_history::dsl as watch_history_dsl;

    let mut conn = state.pool.get().map_err(internal_error)?;

    let watch_history_list = watch_history_dsl::watch_history
        .load::<WatchHistory>(&mut conn)
        .map_err(internal_error)?;

    let videos_list = videos_dsl::videos
        .load::<Video>(&mut conn)
        .map_err(internal_error)?;

    let channels_list = channels_dsl::channels
        .load::<Channel>(&mut conn)
        .map_err(internal_error)?;

    let tags_list = tags_dsl::tags
        .load::<Tag>(&mut conn)
        .map_err(internal_error)?;

    let total_watch_time_seconds = watch_history_list
        .iter()
        .map(|s| s.watch_duration_seconds)
        .sum::<i64>();
    let total_videos_watched = videos_list.iter().map(|s| s.watch_counter).sum::<i64>();
    let total_unique_videos_watched = videos_list.len() as i64;
    let average_watch_time_per_session_seconds = watch_history_list
        .iter()
        .map(|s| s.watch_duration_seconds)
        .sum::<i64>()
        / watch_history_list.len() as i64;
    let total_channels = channels_list.len() as i64;
    let total_tags = tags_list.len() as i64;

    Ok((
        StatusCode::OK,
        Json(OverviewResponse {
            total_watch_time_seconds,
            total_videos_watched,
            total_channels,
            total_tags,
            total_unique_videos_watched,
            average_watch_time_per_session_seconds,
        }),
    ))
}
