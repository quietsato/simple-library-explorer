use super::client::ApiClient;
use anyhow::Result;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct CheckApi {
    client: ApiClient,
}

pub struct CheckRequest {
    pub isbn: Vec<String>,
    pub systemid: Vec<String>,
}

impl CheckApi {
    pub fn from_client(client: ApiClient) -> Self {
        Self { client }
    }

    pub fn check(self, req: CheckRequest) -> Result<CheckResponse> {
        let response = self
            .client
            .reqwest_builder("/check")
            .query(&[
                ("isbn", &req.isbn.join(",")),
                ("systemid", &req.systemid.join(",")),
            ])
            .send()?;
        dbg!(response.json::<serde_json::Value>()?);
        // Ok(response.json::<CheckResponse>()?)
        Err(anyhow::anyhow!(""))
    }
}

#[derive(Debug, Deserialize)]
pub struct CheckResponse {
    session: Option<String>,
    books: BTreeMap<SystemId, CheckStatus>,
    #[serde(rename = "continue")]
    pub _continue: bool,
}

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct SystemId(pub String);

#[derive(Debug, Deserialize)]
pub struct CheckStatus {
    pub status: String,
    pub reserveurl: String,
    pub libkey: BTreeMap<LibraryName, BookStatus>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct LibraryName(pub String);
#[derive(Debug, Deserialize)]
pub struct BookStatus(pub String);
