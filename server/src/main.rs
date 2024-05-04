mod constants;
mod db;
mod helpers;
mod middlewares;
mod models;
mod routes;
mod state;

use db::connect;
use state::AppState;
use std::{env, net::SocketAddr, sync::Arc};

use axum::{middleware, Extension, Router};
use middlewares::response_mapper;
use tokio::net::TcpListener;


#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| panic!("Missing required environment variable: {}", "DATABASE_URL"));

    let tfd = tracing_fast_dev::tfd();

    tfd.info("wisdomia", "INITIALIZATION");

    let db = connect(database_url.as_str()).await.unwrap();

    sqlx::migrate!("../migrations").run(&db).await.unwrap();

    let state = AppState { db: db.clone() };

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

    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
