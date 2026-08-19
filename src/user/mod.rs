mod handler;
mod model;

use axum::{Router, routing::get};
use crate::state::AppState;
pub use model::User;

pub fn router() -> Router<AppState> {
    Router::new().route("/users", get(handler::users).post(handler::add_user))
        .route("/users/{id}", get(handler::get_user))
}
