mod db_connect;
mod wisdoms_checker;

use std::env;

use db_connect::connect;
use wisdoms_checker::worker_thread;

#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| panic!("Missing required environment variable: {}", "DATABASE_URL"));

    let db = connect(database_url.as_str()).await.unwrap();

    tokio::spawn(worker_thread(db)).await.unwrap();
}
