use rocket::*;
use rocket::response::content::RawHtml;

use tera::Tera;
use tera::Context;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = tera_instance();
        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
}

fn tera_instance() -> Tera {
    match Tera::new("static/*.html.tera") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    }
}

#[get("/")]
pub fn home() -> RawHtml<String> {
    let context = Context::new();
    let rendered = TEMPLATES.render("index.html.tera", &context);
    
    match rendered {
        Ok(file_content) => RawHtml(file_content),
        Err(_) => RawHtml("Page not found.".to_string()),
    }
}
