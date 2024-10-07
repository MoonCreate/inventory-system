use std::sync::Mutex;

use crate::structs::User;

#[derive(Debug)]
pub struct AppState {
    pub users: Mutex<Vec<User>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            users: Mutex::new(vec![]),
        }
    }
}
