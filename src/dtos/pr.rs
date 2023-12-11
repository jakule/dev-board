use chrono::{DateTime, Utc};
use salvo::oapi::ToSchema;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, ToSchema, Serialize, Default)]
pub struct PrResponse {
    pub id: String,
    pub title: String,
    pub score: f64,
    pub opened_at: DateTime<Utc>,
    pub should_close_at: DateTime<Utc>,
}
