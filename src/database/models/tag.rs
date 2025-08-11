use super::{Video, prelude::*};

#[derive(
    Queryable, Identifiable, Insertable, Serialize, Debug, Clone, utoipa::ToSchema, Deserialize, TS,
)]
#[diesel(table_name = schema::tags)]
#[diesel(check_for_backend(Sqlite))]
pub struct Tag {
    pub id: String,
    pub name: String,
    #[ts(type = "number")]
    pub added_at: i64,
}

impl Tag {
    pub fn new(name: String) -> Self {
        let Ok(added_at) = time::SystemTime::now().duration_since(time::UNIX_EPOCH) else {
            tracing::error!("Failed to get current time");
            std::process::exit(1);
        };

        Self {
            id: nanoid!(),
            name,
            added_at: added_at.as_secs() as i64,
        }
    }
}

#[derive(Queryable, Identifiable, Associations, Insertable, Serialize, Debug, Clone)]
#[diesel(table_name = schema::video_tags)]
#[diesel(primary_key(video_id, tag_id))]
#[diesel(belongs_to(Video, foreign_key = video_id))]
#[diesel(belongs_to(Tag, foreign_key = tag_id))]
#[diesel(check_for_backend(Sqlite))]
pub struct VideoTags {
    pub video_id: String,
    pub tag_id: String,
}

impl VideoTags {
    pub fn new(video_id: String, tag_id: String) -> Self {
        Self { video_id, tag_id }
    }
}
