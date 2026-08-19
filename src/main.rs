mod routes;
mod user;
mod state;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tracing::info;
use crate::state::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "web=debug".into()),
        )
        .init();

    let state = AppState {
        users: Arc::new(RwLock::new(HashMap::new())),
    };

    let app = routes::router().with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed to bind TCP listener");

    info!(address = %listener.local_addr().unwrap(), "server listening");

    axum::serve(listener, app).await.expect("server error");
}
