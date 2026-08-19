use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::user::model::User;

#[derive(Clone)]
pub struct AppState {
    pub users: Arc<RwLock<HashMap<i32, User>>>,
}