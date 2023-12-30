use rocket::form::Form;
use rocket::response::content::RawHtml;
use rocket::response::Redirect;
use rocket::*;
use sqlx::SqlitePool;
use tera::Context;

use crate::models::message::{FormMessage, Message};
use crate::templates::templates::TEMPLATES;

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

#[get("/event")]
pub async fn message_send(db: &rocket::State<SqlitePool>) -> RawHtml<String> {
    let mut context = Context::new();
    context.insert(
        "messages",
        &match sqlx::query_as::<_, Message>("SELECT * FROM messages")
            .fetch_all(db.inner())
            .await
        {
            Ok(r) => r,
            Err(e) => vec![Message {
                author: "Error".to_string(),
                content: e.to_string(),
                uid: None,
                time_stamp: 0,
            }],
        },
    );
    let rendered = TEMPLATES.render("messages.html", &context);
    match rendered {
        Ok(c) => RawHtml(c),
        Err(e) => RawHtml(format!("Page not found. Error: {}", e)),
    }
}
