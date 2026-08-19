use axum::{Router, routing::get};

use crate::{
    user,
    state::AppState,
};

pub fn router() -> Router<AppState> {
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
