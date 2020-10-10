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
