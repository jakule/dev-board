use salvo::oapi::ToSchema;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, ToSchema, Serialize, Default)]
pub struct PrResponse {
    pub id: String,
    pub title: String,
}