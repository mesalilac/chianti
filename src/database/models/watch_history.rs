use super::{Channel, Video, prelude::*};

#[derive(
    Queryable,
    Identifiable,
    Associations,
    Insertable,
    Serialize,
    Deserialize,
    Debug,
    Clone,
    utoipa::ToSchema,
    TS,
)]
#[diesel(table_name = schema::watch_history)]
#[diesel(belongs_to(Video, foreign_key = video_id))]
#[diesel(belongs_to(Channel, foreign_key = channel_id))]
#[diesel(check_for_backend(Sqlite))]
pub struct WatchHistory {
    pub id: String,
    #[serde(skip)]
    pub video_id: String,
    #[serde(skip)]
    pub channel_id: String,
    #[ts(type = "number")]
    pub watch_duration_seconds: i64,
    #[ts(type = "number")]
    pub session_start_date: i64,
    #[ts(type = "number")]
    pub session_end_date: i64,
    #[ts(type = "number")]
    pub added_at: i64,
}

impl WatchHistory {
    pub fn new(
        video_id: String,
        channel_id: String,
        watch_duration_seconds: i64,
        session_start_date: i64,
        session_end_date: i64,
    ) -> Self {
        let Ok(added_at) = time::SystemTime::now().duration_since(time::UNIX_EPOCH) else {
            tracing::error!("Failed to get current time");
            std::process::exit(1);
        };

        Self {
            id: nanoid!(),
            video_id,
            channel_id,
            watch_duration_seconds,
            session_start_date,
            session_end_date,
            added_at: added_at.as_secs() as i64,
        }
    }
}
