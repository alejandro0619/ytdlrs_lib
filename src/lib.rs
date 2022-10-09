use serde::{Deserialize, Serialize};
// use serde_json; 
use std::collections::HashMap;

/// Model of the data returned when fetching the video information, quality, and keys.
#[derive(Serialize, Deserialize, Debug)]
pub struct APIResponseInfo { 
    a: String,
    links: Link,
    mess: String,
    p: String,
    status: String,
    t: i32,
    title: String,
    vid: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    mp4: HashMap<String, HashMap<String, String>>,
}

impl APIResponseInfo {
    /// Gets the keys within its quality.
    pub fn get_key_by_quality(&self) -> HashMap<String, String> {
        let mut qualities: HashMap<String, String> = HashMap::new();

        for (_, value) in (self.links.mp4).clone() {
            // Iter through every video download option
            let quality = value.get("q").unwrap(); // This should not fail to get the quality
            let key = value.get("k").unwrap(); // This should not fail (either XD) to get the key
            qualities.insert(String::from(quality), String::from(key)); // convert to String and insert in the hashmap
        }
        qualities
    }
}

