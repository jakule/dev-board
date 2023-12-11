use serde_derive::Serialize;
use sqlx::FromRow;

#[derive(FromRow, Serialize, Debug)]
pub struct Pr {
    pub id: String,
    pub title: String,
    pub score: f64,
    pub opened_at: chrono::NaiveDateTime,
}
