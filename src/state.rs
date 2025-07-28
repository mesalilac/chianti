use crate::database::connection::DbPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
    pub channel_avaters_directory: std::path::PathBuf,
    pub video_thumbnails_directory: std::path::PathBuf,
}
