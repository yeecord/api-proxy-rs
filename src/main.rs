pub mod db;
mod handlers;
pub mod hash;

use axum::{
  routing::{get, post},
  serve, Router,
};
use db::DB;
use handlers::{api::api_handler, health::health_handler, invalidate::invalidate_handler};
use tokio::net::TcpListener;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

const BIND_ADDRESS: &str = "0.0.0.0:80";

#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[tokio::main]
async fn main() {
  tracing_subscriber::registry()
    .with(LevelFilter::DEBUG)
    .with(fmt::layer())
    .init();

  DB.seed().await;

  let app = Router::new()
    .route("/", get(health_handler))
    .route("/api/*path", get(api_handler))
    .route("/invalidate", post(invalidate_handler));

  info!("listening on {}", BIND_ADDRESS);

  let listener = TcpListener::bind(BIND_ADDRESS).await.unwrap();
  serve(listener, app).await.unwrap();
}
