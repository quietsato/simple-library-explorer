use crate::models::{ApiResponse, Config};
use reqwest;
use serde_json;
use std::io::Result;

pub(crate) fn access_api(config: Config) -> Result<String> {
    let params = [
        ("appkey", config.api_key),
        ("systemid", config.systemid),
        ("isbn", config.isbn.join(",")),
        ("callback", "no".to_string()),
    ];

    debug!("{}", &config.api_url);

    let client = reqwest::blocking::Client::new();

    let res_raw = match client.get(&config.api_url).form(&params).send() {
        Ok(res_raw) => res_raw,
        Err(e) => panic!(e),
    };

    match res_raw.json::<String>() {
        Ok(res) => Ok(res),
        Err(e) => {
            use std::io::{Error, ErrorKind};
            Err(Error::new(ErrorKind::Other, e))
        }
    }
}

fn deserialize_api_response(json: &String) -> Result<ApiResponse> {
    match serde_json::from_str::<ApiResponse>(&json) {
        Ok(res) => Ok(res),
        Err(e) => {
            use std::io::{Error, ErrorKind};
            Err(Error::new(ErrorKind::Other, e))
        }
    }
}

#[test]


#[test]
fn access_api_test() {
    env_logger::init();

    use httpmock::Method::GET;
    use httpmock::Mock;
    use httpmock::MockServer;
    let json_res = r#"{
            "session": "11a285036112525afe32b1a3d4c36245", 
            "books": {
              "4334926940": {
                "Tokyo_Setagaya": {"status": "OK", "reserveurl": "http://libweb.tokyo.jp/123", 
                  "libkey": {"玉川台": "貸出可", "世田谷": "貸出中", "経堂": "館内のみ"}}
              }, 
              "4088700104": {
                "Tokyo_Setagaya": {"status": "Running", "reserveurl": "", 
                  "libkey": {}}
              }
            }, 
            "continue": 1
          }"#;

    let mock_server = MockServer::start();
    debug!("mock server is listening on {}", mock_server.address());

    let api_mock = Mock::new()
        .expect_method(GET)
        .expect_path("/check")
        .return_status(200)
        .return_header("Content-Type", "application/json")
        .return_json_body(&json_res)
        .create_on(&mock_server);

    let config: Config = {
        let mut c: Config = Default::default();
        c.api_url = mock_server.url("/check");
        c
    };
    if let Ok(res) = access_api(config) {
        info!("{}", res);
    };
}
