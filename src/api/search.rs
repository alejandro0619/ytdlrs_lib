use super::error::Error;
use reqwest::{blocking::Client, header::CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

type VideoId = String;
type VideoTitle = String;
type APIResponseSearchItem = HashMap<VideoId, VideoTitle>; // Afterall it's a HashMap of String, String, but this is more readable

#[derive(Deserialize, Serialize, Debug)]
pub struct APIResponseSearch {
    status: String,
    #[serde(rename = "mess")]
    message: String,
    #[serde(rename = "p")]
    process: String,
    items: Vec<APIResponseSearchItem>,
    #[serde(skip)]
    url: String,
}
// setter for url:
impl APIResponseSearch {
    pub fn set_url(&mut self, id: String) {
        self.url = format!("https://www.youtube.com/{id}");
    }
}
impl APIResponseSearch {
    pub fn get_items(&self) -> Vec<APIResponseSearchItem> {
        self.items.clone()
    }
    pub fn get_message(&self) -> String {
        self.message.clone()
    }
    pub fn get_process(&self) -> String {
        self.process.clone()
    }
    pub fn get_status(&self) -> String {
        self.status.clone()
    }
    pub fn get_url(&self) -> String {
        self.url.clone()
    }
}

pub struct SearchVideo {}

impl SearchVideo {
    pub fn search_video(query: String) -> Result<APIResponseSearch, Error> {
        let client = Client::new();
        let url = String::from("https://9convert.com/api/ajaxSearch/index");

        let params = [("query", query)];
        let response_data = client
            .post(url)
            .form(&params)
            .header(CONTENT_TYPE, "application/json")
            .send()
            .map_err(Error::Request)?
            .json::<APIResponseSearch>()
            .map_err(|_| Error::DeserializeSearch)?;

        Ok(response_data)
    }
}
