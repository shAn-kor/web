mod handler;
mod model;

use axum::{Router, routing::get};

pub fn router() -> Router {
    Router::new().route("/users", get(handler::get_users).post(handler::add_user))
        .route("/users/{id}", get(handler::get_user))
}
