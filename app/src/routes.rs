use lazy_static::lazy_static;
use rocket::fs::NamedFile;
use rocket::response::content::RawHtml;
use rocket::*;
use std::path::{Path, PathBuf};
use tera::Context;
use tera::Tera;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = tera_instance();
        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
}

fn tera_instance() -> Tera {
    match Tera::new("static/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    }
}

/* WEBPAGE routes */
#[get("/")]
pub fn home() -> RawHtml<String> {
    let context = Context::new();
    let rendered = TEMPLATES.render("index.html", &context);
    match rendered {
        Ok(c) => RawHtml(c),
        Err(e) => RawHtml(format!("Page not found. Error: {}", e)),
    }
}

#[get("/<file..>")]
pub async fn file_server(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}
