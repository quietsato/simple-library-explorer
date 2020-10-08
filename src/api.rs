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
    let res_raw = match block_on(client.post(&config.api_url).form(&params).send()) {
        Ok(res_raw) => res_raw,
        Err(e) => panic!(e),
    };

    let res: ApiResponse = match block_on(res_raw.json()) {
        Ok(res) => res,
        Err(e) => panic!(e),
    };

    return Ok(res);
}
