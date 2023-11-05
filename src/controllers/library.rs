use crate::api::{ApiClient, ApiConfig, LibraryApi, ListLibrariesRequest};
use anyhow::Result;

pub fn list_libraries() -> Result<()> {
    let client = LibraryApi::from_client(ApiClient::from_config(ApiConfig::from_env()?));

    let res = client.list_libraries(ListLibrariesRequest {
        pref: "".into(),
        city: "".into(),
    });

    dbg!(res);

    Ok(())
}
