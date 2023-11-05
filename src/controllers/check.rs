use crate::api::{ApiClient, ApiConfig, CheckApi, CheckRequest};
use anyhow::Result;

pub fn check() -> Result<()> {
    let client = CheckApi::from_client(ApiClient::from_config(ApiConfig::from_env()?));

    let res = client.check(CheckRequest {
        isbn: vec![],
        systemid: vec![],
    });

    dbg!(res);

    Ok(())
}
