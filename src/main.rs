use axum::{Router, routing::get};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "web=debug,tower_http=debug".into()),
        )
        .init();

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed to bind TCP listener");

    info!(address = %listener.local_addr().unwrap(), "server listening");

    axum::serve(listener, app)
        .await
        .expect("server error");
}

async fn root() -> &'static str {
    "Hello, Axum!"
}

async fn health() -> &'static str {
    "ok"
}
