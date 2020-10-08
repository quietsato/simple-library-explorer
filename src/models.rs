use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct Config {
    pub api_url: String,
    pub api_key: String,
    pub systemid: String,
    pub isbn: Vec<String>,
}

pub(crate) type ISBN = String;
pub(crate) type SystemId = String;

#[derive(Serialize)]
pub(crate) struct Library {
    status: String,
    reserveurl: String,
    libkey: Vec<(String, String)>,
}

#[derive(Serialize)]
pub(crate) struct Book {
    borrowInfo: Vec<(SystemId, Library)>,
}

#[derive(Serialize)]
pub(crate) struct ApiResponse {
    session: String,
    books: Vec<(ISBN, Book)>,
    r#continue: u8,
}
