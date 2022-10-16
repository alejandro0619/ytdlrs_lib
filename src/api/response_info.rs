use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::error::Error;
type FormatInfo = HashMap<String, HashMap<String, String>>;
type VideoKeys = HashMap<String, String>;
// Model of the data returned when fetching the video information, quality, and keys.
#[derive(Serialize, Deserialize, Debug)]
pub struct APIResponseInfo {
    #[serde(rename = "a")]
    account: String,
    links: Link,
    pub mess: String,
    #[serde(rename = "p")]
    process: String,
    status: String,
    t: i32,
    title: String,
    vid: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum Link {
    #[serde(alias = "mp4")]
    MP4(FormatInfo),
    #[serde(alias = "mp3")]
    MP3(FormatInfo),
}
impl Link {
    /// Returns the video keys and the quality of the video.
    /// where the key is the quality and the value is the key.
    /// info is of type FormatInfo.
    fn get_keys(info: &FormatInfo) ->Result<VideoKeys, Error> {
        let mut keys: VideoKeys = HashMap::new();
       
            for (_, v) in info.iter() {
               // check if v is not empty and if contains k and q
               if !v.is_empty() && v.contains_key("k") && v.contains_key("q") {
                   keys.insert(v["q"].clone(), v["k"].clone());
               } else {
                   return Err(Error::VideoKeysError);
               }
            }
            Ok(keys)
    }
}

impl APIResponseInfo {
    /// Gets the keys within its quality.
    pub fn get_key_by_quality(&self) -> Result<VideoKeys, Error> {
        match &self.links {
            Link::MP4(info) => Link::get_keys(info),
            Link::MP3(info) => Link::get_keys(info),
        }
    }
    // Some getter functions to gather general information about the video.
    pub fn get_title(&self) -> String {
        String::from(&self.title)
    }

    pub fn get_video_id(&self) -> String {
        String::from(&self.vid)
    }

    pub fn get_account(&self) -> String {
        String::from(&self.account)
    }
}
