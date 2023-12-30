use crate::models::message::Message;
use lazy_static::lazy_static;
use rocket::response::content::RawHtml;
use rocket::*;
use tera::Context;
use tera::Tera;

fn tera_instance() -> Tera {
    match Tera::new("static/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    }
}

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let tera = tera_instance();
        tera
    };
}

/* WEBPAGE routes */
#[get("/")]
pub fn home() -> RawHtml<String> {
    let mut context = Context::new();
    context.insert(
        "messages",
        &vec![
            Message {
                author: "a".to_string(),
                content: "hi".to_string(),
                time_stamp: 0,
                uid: Some(2),
            },
            Message {
                author: "b".to_string(),
                content: "hi".to_string(),
                time_stamp: 0,
                uid: Some(2),
            },
            Message {
                author: "c".to_string(),
                content: "hi".to_string(),
                time_stamp: 0,
                uid: Some(2),
            },
        ],
    );
    let rendered = TEMPLATES.render("index.html", &context);
    match rendered {
        Ok(c) => RawHtml(c),
        Err(e) => RawHtml(format!("Page not found. Error: {}", e)),
    }
}
