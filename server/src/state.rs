use crate::{db::Database, RedisRateLimiterDb};

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub redis_rate_limiter_db: RedisRateLimiterDb,
}
