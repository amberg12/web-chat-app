use rocket::*;
mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![routes::home])
        .mount("/static", routes![routes::file_server])
}
