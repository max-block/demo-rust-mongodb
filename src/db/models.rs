use chrono::{DateTime, Utc};
use mongodb::bson::{oid::ObjectId, Bson};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub password: String,
    pub status: UserStatus,
    pub created_at: DateTime<Utc>,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum UserStatus {
    USER,
    MODERATOR,
    BANNED,
}

impl Into<Bson> for UserStatus {
    fn into(self) -> Bson {
        match self {
            UserStatus::USER => Bson::String("USER".to_string()),
            UserStatus::MODERATOR => Bson::String("MODERATOR".to_string()),
            UserStatus::BANNED => Bson::String("BANNED".to_string()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user: ObjectId,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub comments: Vec<Comment>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Comment {
    pub user: ObjectId,
    pub message: String,
    pub created_at: DateTime<Utc>,
}
