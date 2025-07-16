mod database;
mod schema;

use axum::{
    Json, Router,
    body::Bytes,
    extract::{MatchedPath, State},
    http::{HeaderMap, Request, StatusCode},
    response::Response,
    routing::{get, get_service, post},
};
use database::models::{Channel, Video, WatchHistory};
use diesel::{RunQueryDsl, dsl::insert_into};
use serde::Deserialize;
use std::time::Duration;
use tower_http::{
    classify::ServerErrorsFailureClass,
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing::{Span, info_span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[derive(Clone)]
struct AppState {
    pool: database::connection::DbPool,
}

#[tokio::main]
async fn main() {
    let webui_html_file = get_service(ServeFile::new("/app/dist/index.html"));
    let webui_assets = get_service(ServeDir::new("/app/dist/assets"));

    let app_state = AppState {
        pool: database::connection::create_connection_pool(),
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

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .nest_service("/stats", webui_html_file)
        .nest_service("/assets", webui_assets)
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
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3241").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
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

/// check if the server is online
async fn ping() -> (StatusCode, String) {
    (StatusCode::OK, "Server is online".to_string())
}

#[derive(Deserialize)]
struct CreateWatchHistory {
    // For channel
    channel_id: String,
    channel_name: String,
    channel_subscribers_count: i64,

    // For video
    video_id: String,
    video_title: String,
    video_duration: i64,
    published_at: i64,
    view_count: i64,
    watch_duration_seconds: i64,
    session_start_date: i64,
    session_end_date: i64,
}

async fn create_watch_history(
    State(state): State<AppState>,
    Json(payload): Json<CreateWatchHistory>,
) -> Result<StatusCode, (StatusCode, String)> {
    use schema::channels::dsl::*;
    use schema::videos::dsl::*;
    use schema::watch_history::dsl::*;

    let mut conn = state.pool.get().map_err(internal_error)?;

    let channel = Channel::new(
        payload.channel_id.clone(),
        payload.channel_name.clone(),
        payload.channel_subscribers_count,
    );

    insert_into(channels)
        .values(&channel)
        .on_conflict_do_nothing()
        .execute(&mut conn)
        .map_err(internal_error)?;

    let video = Video::new(
        payload.video_id,
        payload.channel_id,
        payload.video_title,
        payload.video_duration,
        payload.watch_duration_seconds,
        payload.view_count,
        payload.published_at,
        payload.session_start_date,
        payload.session_end_date,
    );

    insert_into(videos)
        .values(&video)
        .on_conflict_do_nothing()
        .execute(&mut conn)
        .map_err(internal_error)?;

    let new_watch_history = WatchHistory::new(video.id, channel.id);

    insert_into(watch_history)
        .values(&new_watch_history)
        .on_conflict_do_nothing()
        .execute(&mut conn)
        .map_err(internal_error)?;

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
