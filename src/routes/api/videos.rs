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

/// Returns videos
///
/// This endpoint is used to fetch videos list
#[utoipa::path(
    get,
    path = "/videos",
    tag = "Video",
    responses(
        (status = OK, description = "List of videos", body = Vec<VideoResponse>),
    )
)]
pub async fn get_videos(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<VideoResponse>>), (StatusCode, String)> {
    use schema::channels::dsl as channels_dsl;
    use schema::tags::dsl as tags_dsl;
    use schema::video_tags::dsl as video_tags_dsl;
    use schema::videos::dsl as videos_dsl;

    let mut conn = state.pool.get().map_err(internal_error)?;

    let data = videos_dsl::videos
        .inner_join(channels_dsl::channels)
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
