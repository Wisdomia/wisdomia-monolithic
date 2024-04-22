mod constants;
mod state;
mod db;
mod routes;
mod models;
mod middlewares;
mod helpers;

use state::AppState;
use std::{env, net::SocketAddr, sync::Arc};
use db::connect;

use axum::{middleware, Extension, Router};
use tokio::net::TcpListener;
use middlewares::response_mapper;

#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| panic!("Missing required environment variable: {}","DATABASE_URL"));
   
    let tfd = tracing_fast_dev::tfd();

    tfd.info("wisdomia", "INITIALIZATION");

    let db = connect(database_url.as_str()).await.unwrap();

    sqlx::migrate!("./migrations").run(&db).await.unwrap();

    let state = AppState { db };

    let shared_state = Arc::new(state);

    let listener = TcpListener::bind(format!("{}:{}", constants::HOST, constants::PORT))
        .await
        .unwrap();

    println!("Wisdoms server starting... PORT: {}", constants::PORT);

    let router = Router::new()
        .nest("/api/v1", routes::routes())
        .layer(Extension(shared_state))
        .layer(middleware::from_fn(middlewares::rate_limit))
        .layer(middleware::map_response(response_mapper));

    axum::serve(listener, router.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}
