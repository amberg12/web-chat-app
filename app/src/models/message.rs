#[derive(Debug, rocket::form::FromForm, ::serde::Serialize)]
pub struct Message {
    pub author: String,
    pub content: String,
}
