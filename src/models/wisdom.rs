use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct Wisdom {
    pub id: i32,
    pub description: String,
}
