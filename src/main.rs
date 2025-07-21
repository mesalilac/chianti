mod database;
mod schema;

use axum::{
    Json, Router,
    body::{Body, Bytes},
    extract::{MatchedPath, Path, State},
    http::{HeaderMap, Request, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, get_service, post},
};
use clap::Parser;
use database::models::{Channel, Tag, Video, VideoTags, WatchHistory};
use diesel::prelude::*;
use diesel::{ExpressionMethods, RunQueryDsl, dsl::insert_into};
use serde::Deserialize;
use std::{path::PathBuf, time::Duration};
use tower_http::{
    classify::ServerErrorsFailureClass,
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing::{Span, info_span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use ts_rs::TS;

use crate::database::models::{NewChannelParams, NewVideoParams};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[derive(Clone)]
struct AppState {
    pool: database::connection::DbPool,
    channel_avaters_directory: std::path::PathBuf,
    video_thumbnails_directory: std::path::PathBuf,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 8080)]
    port: u16,

    #[arg(short, long)]
    data_dir: Option<String>,

    #[arg(short, long)]
    frontend_dir: Option<String>,
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let args = Args::parse();
    let local_dir = match std::env::var("XDG_DATA_HOME") {
        Ok(p) => PathBuf::from(p).join("chianti"),
        Err(_) => PathBuf::from("/usr").join("share").join("chianti"),
    };

    let data_dir: PathBuf = match args.data_dir {
        Some(p) => PathBuf::from(p),
        None => local_dir.join("data"),
    };

    let frontend_dir: PathBuf = match args.frontend_dir {
        Some(p) => PathBuf::from(p),
        None => local_dir.join("frontend"),
    };

    let data_path = if cfg!(debug_assertions) {
        PathBuf::from("./dev-data")
    } else {
        data_dir
    };

    let frontend_path = if cfg!(debug_assertions) {
        PathBuf::from("./web/dist")
    } else {
        frontend_dir
    };

    let html_path = frontend_path.join("index.html");
    let assets_path = frontend_path.join("assets");

    if !data_path.exists() {
        if let Err(e) = std::fs::create_dir_all(&data_path) {
            tracing::error!("Failed to create data directory: {}", e);
            std::process::exit(1);
        }
    }

    tracing::debug!("Data directory: {}", data_path.display());
    tracing::debug!("Frontend directory: {}", frontend_path.display());

    let webui_html_file = get_service(ServeFile::new(html_path));
    let webui_assets = get_service(ServeDir::new(assets_path));

    let images_directory = data_path.join("images");

    let channel_avaters_directory = images_directory.join("channel-avatars");
    let video_thumbnails_directory = images_directory.join("video-thumbnails");

    if !channel_avaters_directory.exists() {
        if let Err(e) = std::fs::create_dir_all(&channel_avaters_directory) {
            tracing::error!("Failed to create channel avaters directory: {}", e);
        }
    }

    if !video_thumbnails_directory.exists() {
        if let Err(e) = std::fs::create_dir_all(&video_thumbnails_directory) {
            tracing::error!("Failed to create video thumbnails directory: {}", e);
        }
    }

    let app_state = AppState {
        pool: database::connection::create_connection_pool(data_path),
        channel_avaters_directory,
        video_thumbnails_directory,
    };

    if let Ok(mut conn) = app_state.pool.get() {
        match conn.run_pending_migrations(MIGRATIONS) {
            Ok(_) => {
                tracing::debug!("Successfully ran migrations");
            }
            Err(e) => {
                tracing::error!("Failed to run migrations: {}", e);
                std::process::exit(1);
            }
        }
    }

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .nest_service("/stats", webui_html_file)
        .nest_service("/assets", webui_assets)
        .route("/api/avater/{channel_id}", get(get_channel_avater))
        .route("/api/thumbnail/{video_id}", get(get_video_thumbnail))
        .route("/api/ping", get(ping))
        .route("/api/watch_history", post(create_watch_history))
        .fallback(handle_404)
        .with_state(app_state)
        .layer(CorsLayer::permissive())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    // Log the matched route's path (with placeholders not filled in).
                    // Use request.uri() or OriginalUri if you want the real path.
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path,
                        some_other_field = tracing::field::Empty,
                    )
                })
                .on_request(|_request: &Request<_>, _span: &Span| {
                    // You can use `_span.record("some_other_field", value)` in one of these
                    // closures to attach a value to the initially empty field in the info_span
                    // created above.
                })
                .on_response(|response: &Response, latency: Duration, span: &Span| {
                    span.record("status_code", response.status().as_u16());
                    span.record("latency", format!("{latency:?}"));
                    tracing::info!(
                        "Finished request. Status: {}, Latency: {:?}",
                        response.status(),
                        latency
                    );
                })
                .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {
                    // ...
                })
                .on_eos(
                    |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {
                        // ...
                    },
                )
                .on_failure(
                    |error: ServerErrorsFailureClass, _latency: Duration, span: &Span| {
                        span.record("server_error_failure_class", format!("{error}"));
                        tracing::error!("Request failed: {:?}", error);
                    },
                ),
        );

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", args.port))
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn cache_image_filename(filename: &String) -> String {
    let base = base32::encode(base32::Alphabet::Crockford, filename.as_bytes());

    format!("{base}.webp")
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn handle_404() -> (StatusCode, String) {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found".to_string(),
    )
}

async fn get_channel_avater(
    State(state): State<AppState>,
    Path(channel_id): Path<String>,
) -> impl IntoResponse {
    let avater_file_path = state
        .channel_avaters_directory
        .join(cache_image_filename(&channel_id));

    let Ok(file) = tokio::fs::File::open(&avater_file_path).await else {
        return (StatusCode::NOT_FOUND).into_response();
    };

    let Some(content_type) = mime_guess::from_path(&avater_file_path).first_raw() else {
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    };

    let stream = tokio_util::io::ReaderStream::new(file);
    let body = Body::from_stream(stream);

    match Response::builder()
        .header("Content-Type", content_type)
        .body(body)
    {
        Ok(response) => response,
        Err(err) => {
            tracing::error!("Failed to create response: {err}");
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}

async fn get_video_thumbnail(
    State(state): State<AppState>,
    Path(video_id): Path<String>,
) -> impl IntoResponse {
    let thumbnail_file_path = state
        .video_thumbnails_directory
        .join(cache_image_filename(&video_id));

    let Ok(file) = tokio::fs::File::open(&thumbnail_file_path).await else {
        return (StatusCode::NOT_FOUND).into_response();
    };

    let Some(content_type) = mime_guess::from_path(&thumbnail_file_path).first_raw() else {
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    };

    let stream = tokio_util::io::ReaderStream::new(file);
    let body = Body::from_stream(stream);

    match Response::builder()
        .header("Content-Type", content_type)
        .body(body)
    {
        Ok(response) => response,
        Err(err) => {
            tracing::error!("Failed to create response: {err}");
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}

/// check if the server is online
async fn ping() -> (StatusCode, String) {
    (StatusCode::OK, "Server is online".to_string())
}

#[derive(Deserialize, TS)]
#[ts(export)]
struct CreateWatchHistoryChannel {
    id: String,
    name: String,
    avater_url: String,
    url: String,
    #[ts(type = "number")]
    subscribers_count: i64,
}

#[derive(Deserialize, TS)]
#[ts(export)]
struct CreateWatchHistoryVideo {
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
    duration: i64,
    #[ts(type = "number")]
    published_at: i64,
}

#[derive(Deserialize, TS)]
#[ts(export)]
struct CreateWatchHistoryRequest {
    #[ts(type = "number")]
    watch_duration_seconds: i64,
    #[ts(type = "number")]
    session_start_date: i64,
    #[ts(type = "number")]
    session_end_date: i64,

    channel: CreateWatchHistoryChannel,
    video: CreateWatchHistoryVideo,
}

async fn create_watch_history(
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
            .channel_avaters_directory
            .join(cache_image_filename(&payload.channel.id));
        let video_thumbnail_file_path = state
            .video_thumbnails_directory
            .join(cache_image_filename(&payload.video.id));

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
            subscribers_count: payload.channel.subscribers_count,
        });

        insert_into(channels_dsl::channels)
            .values(&channel)
            .on_conflict(channels_dsl::id)
            .do_update()
            .set((
                channels_dsl::name.eq(payload.channel.name),
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

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    tracing::error!("Unhandled internal error: {}", err);
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
