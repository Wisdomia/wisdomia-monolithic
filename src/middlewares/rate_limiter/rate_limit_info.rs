use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RateLimitInfo {
    pub limit: i32,
}