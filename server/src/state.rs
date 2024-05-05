use crate::{db::Database, RateLimiterConfig, RedisRateLimiterDb};

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub redis_rate_limiter_db: RedisRateLimiterDb,
    pub rate_limiter_config: RateLimiterConfig,
}
