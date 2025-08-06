use crate::database::models::{Channel, Video};
use crate::schema;
use crate::state::AppState;
use crate::utils::internal_error;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(utoipa::ToSchema, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ChannelWithVideosResponse {
    pub id: String,
    pub name: String,
    pub url: String,
    pub is_subscribed: bool,
    pub subscribers_count: i64,
    pub videos: Vec<Video>,
    pub added_at: i64,
}

/// Returns channels
///
/// This endpoint is used to fetch channels list
#[utoipa::path(
    get,
    path = "/channels",
    tag = "Channel",
    responses(
        (status = OK, description = "List of channels", body = Vec<ChannelWithVideosResponse>),
    )
)]
pub async fn get_channels(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<ChannelWithVideosResponse>>), (StatusCode, String)> {
    use schema::channels::dsl as channels_dsl;
    use schema::videos::dsl as videos_dsl;

    let mut conn = state.pool.get().map_err(internal_error)?;

    let data = channels_dsl::channels
        .load::<Channel>(&mut conn)
        .map_err(internal_error)?;

    let list: Vec<ChannelWithVideosResponse> = data
        .iter()
        .map(|channel| {
            let videos = videos_dsl::videos
                .filter(videos_dsl::channel_id.eq(&channel.id))
                .load::<Video>(&mut conn)
                .unwrap_or(Vec::new());

            ChannelWithVideosResponse {
                id: channel.id.clone(),
                name: channel.name.clone(),
                url: channel.url.clone(),
                is_subscribed: channel.is_subscribed,
                subscribers_count: channel.subscribers_count,
                videos,
                added_at: channel.added_at,
            }
        })
        .collect();

    Ok((StatusCode::OK, Json(list)))
}

/// Returns channel by id
///
/// This endpoint is used to fetch one channel by it's id
#[utoipa::path(
    get,
    path = "/channels/{id}",
    tag = "Channel",
    responses(
        (status = OK, description = "One channel", body = ChannelWithVideosResponse),
    )
)]
pub async fn get_channel(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<(StatusCode, Json<ChannelWithVideosResponse>), (StatusCode, String)> {
    use schema::channels::dsl as channels_dsl;
    use schema::videos::dsl as videos_dsl;

    let mut conn = state.pool.get().map_err(internal_error)?;

    let channel = channels_dsl::channels
        .filter(channels_dsl::id.eq(id))
        .get_result::<Channel>(&mut conn)
        .map_err(internal_error)?;

    let videos = videos_dsl::videos
        .filter(videos_dsl::channel_id.eq(&channel.id))
        .load::<Video>(&mut conn)
        .unwrap_or(Vec::new());

    let response = ChannelWithVideosResponse {
        id: channel.id.clone(),
        name: channel.name.clone(),
        url: channel.url.clone(),
        is_subscribed: channel.is_subscribed,
        subscribers_count: channel.subscribers_count,
        videos,
        added_at: channel.added_at,
    };

    Ok((StatusCode::OK, Json(response)))
}
