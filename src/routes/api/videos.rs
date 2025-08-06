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
pub struct VideoResponse {
    pub id: String,
    pub channel: Channel,
    pub url: String,
    pub thumbnail_endpoint: String,
    pub title: String,
    pub description: String,
    pub watch_counter: i64,
    pub duration_seconds: i64,
    pub likes_count: i64,
    pub view_count: i64,
    pub comments_count: i64,
    pub published_at: i64,
    pub tags: Vec<String>,
    pub added_at: i64,
}

#[derive(Deserialize, Debug)]
pub struct GetVideosParams {
    search: Option<String>,
    is_subscribed: Option<bool>,
    subscribers_count: Option<i64>,
    min_subscribers_count: Option<i64>,
    max_subscribers_count: Option<i64>,
    tags: Option<Vec<String>>,
    watch_counter: Option<i64>,
    min_watch_counter: Option<i64>,
    max_watch_counter: Option<i64>,
    duration_seconds: Option<i64>,
    min_duration_seconds: Option<i64>,
    max_duration_seconds: Option<i64>,
    likes_count: Option<i64>,
    min_likes_count: Option<i64>,
    max_likes_count: Option<i64>,
    view_count: Option<i64>,
    min_view_count: Option<i64>,
    max_view_count: Option<i64>,
    comments_count: Option<i64>,
    min_comments_count: Option<i64>,
    max_comments_count: Option<i64>,
    published_at: Option<i64>,
    published_before: Option<i64>,
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
        ("search" = Option<String>, description = "Search videos by title"),
        ("is_subscribed" = Option<bool>, description = "List only videos that belong to subscribed channels (is_subscribed=true)"),
        ("subscribers_count" = Option<i64>, description = "Channel subscribers_count equal to specified value"),
        ("min_subscribers_count" = Option<i64>, description = "Channel subscribers_count greater than specified value"),
        ("max_subscribers_count" = Option<i64>, description = "Channel subscribers_count less than specified value"),
        ("tags" = Option<Vec<String>>, description = "List only videos that include specified tags (tags=x&tags=y&tags=z)"),
        ("watch_counter" = Option<i64>, description = "Video watch_counter equal to specified value"),
        ("min_watch_counter" = Option<i64>, description = "Video watch_counter greater than specified value"),
        ("max_watch_counter" = Option<i64>, description = "Video watch_counter less than specified value"),
        ("duration_seconds" = Option<i64>, description = "Video duration_seconds equal to specified value"),
        ("min_duration_seconds" = Option<i64>, description = "Video duration_seconds greater than specified value"),
        ("max_duration_seconds" = Option<i64>, description = "Video duration_seconds less than specified value"),
        ("likes_count" = Option<i64>, description = "Video likes_count equal to specified value"),
        ("min_likes_count" = Option<i64>, description = "Video likes_count greater than specified value"),
        ("max_likes_count" = Option<i64>, description = "Video likes_count less than specified value"),
        ("view_count" = Option<i64>, description = "Video view_count equal to specified value"),
        ("min_view_count" = Option<i64>, description = "Video view_count greater than specified value"),
        ("max_view_count" = Option<i64>, description = "Video view_count less than specified value"),
        ("comments_count" = Option<i64>, description = "Video comments_count equal to specified value"),
        ("min_comments_count" = Option<i64>, description = "Video comments_count greater than specified value"),
        ("max_comments_count" = Option<i64>, description = "Video comments_count less than specified value"),
        ("published_at" = Option<i64>, description = "Video published_at equal to specified value"),
        ("published_before" = Option<i64>, description = "Video published_at before specified value"),
        ("published_after" = Option<i64>, description = "Video published_at after specified value"),
    ),
    responses(
        (status = OK, description = "List of videos", body = Vec<VideoResponse>),
    )
)]
pub async fn get_videos(
    State(state): State<AppState>,
    Query(params): Query<GetVideosParams>,
) -> Result<(StatusCode, Json<Vec<VideoResponse>>), (StatusCode, String)> {
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

    if let Some(search) = params.search {
        query = query.filter(videos_dsl::title.like(format!("%{search}%")));
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
        .load::<(Video, Channel)>(&mut conn)
        .map_err(internal_error)?;

    let list: Vec<VideoResponse> = data
        .iter()
        .map(|(video, channel)| {
            let tags = tags_dsl::tags
                .inner_join(video_tags_dsl::video_tags)
                .filter(video_tags_dsl::video_id.eq(&video.id))
                .select(tags_dsl::name)
                .load(&mut conn)
                .unwrap_or(Vec::new());

            VideoResponse {
                id: video.id.clone(),
                channel: channel.clone(),
                url: video.url.clone(),
                thumbnail_endpoint: format!("/api/thumbnails/{}", video.id),
                title: video.title.clone(),
                description: video.description.clone(),
                watch_counter: video.watch_counter,
                duration_seconds: video.duration_seconds,
                likes_count: video.likes_count,
                view_count: video.view_count,
                comments_count: video.comments_count,
                published_at: video.published_at,
                tags,
                added_at: video.added_at,
            }
        })
        .collect();

    Ok((StatusCode::OK, Json(list)))
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
) -> Result<(StatusCode, Json<VideoResponse>), (StatusCode, String)> {
    use schema::channels::dsl as channels_dsl;
    use schema::tags::dsl as tags_dsl;
    use schema::video_tags::dsl as video_tags_dsl;
    use schema::videos::dsl as videos_dsl;

    let mut conn = state.pool.get().map_err(internal_error)?;

    let (video, channel) = videos_dsl::videos
        .filter(videos_dsl::id.eq(id))
        .inner_join(channels_dsl::channels)
        .get_result::<(Video, Channel)>(&mut conn)
        .map_err(internal_error)?;

    let tags = tags_dsl::tags
        .inner_join(video_tags_dsl::video_tags)
        .filter(video_tags_dsl::video_id.eq(&video.id))
        .select(tags_dsl::name)
        .load(&mut conn)
        .unwrap_or(Vec::new());

    let response = VideoResponse {
        id: video.id.clone(),
        channel: channel.clone(),
        url: video.url.clone(),
        thumbnail_endpoint: format!("/api/thumbnails/{}", video.id),
        title: video.title.clone(),
        description: video.description.clone(),
        watch_counter: video.watch_counter,
        duration_seconds: video.duration_seconds,
        likes_count: video.likes_count,
        view_count: video.view_count,
        comments_count: video.comments_count,
        published_at: video.published_at,
        tags,
        added_at: video.added_at,
    };

    Ok((StatusCode::OK, Json(response)))
}
