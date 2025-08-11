use crate::api_prelude::*;
use diesel::prelude::*;

type GetVideosResponse = PaginatedResponse<VideoResponse>;

#[derive(Deserialize, Debug, utoipa::IntoParams)]
pub struct GetVideosParams {
    /// Data list offset
    offset: Option<i64>,
    /// Data list limit
    limit: Option<i64>,
    /// Search videos by title
    search: Option<String>,
    /// List only videos that belong to specified channel
    channel_id: Option<String>,
    /// List only videos that are subscribed to
    is_subscribed: Option<bool>,
    /// Video subscribers_count equal to specified value
    subscribers_count: Option<i64>,
    /// Video subscribers_count greater than specified value
    min_subscribers_count: Option<i64>,
    /// Video subscribers_count less than specified value
    max_subscribers_count: Option<i64>,
    /// List only videos that have specified tags
    tags: Option<Vec<String>>,
    /// Video watch_counter equal to specified value
    watch_counter: Option<i64>,
    /// Video watch_counter greater than specified value
    min_watch_counter: Option<i64>,
    /// Video watch_counter less than specified value
    max_watch_counter: Option<i64>,
    /// Video duration_seconds equal to specified value
    duration_seconds: Option<i64>,
    /// Video duration_seconds greater than specified value
    min_duration_seconds: Option<i64>,
    /// Video duration_seconds less than specified value
    max_duration_seconds: Option<i64>,
    /// Video likes_count equal to specified value
    likes_count: Option<i64>,
    /// Video likes_count greater than specified value
    min_likes_count: Option<i64>,
    /// Video likes_count less than specified value
    max_likes_count: Option<i64>,
    /// Video view_count equal to specified value
    view_count: Option<i64>,
    /// Video view_count greater than specified value
    min_view_count: Option<i64>,
    /// Video view_count less than specified value
    max_view_count: Option<i64>,
    /// Video comments_count equal to specified value
    comments_count: Option<i64>,
    /// Video comments_count greater than specified value
    min_comments_count: Option<i64>,
    /// Video comments_count less than specified value
    max_comments_count: Option<i64>,
    /// Video published_at equal to specified value
    published_at: Option<i64>,
    /// Video published_at before specified timestamp
    published_before: Option<i64>,
    /// Video published_at after specified timestamp
    published_after: Option<i64>,
}

/// Returns videos
///
/// This endpoint is used to fetch videos list
#[utoipa::path(
    get,
    path = "/videos",
    tag = "Video",
    params(
        GetVideosParams
    ),
    responses(
        (status = OK, description = "List of videos", body = GetVideosResponse),
    )
)]
pub async fn get_videos(
    State(state): State<AppState>,
    Query(params): Query<GetVideosParams>,
) -> ApiResult<(StatusCode, Json<GetVideosResponse>)> {
    use schema::channels::dsl as channels_dsl;
    use schema::tags::dsl as tags_dsl;
    use schema::video_tags::dsl as video_tags_dsl;
    use schema::videos::dsl as videos_dsl;

    let mut conn = state.pool.get().map_err(internal_error)?;

    let mut query = videos_dsl::videos
        .inner_join(channels_dsl::channels)
        .left_join(video_tags_dsl::video_tags.inner_join(tags_dsl::tags))
        .select((
            videos_dsl::videos::all_columns(),
            channels_dsl::channels::all_columns(),
        ))
        .distinct()
        .into_boxed();

    if let Some(offset) = params.offset {
        query = query.offset(offset);
    }

    if let Some(limit) = params.limit {
        query = query.limit(limit);
    }

    if let Some(search) = params.search {
        query = query.filter(videos_dsl::title.like(format!("%{search}%")));
    }

    if let Some(channel_id) = params.channel_id {
        query = query.filter(channels_dsl::id.eq(channel_id));
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

    if let Some(tags) = params.tags {
        query = query.filter(tags_dsl::name.eq_any(tags));
    }

    if let Some(watch_counter) = params.watch_counter {
        query = query.filter(videos_dsl::watch_counter.eq(watch_counter));
    }

    if let Some(min_watch_counter) = params.min_watch_counter {
        query = query.filter(videos_dsl::watch_counter.gt(min_watch_counter));
    }

    if let Some(max_watch_counter) = params.max_watch_counter {
        query = query.filter(videos_dsl::watch_counter.lt(max_watch_counter));
    }

    if let Some(duration_seconds) = params.duration_seconds {
        query = query.filter(videos_dsl::duration_seconds.eq(duration_seconds));
    }

    if let Some(min_duration_seconds) = params.min_duration_seconds {
        query = query.filter(videos_dsl::duration_seconds.gt(min_duration_seconds));
    }

    if let Some(max_duration_seconds) = params.max_duration_seconds {
        query = query.filter(videos_dsl::duration_seconds.lt(max_duration_seconds));
    }

    if let Some(likes_count) = params.likes_count {
        query = query.filter(videos_dsl::likes_count.eq(likes_count));
    }

    if let Some(min_likes_count) = params.min_likes_count {
        query = query.filter(videos_dsl::likes_count.gt(min_likes_count));
    }

    if let Some(max_likes_count) = params.max_likes_count {
        query = query.filter(videos_dsl::likes_count.lt(max_likes_count));
    }

    if let Some(view_count) = params.view_count {
        query = query.filter(videos_dsl::view_count.eq(view_count));
    }

    if let Some(min_view_count) = params.min_view_count {
        query = query.filter(videos_dsl::view_count.gt(min_view_count));
    }

    if let Some(max_view_count) = params.max_view_count {
        query = query.filter(videos_dsl::view_count.lt(max_view_count));
    }

    if let Some(comments_count) = params.comments_count {
        query = query.filter(videos_dsl::comments_count.eq(comments_count));
    }

    if let Some(min_comments_count) = params.min_comments_count {
        query = query.filter(videos_dsl::comments_count.gt(min_comments_count));
    }

    if let Some(max_comments_count) = params.max_comments_count {
        query = query.filter(videos_dsl::comments_count.lt(max_comments_count));
    }

    if let Some(published_at) = params.published_at {
        query = query.filter(videos_dsl::published_at.eq(published_at));
    }

    if let Some(published_before) = params.published_before {
        query = query.filter(videos_dsl::published_at.lt(published_before));
    }

    if let Some(published_after) = params.published_after {
        query = query.filter(videos_dsl::published_at.gt(published_after));
    }

    let data = query
        .load::<(models::Video, models::Channel)>(&mut conn)
        .map_err(internal_error)?;

    let list: Vec<VideoResponse> = data
        .into_iter()
        .map(|(video, channel)| {
            let tags = tags_dsl::tags
                .inner_join(video_tags_dsl::video_tags)
                .filter(video_tags_dsl::video_id.eq(&video.id))
                .select(tags_dsl::name)
                .load(&mut conn)
                .unwrap_or(Vec::new());

            VideoResponse {
                thumbnail_endpoint: format!("/api/thumbnails/{}", video.id),
                video,
                tags,
                channel: Some(channel),
            }
        })
        .collect();

    let total = videos_dsl::videos
        .count()
        .get_result::<i64>(&mut conn)
        .unwrap_or(0);

    let res = GetVideosResponse {
        data: list,
        offset: params.offset,
        limit: params.limit,
        total,
    };

    Ok((StatusCode::OK, Json(res)))
}

/// Returns video by id
///
/// This endpoint is used to fetch one video by it's id
#[utoipa::path(
    get,
    path = "/videos/{id}",
    tag = "Video",
    responses(
        (status = OK, description = "One video", body = VideoResponse),
    )
)]
pub async fn get_video(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<(StatusCode, Json<VideoResponse>)> {
    use schema::channels::dsl as channels_dsl;
    use schema::tags::dsl as tags_dsl;
    use schema::video_tags::dsl as video_tags_dsl;
    use schema::videos::dsl as videos_dsl;

    let mut conn = state.pool.get().map_err(internal_error)?;

    let (video, channel) = videos_dsl::videos
        .filter(videos_dsl::id.eq(id))
        .inner_join(channels_dsl::channels)
        .get_result::<(models::Video, models::Channel)>(&mut conn)
        .map_err(internal_error)?;

    let tags = tags_dsl::tags
        .inner_join(video_tags_dsl::video_tags)
        .filter(video_tags_dsl::video_id.eq(&video.id))
        .select(tags_dsl::name)
        .load(&mut conn)
        .unwrap_or(Vec::new());

    let response = VideoResponse {
        thumbnail_endpoint: format!("/api/thumbnails/{}", video.id),
        video,
        tags,
        channel: Some(channel),
    };

    Ok((StatusCode::OK, Json(response)))
}
