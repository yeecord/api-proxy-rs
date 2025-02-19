pub mod db;
mod handlers;
pub mod hash;

use axum::{routing::any, serve, Router};
use db::DB;
use handlers::{health::health_handler, proxy::proxy_handler};
use tokio::{
  net::TcpListener,
  select,
  signal::unix::{signal, SignalKind},
};
use tracing::{debug, info, level_filters::LevelFilter};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

const BIND_ADDRESS: &str = "0.0.0.0:80";

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() {
  tracing_subscriber::registry()
    .with(LevelFilter::DEBUG)
    .with(fmt::layer())
    .init();

  DB.seed().await;

  let app = Router::new()
    .route("/", any(health_handler))
    .route("/{*path}", any(proxy_handler));

  info!("listening on {}", BIND_ADDRESS);

  let listener = TcpListener::bind(BIND_ADDRESS).await.unwrap();

  serve(listener, app)
    .with_graceful_shutdown(shutdown_signal())
    .await
    .unwrap();
}

async fn shutdown_signal() {
  // https://www.gnu.org/software/libc/manual/html_node/Termination-Signals.html
  let mut signal_terminate = signal(SignalKind::terminate()).unwrap();
  let mut signal_interrupt = signal(SignalKind::interrupt()).unwrap();

  select! {
    _ = signal_terminate.recv() => debug!("Received SIGTERM."),
    _ = signal_interrupt.recv() => debug!("Received SIGINT."),
  };
}
