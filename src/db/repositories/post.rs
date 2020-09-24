use chrono::Utc;
use mongodb::{
    bson::doc,
    bson::{self, oid::ObjectId},
    error::Error,
    results::DeleteResult,
    results::InsertOneResult,
    results::UpdateResult,
    sync::Collection,
};

use crate::db::models::{Comment, Post};

pub struct PostRepository {
    col: Collection,
}

impl PostRepository {
    pub fn new(col: Collection) -> Self {
        Self { col }
    }

    pub fn delete_all(&self) -> Result<DeleteResult, Error> {
        self.col.delete_many(doc! {}, None)
    }

    pub fn insert_post(&self, user: ObjectId, title: String, body: String) -> Result<InsertOneResult, Error> {
        let new_post = Post {
            id: None,
            user,
            title,
            body,
            created_at: Utc::now(),
            comments: vec![],
        };
        self.col.insert_one(bson::to_document(&new_post)?, None)
    }

    pub fn insert_comment(&self, post: ObjectId, user: ObjectId, message: String) -> Result<UpdateResult, Error> {
        let new_comment = Comment {
            user,
            message,
            created_at: Utc::now(),
        };
        self.col
            .update_one(doc! {"_id": post}, doc! {"$push": {"comments": bson::to_document(&new_comment)?}}, None)
    }

    pub fn find_all(&self) -> Result<Vec<Post>, Error> {
        let cursor = self.col.find(doc! {}, None)?;
        let mut result: Vec<Post> = Vec::new();
        for res in cursor {
            match res {
                Ok(res) => result.push(bson::from_document::<Post>(res)?),
                Err(e) => return Err(e),
            }
        }
        Ok(result)
    }
}
