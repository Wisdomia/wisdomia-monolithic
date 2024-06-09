mod daily_rotator;
mod db_connect;

use std::env;

use daily_rotator::worker_thread;
use db_connect::{connect, initialize_db};

#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| panic!("Missing required environment variable: {}", "DATABASE_URL"));

    let db = connect(database_url.as_str()).await.unwrap();
    
    sqlx::migrate!("../migrations").run(&db).await.unwrap();

    initialize_db(&db).await.unwrap();

    tokio::spawn(worker_thread(db)).await.unwrap();
}
