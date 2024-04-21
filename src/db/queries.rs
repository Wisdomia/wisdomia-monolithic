use crate::models::Wisdom;
use super::connect::Database;

pub async fn _get_wisdoms(db: &Database) -> Result<Vec<Wisdom>, sqlx::Error> {
    tracing_fast_dev::tfd().info("GET_WISDOMS_INTERNAL", "QUERY");
    let wisdoms: Vec<Wisdom> = sqlx::query_as("SELECT * from wisdoms")
        .fetch_all(db)
        .await?;
    Ok(wisdoms)
}