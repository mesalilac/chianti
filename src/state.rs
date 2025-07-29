use crate::database::connection::DbPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
    pub channel_avaters_dir: std::path::PathBuf,
    pub video_thumbnails_dir: std::path::PathBuf,
}
