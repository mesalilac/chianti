mod database;
mod routes;
mod schema;
mod state;
mod utils;

use axum::{
    body::Bytes,
    extract::MatchedPath,
    http::{HeaderMap, Request},
    response::Response,
    routing::{get, get_service},
};
use clap::Parser;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use routes::{api_routes, handle_404, root};
use state::AppState;
use std::{path::PathBuf, time::Duration};
use tower_http::{
    classify::ServerErrorsFailureClass,
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing::{Span, info_span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa_axum::router::OpenApiRouter;
use utoipa_rapidoc::RapiDoc;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

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

    let channel_avaters_dir = images_directory.join("channel-avatars");
    let video_thumbnails_dir = images_directory.join("video-thumbnails");

    if !channel_avaters_dir.exists() {
        if let Err(e) = std::fs::create_dir_all(&channel_avaters_dir) {
            tracing::error!("Failed to create channel avaters directory: {}", e);
        }
    }

    if !video_thumbnails_dir.exists() {
        if let Err(e) = std::fs::create_dir_all(&video_thumbnails_dir) {
            tracing::error!("Failed to create video thumbnails directory: {}", e);
        }
    }

    let app_state = AppState {
        pool: database::connection::create_connection_pool(data_path),
        channel_avaters_dir,
        video_thumbnails_dir,
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

    let (openapi_router, mut api_doc) = OpenApiRouter::<AppState>::new()
        .nest("/api", api_routes())
        .split_for_parts();

    let mut contact = utoipa::openapi::Contact::new();

    contact.name = Some("mesalilac".into());
    contact.url = Some("https://github.com/mesalilac".into());

    api_doc.info.title = "Chianti API".to_string();
    api_doc.info.version = env!("CARGO_PKG_VERSION").to_string();
    api_doc.info.contact = Some(contact);
    api_doc.info.description = Some(String::from(
        "Collect info about the youtube videos you watch.",
    ));

    let rapi_doc = RapiDoc::with_openapi("/api-docs/openapi.json", api_doc).path("/docs");

    // build our application with a route
    let app = openapi_router
        .route("/", get(root))
        .merge(rapi_doc)
        .nest_service("/stats", webui_html_file)
        .nest_service("/assets", webui_assets)
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
