use crate::models::message::Message;
use crate::templates::templates::TEMPLATES;
use rocket::response::content::RawHtml;
use rocket::*;
use sqlx::SqlitePool;
use tera::Context;

/* WEBPAGE routes */
#[get("/")]
pub async fn home(db: &rocket::State<SqlitePool>) -> RawHtml<String> {
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
    let rendered = TEMPLATES.render("index.html", &context);
    match rendered {
        Ok(c) => RawHtml(c),
        Err(e) => RawHtml(format!("Page not found. Error: {}", e)),
    }
}
