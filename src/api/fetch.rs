use crate::api::response_info;
use reqwest::{header::CONTENT_TYPE, Url};

pub async fn request(url: &String) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res: reqwest::Response = client
        .post(url.clone())
        .header(CONTENT_TYPE, "application/json")
        .form(&[("query", url), ("vt", &String::from("mp4"))])
        .send()
        .await?;
    res.json::<response_info::APIResponseInfo>().await?;
    Ok(())
}