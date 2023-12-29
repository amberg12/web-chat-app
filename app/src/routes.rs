use std::fs;

use rocket::*;
use rocket::response::content::RawHtml;

#[get("/")]
pub fn world() -> RawHtml<String> {
    let contents = fs::read_to_string("static/index.html");
    match contents {
        Ok(file_content) => RawHtml(file_content),
        Err(_) => RawHtml("Page not found.".to_string()),
    }
}
