use sqlx::FromRow;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, rocket::form::FromForm, ::serde::Serialize, FromRow)]
pub struct Message {
    pub author: String,
    pub content: String,
    pub time_stamp: u64,
    pub uid: Option<u64>,
}

#[derive(Debug, rocket::form::FromForm, ::serde::Serialize, FromRow)]
pub struct FormMessage {
    // Forms should not be trusted for good data.
    // Therefore, time_stamps and uids are generated server side.
    pub author: String,
    pub content: String,
}

impl From<FormMessage> for Message {
    fn from(item: FormMessage) -> Self {
        Message {
            author: item.author,
            content: item.content,
            time_stamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards.")
                .as_millis() as u64,
            uid: None,
        }
    }
}
