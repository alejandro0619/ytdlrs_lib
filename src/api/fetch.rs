use crate::api::response_info;
use dotenv;
use envy;
use reqwest::{header::CONTENT_TYPE, StatusCode};
use serde_json;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct APIConfig {
    url_base_info: String,
    url_base_download: String,
}
// Implement new for APIConfig
impl APIConfig {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        envy::from_env::<APIConfig>().unwrap()
    }
}

pub struct APIClient {
    client: reqwest::Client,
    url: String,
    base_url: APIConfig,
}

impl APIClient {

    pub async fn request(
        &self,
    ) -> Result<response_info::APIResponseInfo, Box<dyn std::error::Error>> {
        let url_base = self.base_url.url_base_info.clone();

        // These are the params, vt does not change as it will always download mp4
        let params = [("query", self.url.clone()), ("vt", String::from("mp4"))];

        let client = &self.client; // Initialize the client

        let response_data = client
            .post(url_base)
            .form(&params)
            .header(CONTENT_TYPE, "application/json")
            .send()
            .await?
            .json::<response_info::APIResponseInfo>() // Deserialize the response into the APIResponseInfo struct
            .await?;

        

        Ok(response_data)
    }
}

// implement new for APiCient
impl APIClient {
    pub fn new(url: String) -> Self {
        let base_url = APIConfig::new();
        let client = reqwest::Client::new();
        APIClient {
            client,
            url,
            base_url,
        }
    }
}
