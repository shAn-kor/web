use axum::Json;
use axum::extract::Path;
use tracing::info;

use super::model::User;
use crate::state::AppState;
use axum::extract::State;

pub async fn users() -> Json<Vec<User>> {
    Json(vec![
        User {
            id: 1,
            name: "John".into(),
        }
    ])
}

pub async fn add_user(State(state): State<AppState>, Json(user): Json<User>) {
    info!(?user, "user received");
    let mut users = state.users.write().unwrap();
    users.insert(user.id, user);
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