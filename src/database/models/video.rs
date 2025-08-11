use super::{Channel, prelude::*};

pub struct NewVideoParams {
    pub id: String,
    pub channel_id: String,
    pub title: String,
    pub description: String,
    pub duration_seconds: i64,
    pub likes_count: i64,
    pub view_count: i64,
    pub comments_count: i64,
    pub published_at: i64,
}

#[derive(
    Queryable,
    Identifiable,
    Associations,
    Insertable,
    Serialize,
    Debug,
    Clone,
    utoipa::ToSchema,
    Deserialize,
    TS,
)]
#[diesel(table_name = schema::videos)]
#[diesel(belongs_to(Channel, foreign_key = channel_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Video {
    pub id: String,
    #[serde(skip)]
    pub channel_id: String,
    pub url: String,
    pub title: String,
    pub description: String,
    #[ts(type = "number")]
    pub watch_counter: i64,
    #[ts(type = "number")]
    pub duration_seconds: i64,
    #[ts(type = "number")]
    pub likes_count: i64,
    #[ts(type = "number")]
    pub view_count: i64,
    #[ts(type = "number")]
    pub comments_count: i64,
    #[ts(type = "number")]
    pub published_at: i64,
    #[ts(type = "number")]
    pub added_at: i64,
}

impl Video {
    pub fn new(p: NewVideoParams) -> Self {
        let Ok(added_at) = time::SystemTime::now().duration_since(time::UNIX_EPOCH) else {
            tracing::error!("Failed to get current time");
            std::process::exit(1);
        };

        let url = format!("https://www.youtube.com/watch?v={}", p.id);

        Self {
            id: p.id,
            channel_id: p.channel_id,
            url,
            title: p.title,
            description: p.description,
            watch_counter: 0,
            duration_seconds: p.duration_seconds,
            likes_count: p.likes_count,
            view_count: p.view_count,
            comments_count: p.comments_count,
            published_at: p.published_at,
            added_at: added_at.as_secs() as i64,
        }
    }
}
