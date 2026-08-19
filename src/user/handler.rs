use axum::Json;
use axum::extract::Path;
use tracing::info;

use super::model::User;
use crate::state::AppState;
use axum::extract::State;

pub async fn users(
    State(state): State<AppState>,
) -> Json<Vec<User>> {
    let users = state.users.read().unwrap();

    Json(users.values().cloned().collect())
}

pub async fn add_user(State(state): State<AppState>, Json(user): Json<User>) {
    info!(?user, "user received");
    let mut users = state.users.write().unwrap();
    users.insert(user.id, user);
    info!(?users, "users updated");   
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<i32>
) -> Json<User> {
    let users = state.users.read().unwrap();
    info!(?users, "users received");
    let user = users.get(&id).unwrap().clone();
    Json(user)
}