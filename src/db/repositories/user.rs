use bson::{doc, Document};
use chrono::Utc;
use mongodb::{bson, error::Error, options::FindOptions, results::DeleteResult, results::InsertOneResult, sync::Collection};

use crate::db::models::{User, UserStatus};

pub struct UserRepository {
    col: Collection,
}

impl UserRepository {
    pub fn new(col: Collection) -> Self {
        Self { col }
    }

    pub fn insert(&self, username: String, password: String, status: UserStatus) -> Result<InsertOneResult, Error> {
        let new_user = User {
            id: None,
            username,
            password,
            status,
            created_at: Utc::now(),
        };
        self.col.insert_one(bson::to_document(&new_user)?, None)
    }

    pub fn delete_all(&self) -> Result<DeleteResult, Error> {
        self.col.delete_many(doc! {}, None)
    }

    pub fn find(&self, username: Option<String>, status: Option<UserStatus>, limit: i64) -> Result<Vec<User>, Error> {
        let mut filter = Document::new();
        if username.is_some() {
            filter.insert("username".to_string(), username.unwrap());
        }
        if status.is_some() {
            filter.insert("status".to_string(), status.unwrap());
        }
        let options = FindOptions::builder().sort(doc! {"created_at": -1}).limit(limit).build();
        let cursor = self.col.find(filter, options)?;
        let mut result: Vec<User> = Vec::new();
        for res in cursor {
            match res {
                Ok(res) => result.push(bson::from_document::<User>(res)?),
                Err(e) => return Err(e),
            }
        }
        Ok(result)
    }
}
