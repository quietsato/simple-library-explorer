use crate::models::{ApiResponse, Config};
use futures::executor::block_on;
use reqwest;
use std::io::Result;

pub(crate) fn access_api(config: Config) -> Result<ApiResponse> {
    let params = [
        ("appkey", config.api_key),
        ("systemid", config.systemid),
        ("isbn", config.isbn.join(",")),
    ];

    let client = reqwest::Client::new();
    let res_raw = block_on(client.post(&config.api_url).form(&params).send()?);

    let res = "";

    return res;
}
