use super::error::Error;
use super::quality::{Quality, QualityAudio, QualityVideo};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    fn get_keys(info: &FormatInfo) -> Result<VideoKeys, Error> {
        let mut keys: VideoKeys = HashMap::new();

        for (_, v) in info.iter() {
            // check if v is not empty and if contains k and q
            if !v.is_empty() && v.contains_key("k") && v.contains_key("q") {
                keys.insert(v["q"].clone(), v["k"].clone());
            } else {
                return Err(Error::VideoKeys);
            }
        }
        Ok(keys)
    }
}

impl APIResponseInfo {
    pub fn get_account(&self) -> String {
        String::from(&self.account)
    }
    /// Gets the keys within its quality.
    pub fn get_keys_by_quality(&self) -> Result<VideoKeys, Error> {
        match &self.links {
            Link::MP4(info) => Link::get_keys(info),
            Link::MP3(info) => Link::get_keys(info),
        }
    }
    // Some getter functions to gather general information about the video.
    pub fn get_title(&self) -> String {
        String::from(&self.title)
    }

    pub fn get_unique_key_by_quality(&self, quality: String) -> Result<String, Error> {
        let keys = self.get_keys_by_quality()?;
        // check if quality is in the QualityAudio or QualityVideo

        match self.links {
            Link::MP4(_) => {
                let q = QualityVideo::get_quality(&quality)?;

                keys.get(&q)
                    .ok_or(Error::MissingQuality)
                    .map(|s| s.to_string())
            }
            Link::MP3(_) => {
                let q = QualityAudio::get_quality(&quality)?;
                
                keys.get(&q)
                    .ok_or(Error::MissingQuality)
                    .map(|s| s.to_string())
            }
        }
    }
    pub fn get_video_id(&self) -> String {
        String::from(&self.vid)
    }
}
