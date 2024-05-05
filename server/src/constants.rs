use std::time::Duration;

pub const HOST: &str = "0.0.0.0";
pub const PORT: &str = "6565";
pub const REQUESTS_AMOUNT_LIMIT: u8 = 10;
pub const REQUESTS_AMOUNT_TIME_FRAME: Duration = Duration::from_secs(30);
