use crate::database::models::{Channel, Video};
use crate::schema;
use crate::state::AppState;
use crate::utils::internal_error;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use axum_extra::extract::Query;
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

#[derive(Deserialize, Debug)]
pub struct GetChannelsParams {
    search: Option<String>,
    is_subscribed: Option<bool>,
    subscribers_count: Option<i64>,
    min_subscribers_count: Option<i64>,
    max_subscribers_count: Option<i64>,
}

/// Returns channels
///
/// This endpoint is used to fetch channels list
#[utoipa::path(
    get,
    path = "/channels",
    tag = "Channel",
    params(
        ("search" = String, description = "Search channels by name"),
        ("is_subscribed" = bool, description = "List only channels that are subscribed to (is_subscribed=true)"),
        ("subscribers_count" = i64, description = "Channel subscribers_count equal to specified value"),
        ("min_subscribers_count" = i64, description = "Channel subscribers_count greater than specified value"),
        ("max_subscribers_count" = i64, description = "Channel subscribers_count less than specified value"),
    ),
    responses(
        (status = OK, description = "List of channels", body = Vec<ChannelWithVideosResponse>),
    )
)]
pub async fn get_channels(
    State(state): State<AppState>,
    Query(params): Query<GetChannelsParams>,
) -> Result<(StatusCode, Json<Vec<ChannelWithVideosResponse>>), (StatusCode, String)> {
    use schema::channels::dsl as channels_dsl;
    use schema::videos::dsl as videos_dsl;

    let mut conn = state.pool.get().map_err(internal_error)?;

    let mut query = channels_dsl::channels.into_boxed();

    if let Some(search) = params.search {
        query = query.filter(channels_dsl::name.like(format!("%{search}%")));
    }

    if let Some(is_subscribed) = params.is_subscribed {
        query = query.filter(channels_dsl::is_subscribed.eq(is_subscribed));
    }

    if let Some(subscribers_count) = params.subscribers_count {
        query = query.filter(channels_dsl::subscribers_count.eq(subscribers_count));
    }

    if let Some(min_subscribers_count) = params.min_subscribers_count {
        query = query.filter(channels_dsl::subscribers_count.gt(min_subscribers_count));
    }

    if let Some(max_subscribers_count) = params.max_subscribers_count {
        query = query.filter(channels_dsl::subscribers_count.lt(max_subscribers_count));
    }

    let data = query.load::<Channel>(&mut conn).map_err(internal_error)?;

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
