use serde::{Deserialize, Serialize};
// use serde_json; 
use std::collections::HashMap;

/// Model of the data returned when fetching the video information, quality, and keys.
#[derive(Serialize, Deserialize, Debug)]
pub struct APIResponseInfo { 
    a: String,
    pub links: Link,
    mess: String,
    p: String,
    status: String,
    t: i32,
    title: String,
    vid: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    pub mp4: HashMap<String, HashMap<String, String>>,
}