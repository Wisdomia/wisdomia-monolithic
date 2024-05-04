use base64::{self, Engine};
use dotenv::dotenv;
use serde::Deserialize;
use sqlx::PgPool;
use std::time::Duration;
use std::{env, fs};
use teloxide::{prelude::*, types::Recipient, RequestError};

#[derive(Debug, Deserialize)]
struct Wisdom {
    description: String,
}

#[derive(Debug, sqlx::FromRow)]
struct CountResult {
    count: i64,
}

async fn send_tg_message(bot: Bot, message: &str, chat_id: i64) -> Result<Message, RequestError> {
    bot.send_message(Recipient::Id(ChatId(chat_id)), message)
        .await
}

//TODO: This code should BE fixed to not use so many match-es everywhere.
pub async fn worker_thread(pool: PgPool) {
    let _ = dotenv().ok();
    let mut prev_base64_string = String::new();
    let bot = Bot::from_env();

    let chat_id = env::var("WISDOMIA_MONOLITHIC_BOT_CHAT_ID").unwrap_or_else(|_| {
        panic!("WISDOMIA_MONOLITHIC_BOT_CHAT_ID not found in .env");
    });
    let chat_id = chat_id.parse::<i64>().unwrap_or_else(|_| {
        panic!("Unable to parse chat_id to i64 type.");
    });

    loop {
        tokio::time::sleep(Duration::from_secs(5)).await;

        let base64_string = match fs::read_to_string("encoded-wisdoms.b64") {
            Ok(content) => content.replace(['\n', '\r'], ""),
            Err(e) => {
                send_tg_message(
                    bot,
                    format!("Failed to read file encoded-wisdoms.b64: {}", e).as_str(),
                    chat_id,
                )
                .await
                .unwrap();
                return;
            }
        };

        if base64_string == prev_base64_string {
            continue;
        }

        let decoded_bytes = match base64::prelude::BASE64_STANDARD.decode(&base64_string) {
            Ok(bytes) => bytes,
            Err(e) => {
                send_tg_message(
                    bot,
                    format!("Failed to decode base64 string: {}", e).as_str(),
                    chat_id,
                )
                .await
                .unwrap();
                return;
            }
        };

        let wisdoms: Vec<Wisdom> = match serde_json::from_slice(&decoded_bytes) {
            Ok(parsed) => parsed,
            Err(e) => {
                send_tg_message(
                    bot,
                    format!("Failed to parse JSON: {}", e).as_str(),
                    chat_id,
                )
                .await
                .unwrap();
                return;
            }
        };

        for wisdom in wisdoms {
            let existing_wisdom = match sqlx::query_as::<_, CountResult>(
                "SELECT COUNT(*) as count FROM wisdoms WHERE description = $1",
            )
            .bind(&wisdom.description)
            .fetch_one(&pool)
            .await
            {
                Ok(result) => result,
                Err(e) => {
                    send_tg_message(
                        bot,
                        format!("Failed to check if wisdom exists: {}", e).as_str(),
                        chat_id,
                    )
                    .await
                    .unwrap();
                    return;
                }
            };

            if existing_wisdom.count == 0 {
                match sqlx::query("INSERT INTO wisdoms (description) VALUES ($1)")
                    .bind(&wisdom.description)
                    .execute(&pool)
                    .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        send_tg_message(
                            bot,
                            format!("Failed to insert wisdom into database: {}", e).as_str(),
                            chat_id,
                        )
                        .await
                        .unwrap();
                        return;
                    }
                };
            } else {
                println!("Wisdom already exists, skipping: {:?}", wisdom);
            }
        }

        prev_base64_string = base64_string;
    }
}
