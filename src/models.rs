use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct Config {
    pub api_url: String,
    pub api_key: String,
    pub systemid: String,
    pub isbn: Vec<String>,
}

pub(crate) type ISBN = String;
pub(crate) type Library = (String, String);
pub(crate) type Books = Vec<Book>;

#[derive(Debug)]
pub(crate) struct Book {
    pub isbn: ISBN,
    pub libraries: Vec<Library>,
}

impl Book {
    pub(crate) fn new(isbn: ISBN, libraries: Vec<Library>) -> Self {
        Self { isbn, libraries }
    }
}
