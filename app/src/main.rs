use rocket::*;
mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![routes::home::home, routes::message::message_receive],
        )
        .mount("/static", routes![routes::file::file_server])
}
