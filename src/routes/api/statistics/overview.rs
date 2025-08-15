use crate::api_prelude::*;
use diesel::prelude::*;

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
    #[ts(type = "number")]
    pub average_session_duration_seconds: i64,
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
) -> ApiResult<(StatusCode, Json<OverviewResponse>)> {
    use schema::channels::dsl as channels_dsl;
    use schema::tags::dsl as tags_dsl;
    use schema::videos::dsl as videos_dsl;
    use schema::watch_history::dsl as watch_history_dsl;

    let mut conn = state.pool.get().map_err(internal_error)?;

    let watch_history_list = watch_history_dsl::watch_history
        .load::<models::WatchHistory>(&mut conn)
        .map_err(internal_error)?;

    let videos_list = videos_dsl::videos
        .load::<models::Video>(&mut conn)
        .map_err(internal_error)?;

    let channels_list = channels_dsl::channels
        .load::<models::Channel>(&mut conn)
        .map_err(internal_error)?;

    let tags_list = tags_dsl::tags
        .load::<models::Tag>(&mut conn)
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
    let average_session_duration_seconds = watch_history_list
        .iter()
        .map(|s| s.session_end_date - s.session_start_date)
        .sum::<i64>()
        / watch_history_list.len() as i64;

    Ok((
        StatusCode::OK,
        Json(OverviewResponse {
            total_watch_time_seconds,
            total_videos_watched,
            total_channels,
            total_tags,
            total_unique_videos_watched,
            average_watch_time_per_session_seconds,
            average_session_duration_seconds,
        }),
    ))
}
