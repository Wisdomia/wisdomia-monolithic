use std::time::Duration;

pub trait RateLimiter {
    fn new(requests_amount: u8, limit: Duration) -> Self;
    fn set_requests_amount(&mut self, requests_amount: u8);
    fn set_limit(&mut self, limit: Duration);
}

#[derive(Clone)]
pub struct RateLimiterConfig {
    pub requests_amount: u8,
    pub time_frame: Duration,
}

impl RateLimiter for RateLimiterConfig {
    fn new(requests_amount: u8, limit: Duration) -> Self {
        Self {
            requests_amount,
            time_frame: limit,
        }
    }

    fn set_requests_amount(&mut self, requests_amount: u8) {
        self.requests_amount = requests_amount;
    }

    fn set_limit(&mut self, limit: Duration) {
        self.time_frame = limit;
    }
}
