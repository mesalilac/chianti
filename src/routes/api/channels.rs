use crate::api_prelude::*;
use diesel::prelude::*;

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
        (status = OK, description = "List of channels", body = Vec<ChannelResponse>),
    )
)]
pub async fn get_channels(
    State(state): State<AppState>,
    Query(params): Query<GetChannelsParams>,
) -> ApiResult<(StatusCode, Json<Vec<ChannelResponse>>)> {
    use schema::channels::dsl as channels_dsl;
    use schema::tags::dsl as tags_dsl;
    use schema::video_tags::dsl as video_tags_dsl;
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

    let data = query
        .load::<models::Channel>(&mut conn)
        .map_err(internal_error)?;

    let list: Vec<ChannelResponse> = data
        .into_iter()
        .map(|channel| {
            let videos: Vec<VideoResponse> = videos_dsl::videos
                .filter(videos_dsl::channel_id.eq(&channel.id))
                .load::<models::Video>(&mut conn)
                .unwrap_or(Vec::new())
                .into_iter()
                .map(|video| {
                    let tags = tags_dsl::tags
                        .inner_join(video_tags_dsl::video_tags)
                        .filter(video_tags_dsl::video_id.eq(&video.id))
                        .select(tags_dsl::name)
                        .load(&mut conn)
                        .unwrap_or(Vec::new());

                    VideoResponse {
                        thumbnail_endpoint: format!("/api/thumbnails/{}", video.id),
                        video,
                        channel: None,
                        tags,
                    }
                })
                .collect();

            ChannelResponse {
                channel,
                videos: Some(videos),
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
        (status = OK, description = "One channel", body = ChannelResponse),
    )
)]
pub async fn get_channel(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<(StatusCode, Json<ChannelResponse>)> {
    use schema::channels::dsl as channels_dsl;
    use schema::tags::dsl as tags_dsl;
    use schema::video_tags::dsl as video_tags_dsl;
    use schema::videos::dsl as videos_dsl;

    let mut conn = state.pool.get().map_err(internal_error)?;

    let channel = channels_dsl::channels
        .filter(channels_dsl::id.eq(id))
        .get_result::<models::Channel>(&mut conn)
        .map_err(internal_error)?;

    let videos: Vec<VideoResponse> = videos_dsl::videos
        .filter(videos_dsl::channel_id.eq(&channel.id))
        .load::<models::Video>(&mut conn)
        .unwrap_or(Vec::new())
        .into_iter()
        .map(|video| {
            let tags = tags_dsl::tags
                .inner_join(video_tags_dsl::video_tags)
                .filter(video_tags_dsl::video_id.eq(&video.id))
                .select(tags_dsl::name)
                .load(&mut conn)
                .unwrap_or(Vec::new());

            VideoResponse {
                thumbnail_endpoint: format!("/api/thumbnails/{}", video.id),
                video,
                channel: None,
                tags,
            }
        })
        .collect();

    let response = ChannelResponse {
        videos: Some(videos),
        channel,
    };

    Ok((StatusCode::OK, Json(response)))
}
