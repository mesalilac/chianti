use crate::schema;
use diesel::prelude::*;
use nanoid::nanoid;
use serde::Serialize;
use std::time;

#[derive(Queryable, Identifiable, Insertable, Serialize, Debug, Clone)]
#[diesel(table_name = schema::channels)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub url: String,
    pub subscribers_count: i64,
    pub added_at: i64,
}

impl Channel {
    pub fn new(id: String, name: String, subscribers_count: i64) -> Self {
        let Ok(added_at) = time::SystemTime::now().duration_since(time::UNIX_EPOCH) else {
            tracing::error!("Failed to get current time");
            std::process::exit(1);
        };

        let url = format!("https://www.youtube.com/channel/{id}");

        Self {
            id,
            name,
            url,
            subscribers_count,
            added_at: added_at.as_secs() as i64,
        }
    }
}

#[derive(Queryable, Identifiable, Associations, Insertable, Serialize, Debug, Clone)]
#[diesel(table_name = schema::videos)]
#[diesel(belongs_to(Channel, foreign_key = channel_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Video {
    pub id: String,
    pub channel_id: String,
    pub url: String,
    pub title: String,
    pub watch_counter: i64,
    pub duration_seconds: i64,
    pub view_count: i64,
    pub published_at: i64,
    pub added_at: i64,
}

impl Video {
    pub fn new(
        id: String,
        channel_id: String,
        title: String,
        duration_seconds: i64,
        view_count: i64,
        published_at: i64,
    ) -> Self {
        let Ok(added_at) = time::SystemTime::now().duration_since(time::UNIX_EPOCH) else {
            tracing::error!("Failed to get current time");
            std::process::exit(1);
        };

        let url = format!("https://www.youtube.com/watch?v={id}");

        Self {
            id,
            channel_id,
            url,
            title,
            watch_counter: 0,
            duration_seconds,
            view_count,
            published_at,
            added_at: added_at.as_secs() as i64,
        }
    }
}

#[derive(Queryable, Identifiable, Insertable, Serialize, Debug, Clone)]
#[diesel(table_name = schema::tags)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Tag {
    pub id: String,
    pub name: String,
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
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct VideoTags {
    pub video_id: String,
    pub tag_id: String,
}

impl VideoTags {
    pub fn new(video_id: String, tag_id: String) -> Self {
        Self { video_id, tag_id }
    }
}

#[derive(Queryable, Identifiable, Associations, Insertable, Serialize, Debug, Clone)]
#[diesel(table_name = schema::watch_history)]
#[diesel(belongs_to(Video, foreign_key = video_id))]
#[diesel(belongs_to(Channel, foreign_key = channel_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct WatchHistory {
    pub id: String,
    pub video_id: String,
    pub channel_id: String,
    pub watch_duration_seconds: i64,
    pub session_start_date: i64,
    pub session_end_date: i64,
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
