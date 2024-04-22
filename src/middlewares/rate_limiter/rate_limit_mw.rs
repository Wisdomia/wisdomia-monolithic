use std::net::SocketAddr;

use axum::{extract::{ConnectInfo, Request}, middleware::Next, response::Response};


pub async fn rate_limit(
    ConnectInfo(ip_addr): ConnectInfo<SocketAddr>,
    mut req: Request,
    next: Next,
)-> Response {
    println!("Rate limiter hit with ip: {}",ip_addr);
    next.run(req).await
}