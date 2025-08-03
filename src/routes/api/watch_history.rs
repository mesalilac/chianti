use crate::database::models::{Channel, Tag, Video, VideoTags, WatchHistory};
use crate::schema;
use crate::state::AppState;
use crate::utils::internal_error;
use crate::utils::{build_avater_cache_image_filename, build_thumbnail_cache_image_filename};
use axum::{Json, extract::State, http::StatusCode};
use diesel::prelude::*;
use diesel::{ExpressionMethods, RunQueryDsl, dsl::insert_into};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::database::models::{NewChannelParams, NewVideoParams};

#[derive(utoipa::ToSchema, Deserialize, TS)]
#[ts(export)]
struct CreateWatchHistoryChannel {
    id: String,
    name: String,
    avater_url: String,
    url: String,
    is_subscribed: bool,
    #[ts(type = "number")]
    subscribers_count: i64,
}

#[derive(utoipa::ToSchema, Deserialize, TS)]
#[ts(export)]
pub struct CreateWatchHistoryVideo {
    id: String,
    title: String,
    description: String,
    thumbnail_url: String,
    tags: Vec<String>,
    #[ts(type = "number")]
    likes_count: i64,
    #[ts(type = "number")]
    view_count: i64,
    #[ts(type = "number")]
    comments_count: i64,
    #[ts(type = "number")]
    duration: i64,
    #[ts(type = "number")]
    published_at: i64,
}

#[derive(utoipa::ToSchema, Deserialize, TS)]
#[ts(export)]
pub struct CreateWatchHistoryRequest {
    #[ts(type = "number")]
    watch_duration_seconds: i64,
    #[ts(type = "number")]
    session_start_date: i64,
    #[ts(type = "number")]
    session_end_date: i64,

    channel: CreateWatchHistoryChannel,
    video: CreateWatchHistoryVideo,
}

/// Create new watch history records
///
/// This endpoint is used to create new watch history records
#[utoipa::path(
    post,
    path = "/watch_history",
    tag = "Watch history",
    responses(
        (status = CREATED, description = "Watch history record created", body = ()),
    )
)]
pub async fn create_watch_history(
    State(state): State<AppState>,
    Json(payload_list): Json<Vec<CreateWatchHistoryRequest>>,
) -> Result<StatusCode, (StatusCode, String)> {
    use schema::channels::dsl as channels_dsl;
    use schema::tags::dsl as tags_dsl;
    use schema::video_tags::dsl as video_tags_dsl;
    use schema::videos::dsl as videos_dsl;
    use schema::watch_history::dsl as watch_history_dsl;

    let mut conn = state.pool.get().map_err(internal_error)?;

    for payload in payload_list {
        let channel_avater_file_path = state
            .channel_avaters_dir
            .join(build_avater_cache_image_filename(&payload.channel.id));
        let video_thumbnail_file_path = state
            .video_thumbnails_dir
            .join(build_thumbnail_cache_image_filename(&payload.video.id));

        if !channel_avater_file_path.exists() {
            tracing::info!(
                "Downloading channel avater for channel {}",
                payload.channel.id
            );
            let res = reqwest::get(&payload.channel.avater_url)
                .await
                .map_err(internal_error)?;

            if res.status() == reqwest::StatusCode::OK {
                let image = res.bytes().await.map_err(internal_error)?;

                image::load_from_memory(&image)
                    .map_err(internal_error)?
                    .save_with_format(&channel_avater_file_path, image::ImageFormat::WebP)
                    .map_err(internal_error)?;
            } else {
                tracing::warn!(
                    "Failed to download channel avater for channel {}",
                    payload.channel.id
                );
            }
        }

        if !video_thumbnail_file_path.exists() {
            tracing::info!("Downloading video thumbnail for video {}", payload.video.id);
            let res = reqwest::get(&payload.video.thumbnail_url)
                .await
                .map_err(internal_error)?;

            if res.status() == reqwest::StatusCode::OK {
                let image = res.bytes().await.map_err(internal_error)?;

                image::load_from_memory(&image)
                    .map_err(internal_error)?
                    .save_with_format(&video_thumbnail_file_path, image::ImageFormat::WebP)
                    .map_err(internal_error)?;
            } else {
                tracing::warn!(
                    "Failed to download video thumbnail for video {}",
                    payload.video.id
                );
            }
        }

        let channel = Channel::new(NewChannelParams {
            id: payload.channel.id.clone(),
            name: payload.channel.name.clone(),
            url: payload.channel.url,
            is_subscribed: payload.channel.is_subscribed,
            subscribers_count: payload.channel.subscribers_count,
        });

        insert_into(channels_dsl::channels)
            .values(&channel)
            .on_conflict(channels_dsl::id)
            .do_update()
            .set((
                channels_dsl::name.eq(payload.channel.name),
                channels_dsl::is_subscribed.eq(payload.channel.is_subscribed),
                channels_dsl::subscribers_count.eq(payload.channel.subscribers_count),
            ))
            .execute(&mut conn)
            .map_err(internal_error)?;

        let video = Video::new(NewVideoParams {
            id: payload.video.id,
            channel_id: payload.channel.id,
            title: payload.video.title.clone(),
            description: payload.video.description,
            duration_seconds: payload.video.duration,
            likes_count: payload.video.likes_count,
            view_count: payload.video.view_count,
            comments_count: payload.video.comments_count,
            published_at: payload.video.published_at,
        });

        insert_into(videos_dsl::videos)
            .values(&video)
            .on_conflict(videos_dsl::id)
            .do_update()
            .set((
                videos_dsl::title.eq(payload.video.title),
                videos_dsl::view_count.eq(payload.video.view_count),
                videos_dsl::likes_count.eq(payload.video.likes_count),
                videos_dsl::comments_count.eq(payload.video.comments_count),
            ))
            .execute(&mut conn)
            .map_err(internal_error)?;

        for tag_name in payload.video.tags {
            let tag = match tags_dsl::tags
                .filter(tags_dsl::name.eq(&tag_name))
                .get_result::<Tag>(&mut conn)
            {
                Ok(r) => r,
                Err(_) => {
                    let new_tag = Tag::new(tag_name);

                    insert_into(tags_dsl::tags)
                        .values(&new_tag)
                        .on_conflict_do_nothing()
                        .execute(&mut conn)
                        .map_err(internal_error)?;

                    new_tag
                }
            };

            let video_tag = VideoTags::new(video.id.clone(), tag.id);

            insert_into(video_tags_dsl::video_tags)
                .values(&video_tag)
                .on_conflict_do_nothing()
                .execute(&mut conn)
                .map_err(internal_error)?;
        }

        let new_watch_history = WatchHistory::new(
            video.id,
            channel.id,
            payload.watch_duration_seconds,
            payload.session_start_date,
            payload.session_end_date,
        );

        insert_into(watch_history_dsl::watch_history)
            .values(&new_watch_history)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .map_err(internal_error)?;
    }

    Ok(StatusCode::CREATED)
}

#[derive(utoipa::ToSchema, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct GetWatchHistoryResponse {
    pub id: String,
    pub video: Video,
    pub channel: Channel,
    pub watch_duration_seconds: i64,
    pub session_start_date: i64,
    pub session_end_date: i64,
    pub added_at: i64,
}

/// Returns watch history records
///
/// This endpoint is used to fetch watch history records
#[utoipa::path(
    get,
    path = "/watch_history",
    tag = "Watch history",
    responses(
        (status = OK, description = "List of watch history records", body = Vec<GetWatchHistoryResponse>),
    )
)]
pub async fn get_watch_history(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<GetWatchHistoryResponse>>), (StatusCode, String)> {
    use schema::channels::dsl as channels_dsl;
    use schema::videos::dsl as videos_dsl;
    use schema::watch_history::dsl as watch_history_dsl;

    let mut conn = state.pool.get().map_err(internal_error)?;

    let data = watch_history_dsl::watch_history
        .inner_join(channels_dsl::channels)
        .inner_join(videos_dsl::videos)
        .load::<(WatchHistory, Channel, Video)>(&mut conn)
        .map_err(internal_error)?;

    let list = data
        .iter()
        .map(|(watch_history, channel, video)| GetWatchHistoryResponse {
            id: watch_history.id.clone(),
            video: video.clone(),
            channel: channel.clone(),
            watch_duration_seconds: watch_history.watch_duration_seconds,
            session_start_date: watch_history.session_start_date,
            session_end_date: watch_history.session_end_date,
            added_at: watch_history.added_at,
        })
        .collect::<Vec<GetWatchHistoryResponse>>();

    Ok((StatusCode::OK, Json(list)))
}
