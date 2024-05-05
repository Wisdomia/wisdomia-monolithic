use std::time::Duration;

use sqlx::PgPool;

pub async fn worker_thread(_pool: PgPool) {
    loop {
        dbg!("Rotating...");
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
