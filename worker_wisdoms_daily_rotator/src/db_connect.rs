pub type Result<T> = core::result::Result<T, Error>;

use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};

pub type Database = sqlx::Pool<sqlx::Postgres>;

use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct Wisdom {
    pub id: i32,
    pub description: String,
}


#[derive(Debug)]
pub enum Error {
    FailedConnectingToDatabase { error: String },
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Self::FailedConnectingToDatabase {
            error: value.to_string(),
        }
    }
}

pub async fn connect(db_url: &str) -> Result<Database> {
    let pool: Database = PgPoolOptions::new()
        // TODO: Max connections! Dynamic?
        .max_connections(5)
        .connect(db_url)
        .await?;
    Ok(pool)
}

pub async fn get_current_index(pool: &PgPool) -> Result<i32> {
    let row = sqlx::query("SELECT current_index FROM current_wisdom WHERE id = 1")
        .fetch_one(pool)
        .await?;

    Ok(row.try_get("current_index")?)
}

pub async fn update_current_index(pool: &PgPool, index: i32) -> Result<()> {
    sqlx::query("UPDATE current_wisdom SET current_index = $1 WHERE id = 1")
        .bind(index)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_wisdom_by_id(pool: &PgPool, id: i32) -> Result<Option<Wisdom>> {
    let row = sqlx::query_as::<_, Wisdom>("SELECT id, description FROM wisdoms WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok(row)
}

pub async fn update_selected_wisdom(pool: &PgPool, wisdom: &str) -> Result<()> {
    sqlx::query("UPDATE current_wisdom SET selected_wisdom = $1 WHERE id = 1")
        .bind(wisdom)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn initialize_db(pool: &PgPool) -> Result<()> {
    sqlx::query("INSERT INTO current_wisdom (id, current_index, selected_wisdom) VALUES (1, 1, NULL) ON CONFLICT DO NOTHING")
        .execute(pool)
        .await?;
    Ok(())
}