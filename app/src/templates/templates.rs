use lazy_static::lazy_static;
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
