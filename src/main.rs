use db::{models::UserStatus, DB};
use mongodb::error::Error;

mod db;

fn main() -> Result<(), Error> {
    let db = DB::new(String::from("mongodb://localhost/demo-rust-mongodb"))?;
    db.user.delete_all()?;
    db.post.delete_all()?;

    let usr1_id = db
        .user
        .insert(String::from("usr1"), String::from("pass"), UserStatus::MODERATOR)?
        .inserted_id
        .as_object_id()
        .unwrap()
        .to_owned();

    let usr2_id = db
        .user
        .insert(String::from("usr2"), String::from("pass"), UserStatus::USER)?
        .inserted_id
        .as_object_id()
        .unwrap()
        .to_owned();

    let moderators = db.user.find(None, Some(UserStatus::MODERATOR), 100)?;
    println!("moderators: {:#?}", moderators);

    let post1_id = db
        .post
        .insert_post(usr1_id.clone(), "post 1 title".to_string(), "post 1 body".to_string())?
        .inserted_id
        .as_object_id()
        .unwrap()
        .to_owned();

    db.post.insert_comment(post1_id.clone(), usr1_id.clone(), "comment 1".to_string())?;
    db.post.insert_comment(post1_id.clone(), usr2_id.clone(), "comment 2".to_string())?;

    let all_posts = db.post.find_all();
    println!("posts: {:#?}", all_posts);

    Ok(())
}
