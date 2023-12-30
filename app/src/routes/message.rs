use rocket::form::Form;
use rocket::response::Redirect;
use rocket::*;
use sqlx::SqlitePool;

use crate::models::message::{FormMessage, Message};

#[post("/send", format = "form", data = "<data>")]
pub async fn message_receive(data: Form<FormMessage>, db: &rocket::State<SqlitePool>) -> Redirect {
    let insert_data = Message::from(data.into_inner());

    match sqlx::query("INSERT INTO messages (author, content, time_stamp) VALUES (?, ?, ?)")
        .bind(insert_data.author)
        .bind(insert_data.content)
        .bind(insert_data.time_stamp as i64)
        .execute(&**db)
        .await
    {
        Ok(r) => println!("OK: {:?}", r),
        Err(e) => println!("Could not be inserted: {}", e),
    }
    Redirect::to(uri!(crate::routes::home::home))
}
