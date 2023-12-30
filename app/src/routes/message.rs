use rocket::form::Form;
use rocket::response::Redirect;
use rocket::*;

use crate::models::message::Message;

#[post("/send", format = "form", data = "<data>")]
pub fn message_receive(data: Form<Message>) -> Redirect {
    dbg!(data);
    Redirect::to(uri!(crate::routes::home::home))
}
