use mongodb::sync::Collection;

use crate::db::models::UserStatus;

pub struct UserRepository {
    col: Collection,
}

impl UserRepository {
    pub fn new(col: Collection) -> Self {
        Self { col }
    }

    pub fn insert(&self, username: String, password: String, status: UserStatus) {}
}
