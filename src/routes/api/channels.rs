use crate::api_prelude::*;
use diesel::prelude::*;

type GetChannelsResponse = PaginatedResponse<ChannelResponse>;

#[derive(Debug, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
enum SortBy {
    Name,
    IsSubscribed,
    SubscribersCount,
}

#[derive(Deserialize, Debug, utoipa::IntoParams)]
pub struct GetChannelsParams {
    /// Sort order
    sort_order: Option<SortOrder>,
    /// Sort by specified field
    sort_by: Option<SortBy>,
    /// Data list offset
    offset: Option<i64>,
    /// Data list limit
    limit: Option<i64>,
    /// Search channels by name
    search: Option<String>,
    /// List only channels that are subscribed to
    is_subscribed: Option<bool>,
    /// Channel subscribers_count equal to specified value
    subscribers_count: Option<i64>,
    /// Channel subscribers_count greater than specified value
    min_subscribers_count: Option<i64>,
    /// Channel subscribers_count less than specified value
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
        GetChannelsParams
    ),
    responses(
        (status = OK, description = "List of channels", body = PaginatedResponse<ChannelResponse>),
    )
)]
pub async fn get_channels(
    State(state): State<AppState>,
    Query(params): Query<GetChannelsParams>,
) -> ApiResult<(StatusCode, Json<GetChannelsResponse>)> {
    use schema::channels::dsl as channels_dsl;
    use schema::tags::dsl as tags_dsl;
    use schema::video_tags::dsl as video_tags_dsl;
    use schema::videos::dsl as videos_dsl;

    let mut conn = state.pool.get().map_err(internal_error)?;

    let mut query = channels_dsl::channels.into_boxed();

    if let Some(offset) = params.offset {
        query = query.offset(offset);
    }

    if let Some(limit) = params.limit {
        query = query.limit(limit);
    }

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

    if let Some(sort_by) = params.sort_by {
        query = match sort_by {
            SortBy::Name => apply_sort!(query, channels_dsl::name, params.sort_order),
            SortBy::IsSubscribed => {
                apply_sort!(query, channels_dsl::is_subscribed, params.sort_order)
            }
            SortBy::SubscribersCount => {
                apply_sort!(query, channels_dsl::subscribers_count, params.sort_order)
            }
        };
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

    let total = channels_dsl::channels
        .count()
        .get_result::<i64>(&mut conn)
        .unwrap_or(0);

    let res = GetChannelsResponse {
        data: list,
        offset: params.offset,
        limit: params.limit,
        total,
    };

    Ok((StatusCode::OK, Json(res)))
}

/// Returns channel by id
///
/// This endpoint is used to fetch one channel by it's id
#[utoipa::path(
    get,
    path = "/channels/{id}",
    tag = "Channel",
    params(
        ("id" = String, Path, description = "Channel id")
    ),
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
