use axum::{Router, routing::get};

use crate::user;

pub fn router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .merge(user::router())
}

async fn root() -> &'static str {
    "Hello, Axum!"
}

async fn health() -> &'static str {
    "ok"
}
