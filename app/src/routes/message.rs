use rocket::form::Form;
use rocket::response::Redirect;
use rocket::*;

#[derive(Debug, FromForm)]
struct Message {
    author: String,
    content: String,
}

#[post("/send", format = "form", data = "<data>")]
pub fn message_receive(data: Form<Message>) -> Redirect {
    dbg!(data);
    Redirect::to(uri!(crate::routes::home::home))
}
