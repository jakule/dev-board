use salvo::oapi::ToSchema;
use serde_derive::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, ToSchema, Serialize, Default)]
pub struct PrResponse {
    pub id: String,
    pub title: String,
    pub score: f64,
    pub opened_at: DateTime<Utc>,
    pub should_close_at: DateTime<Utc>,
}
