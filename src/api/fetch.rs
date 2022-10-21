// allow unused fields in struct for now
#![allow(dead_code)]
use super::convert_video::APIResponseConvert;
use super::error::Error;
use super::quality::FileType;
use super::response_info::APIResponseInfo;
use dotenv;
use envy;
use reqwest::header::CONTENT_TYPE;
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
        envy::from_env::<APIConfig>().map_err(|e| Error::EnvError(e))
    }
}

pub struct APIClient {
    client: reqwest::Client,
    url: String,
    base_url: APIConfig,
    vt: String,
    id: String,
}
pub struct APIClientBuilder {
    pub url: Option<String>,
    pub vt: Option<String>,
    pub id: Option<String>,
}
impl APIClientBuilder {
    pub fn new() -> Self {
        Self {
            url: None,
            vt: None,
            id: None,
        }
    }
    pub fn set_url(&mut self, url: String) -> &mut Self {
        self.url = Some(url);
        self
    }
    pub fn set_video_type(&mut self, vt: String) -> &mut Self {
        self.vt = Some(vt);
        self
    }
    pub fn set_video_id(&mut self, id: String) -> &mut Self {
        self.id = Some(id);
        self
    }
    pub fn build(&self) -> Result<APIClient, Error> {
        if let Some(url) = &self.url {
            if let Some(vt) = &self.vt {
                if let Some(id) = &self.id {
                    APIClient::new(url.clone(), vt.clone(), id.clone())
                } else {
                    Err(Error::MissingVideoId)
                }
            } else {
                Err(Error::MissingVideoType)
            }
        } else {
            Err(Error::MissingUrl)
        }
    }
}
impl APIClient {
    pub async fn fetch_video_info(&mut self) -> Result<APIResponseInfo, Error> {
        let url_base = self.base_url.url_base_info.clone();

        // we check if the file type is mp3 or mp4.
        let file_type = FileType::get_file_type(&self.vt)?;

        let params: [(&str, String); 2] = [("query", self.url.clone()), ("vt", file_type)]; // params for the request
        let client = &self.client; // Initialize the client

        let response_data = client
            .post(url_base)
            .form(&params)
            .header(CONTENT_TYPE, "application/json")
            .send()
            .await
            .map_err(|e| Error::RequestError(e))? // Returns error if request fails
            .json::<APIResponseInfo>()
            .await
            .map_err(|_| Error::DeserializeError)?; // Tries to deserialize the response data into APIResponseInfo

        self.id = response_data.get_video_id(); // Get the video id and set it to self.id

        // If the mess is empty means it all went well.
        if !response_data.mess.is_empty() {
            return Err(Error::InvalidResponse);
        } else {
            Ok(response_data)
        }
    }
}

impl APIClient {
    pub async fn fetch_convert_video(&self, k: &String) -> Result<APIResponseConvert, Error> {
        let url_base = self.base_url.url_base_download.clone();

        // These are the params, vt = video_type.

        let params = [("vid", self.id.clone()), ("k", k.clone())];

        let client = &self.client; // Initialize the client

        let response_data = client
            .post(url_base)
            .form(&params)
            .header(CONTENT_TYPE, "application/json")
            .send()
            .await
            .map_err(|e| Error::RequestError(e))? // Returns error if request fails
            .json::<APIResponseConvert>()
            .await
            .map_err(|_| Error::DeserializeError)?; // Tries to deserialize the response data into APIResponseInfo

        if !response_data.get_mess().is_empty() {
            return Err(Error::ConvertError);
        } else {
            Ok(response_data)
        }
    }
}
// implement new for APiCient
impl APIClient {
    pub fn new(url: String, video_type: String, id: String) -> Result<Self, Error> {
        let base_url: APIConfig = APIConfig::new()?;
        let client = reqwest::Client::new();
        Ok(APIClient {
            client,
            url,
            base_url,
            vt: video_type,
            id, 
        })
    }
}
