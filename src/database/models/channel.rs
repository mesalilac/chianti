use super::prelude::*;

pub struct NewChannelParams {
    pub id: String,
    pub name: String,
    pub url: String,
    pub is_subscribed: bool,
    pub subscribers_count: i64,
}

#[derive(
    Queryable, Identifiable, Insertable, Serialize, Debug, Clone, utoipa::ToSchema, Deserialize, TS,
)]
#[diesel(table_name = schema::channels)]
#[diesel(check_for_backend(Sqlite))]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub url: String,
    pub is_subscribed: bool,
    #[ts(type = "number")]
    pub subscribers_count: i64,
    #[ts(type = "number")]
    pub added_at: i64,
}

impl Channel {
    pub fn new(p: NewChannelParams) -> Self {
        let Ok(added_at) = time::SystemTime::now().duration_since(time::UNIX_EPOCH) else {
            tracing::error!("Failed to get current time");
            std::process::exit(1);
        };

        Self {
            id: p.id,
            name: p.name,
            url: p.url,
            is_subscribed: p.is_subscribed,
            subscribers_count: p.subscribers_count,
            added_at: added_at.as_secs() as i64,
        }
    }
}
