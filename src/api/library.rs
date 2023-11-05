use super::client::ApiClient;
use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct LibraryApi {
    client: ApiClient,
}

pub struct ListLibrariesRequest {
    pub pref: String,
    pub city: String,
}

impl LibraryApi {
    pub fn from_client(client: ApiClient) -> Self {
        Self { client }
    }

    pub fn list_libraries(self, req: ListLibrariesRequest) -> Result<ListLibrariesResponse> {
        let response = self
            .client
            .reqwest_builder("/library")
            .query(&[("pref", &req.pref), ("city", &req.city)])
            .send()?;
        Ok(response.json::<ListLibrariesResponse>()?)
    }
}

#[derive(Debug, Deserialize)]
pub struct ListLibrariesResponse(pub Vec<Library>);

#[derive(Debug, Deserialize)]
pub struct Library {
    pub systemid: String,
    pub systemname: String,
    pub libkey: String,
    pub libid: String,
    pub short: String,
    pub formal: String,
    pub url_pc: String,
    pub address: String,
    pub pref: String,
    pub city: String,
    pub post: String,
    pub tel: String,
    pub geocode: String,
    pub category: String,
}
