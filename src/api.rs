mod response_info;

// use serde_json; 
use std::collections::HashMap;

impl response_info::APIResponseInfo {
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

