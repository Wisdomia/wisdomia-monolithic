use base64::{self, Engine};
use serde::Deserialize;
use std::fs;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct Wisdom {
    description: String,
}

pub async fn worker_thread() {
    loop {
        let base64_string = fs::read_to_string("encoded-wisdoms.b64")
            .expect("Failed to read encoded-wisdoms.b64 file")
            .replace(['\n','\r'], "");

        dbg!(&base64_string);

        let decoded_bytes = base64::prelude::BASE64_STANDARD.decode(base64_string)
            .expect("Failed to decode base64 string");

        let wisdoms: Vec<Wisdom> =
            serde_json::from_slice(&decoded_bytes).expect("Failed to parse JSON");

        for wisdom in wisdoms {
            println!("Decoded wisdom: {:?}", wisdom);
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
