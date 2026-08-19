use axum::Json;
use axum::extract::Path;
use tracing::info;

use super::model::User;

pub async fn get_users() -> Json<User> {
    Json(User {
        id: 1,
        name: "John".into(),
    })
}

pub async fn add_user(Json(user): Json<User>) -> Json<User> {
    info!(?user, "user received");

    Json(User {
        id: 2,
        name: user.name,
    })
}

pub async fn get_user(Path(id): Path<i32>) -> Json<User> {
    Json(User {
        id,
        name: "John".into(),
    })
}