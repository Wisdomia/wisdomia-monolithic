use base64::{self, Engine};
use serde::Deserialize;
use sqlx::PgPool;
use std::fs;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct Wisdom {
    description: String,
}

#[derive(Debug, sqlx::FromRow)]
struct CountResult {
    count: i64,
}

pub async fn worker_thread(pool: PgPool) {
    let mut prev_base64_string = String::new();

    loop {
        tokio::time::sleep(Duration::from_secs(5)).await;

        let base64_string = fs::read_to_string("encoded-wisdoms.b64")
            .expect("Failed to read encoded-wisdoms.b64 file")
            .replace(['\n', '\r'], "");

        if base64_string == prev_base64_string {
            continue;
        }

        let decoded_bytes = base64::prelude::BASE64_STANDARD
            .decode(&base64_string)
            .expect("Failed to decode base64 string");

        let wisdoms: Vec<Wisdom> =
            serde_json::from_slice(&decoded_bytes).expect("Failed to parse JSON");

        for wisdom in wisdoms {
            let existing_wisdom = sqlx::query_as::<_, CountResult>(
                "SELECT COUNT(*) as count FROM wisdoms WHERE description = $1",
            )
            .bind(&wisdom.description)
            .fetch_one(&pool)
            .await
            .expect("Failed to check if wisdom exists");

            if existing_wisdom.count == 0 {
                sqlx::query("INSERT INTO wisdoms (description) VALUES ($1)")
                .bind(&wisdom.description)
                .execute(&pool)
                .await
                .expect("Failed to insert wisdom into database");
            } else {
                println!("Wisdom already exists, skipping: {:?}", wisdom);
            }
        }

        prev_base64_string = base64_string;
    }
}
