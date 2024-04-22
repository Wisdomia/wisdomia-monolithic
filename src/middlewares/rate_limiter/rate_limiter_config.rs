use std::time::Duration;


pub trait RateLimiter {
    fn new(requests_amount:u32,limit:Duration)->Self;
    fn set_requests_amount(&mut self,requests_amount:u32);
    fn set_limit(&mut self, limit:Duration);
}

pub struct RateLimiterConfig {
    pub requests_amount:u32,
    pub limit:Duration,
}

impl RateLimiter for RateLimiterConfig {
    fn new(requests_amount:u32,limit:Duration) -> Self {
        Self {
            requests_amount,
            limit
        }
    }
    
    fn set_requests_amount(&mut self,requests_amount:u32) {
        self.requests_amount=requests_amount;
    }
    
    fn set_limit(&mut self, limit:Duration) {
        self.limit=limit;
    }

}