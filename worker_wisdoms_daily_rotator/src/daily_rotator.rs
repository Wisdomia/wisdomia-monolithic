use std::{future::Future, pin::Pin, time::Duration};

use sqlx::PgPool;

use crate::db_connect::{get_current_index, get_wisdom_by_id, update_current_index, update_selected_wisdom, Result};

pub async fn worker_thread(pool: PgPool) {
    loop {
        if let Err(e) = rotate_wisdom(&pool).await {
            eprintln!("Failed to rotate wisdom: {:?}", e);
            //TODO: Send TG Message?
            break;
        }
        tokio::time::sleep(Duration::from_secs(5)).await; // Run daily (86400)
    }
}

async fn rotate_wisdom(pool: &PgPool) -> Result<()> {
    let current_index = get_current_index(pool).await?;

    match get_wisdom_by_id(pool, current_index).await? {
        Some(wisdom) => {
            update_selected_wisdom(pool, &wisdom.description).await?;
            update_current_index(pool, current_index + 1).await?;
        },
        None => {
            if let Some(wisdom) = get_wisdom_by_id(pool, 1).await? {
                update_selected_wisdom(pool, &wisdom.description).await?;
            }
            update_current_index(pool, 1).await?;
        }
    }
    Ok(())
}