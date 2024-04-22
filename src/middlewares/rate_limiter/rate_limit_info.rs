use redis_macros::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,FromRedisValue,ToRedisArgs,PartialEq,Eq,Debug)]
pub struct RateLimitInfo {
    pub limit: i32,
}