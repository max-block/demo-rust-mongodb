use db::{models::UserStatus, DB};
use mongodb::error::Error;

mod db;

fn main() -> Result<(), Error> {
    let db = DB::new(String::from("mongodb://localhost/demo-rust-mongodb"))?;

    db.user.delete_all()?;
    db.user.insert(String::from("usr1"), String::from("pass"), UserStatus::MODERATOR)?;
    db.user.insert(String::from("usr2"), String::from("pass"), UserStatus::USER)?;

    let moderators = db.user.find(None, Some(UserStatus::MODERATOR), 100)?;
    println!("moderators: {:?}", moderators);

    Ok(())
}
