// allow unused fields in struct for now 
#![allow(dead_code)]
use super::error::Error;
use super::response_info::APIResponseInfo;
use dotenv;
use envy;
use reqwest::{header::CONTENT_TYPE, StatusCode};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct APIConfig {
    url_base_info: String,
    url_base_download: String,
}
// Implement new for APIConfig
impl APIConfig {
    pub fn new() -> Result<Self, Error> {
        dotenv::dotenv().ok();
        envy::from_env::<APIConfig>()
        .map_err(|e| Error::EnvError(e))
    }
}

pub struct APIClient {
    client: reqwest::Client,
    url: String,
    base_url: APIConfig,
}

impl APIClient {
    pub async fn request(&self) -> Result<APIResponseInfo, Error> {
        let url_base = self.base_url.url_base_info.clone();

        // These are the params, vt = video_type.
        let params = [("query", self.url.clone()), ("vt", String::from("mp3"))];

        let client = &self.client; // Initialize the client

        let response_data = client
            .post(url_base)
            .form(&params)
            .header(CONTENT_TYPE, "application/json")
            .send()
            .await
            .map_err(|e| Error::RequestError(e))?;

        if let StatusCode::OK = response_data.status() {
            let data = response_data.json::<APIResponseInfo>().await;
            match data {
                Ok(data) => Ok(data),

                Err(_) => Err(Error::DeserializeError),
            }
        } else { 
            Err(Error::StatusCodeError(response_data.status()))
        }
    }
}

// implement new for APiCient
impl APIClient {
    pub fn new(url: String) -> Result<Self, Error> {
        let base_url: APIConfig = APIConfig::new()?;
        let client = reqwest::Client::new();
        Ok(APIClient {
            client,
            url,
            base_url,
        })
    }
}
