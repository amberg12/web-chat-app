use rocket::*;

pub mod database;
pub mod models;
pub mod routes;

#[launch]
async fn rocket() -> _ {
    let db = match database::init::init_db().await {
        Ok(r) => r,
        Err(e) => panic!("error: {}", e),
    };

    match database::init::create_tables(&db).await {
        Ok(_) => (),
        Err(e) => panic!("error: {}", e),
    }

    rocket::build()
        .mount(
            "/",
            routes![routes::home::home, routes::message::message_receive],
        )
        .mount("/static", routes![routes::file::file_server])
}
