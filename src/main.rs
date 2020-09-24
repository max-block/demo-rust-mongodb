#![allow(dead_code, unused_variables, unused_imports)]

use db::{models::UserStatus, DB};

mod db;

fn main() {
    let db = DB::new(String::from("mongodb://localhost/demo-rust-mongodb")).unwrap();
    db.user.insert(String::from("user1"), String::from("pass"), UserStatus::USER);
}
