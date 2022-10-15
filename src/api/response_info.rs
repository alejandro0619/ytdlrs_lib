use serde::{Deserialize, Serialize};
use std::collections::HashMap;
type FormatInfo = HashMap<String, HashMap<String, String>>;
// Model of the data returned when fetching the video information, quality, and keys.
#[derive(Serialize, Deserialize, Debug)]
pub struct APIResponseInfo {
    a: String,
    links: Link,
    pub mess: String,
    p: String,
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
    fn get_keys(info: &FormatInfo) -> HashMap<String, String> {
        let mut qualities: HashMap<String, String> = HashMap::new();
        for (_, value) in info.clone() {
            // Iter through every video download option
            let quality = value.get("q").unwrap(); // This should not fail to get the quality
            let key = value.get("k").unwrap(); // This should not fail (either XD) to get the key
            qualities.insert(String::from(quality), String::from(key)); // convert to String and insert in the
        }
        qualities
    }
}
impl APIResponseInfo {
    /// Gets the keys within its quality.
    pub fn get_key_by_quality(&self) -> Option<HashMap<String, String>> {
        match &self.links {
            Link::MP4(info) => Some(Link::get_keys(info)),
            Link::MP3(info) => Some(Link::get_keys(info)),
        }
    }
}
