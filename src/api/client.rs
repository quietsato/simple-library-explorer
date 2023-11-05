use anyhow::Result;
use reqwest::blocking::RequestBuilder;

#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub api_url: String,
    pub api_key: String,
}

impl ApiConfig {
    pub fn from_env() -> Result<ApiConfig> {
        Ok(ApiConfig {
            api_url: std::env::var("API_URL")?,
            api_key: std::env::var("API_KEY")?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ApiClient {
    config: ApiConfig,
}

impl ApiClient {
    pub fn from_config(config: ApiConfig) -> Self {
        Self { config }
    }

    pub fn reqwest_builder(&self, path: &str) -> RequestBuilder {
        assert!(path.starts_with("/"));
        let url = format!("{}{}", self.config.api_url, path);
        reqwest::blocking::Client::new().get(&url).query(&[
            ("appkey", self.config.api_key.as_str()),
            ("format", "json"),
            ("callback", "no"),
        ])
    }
}
