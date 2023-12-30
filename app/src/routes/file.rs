use rocket::fs::NamedFile;
use rocket::*;
use std::path::{Path, PathBuf};

#[get("/<file..>")]
pub async fn file_server(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}
