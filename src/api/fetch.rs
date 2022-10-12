use crate::api::response_info;
use reqwest::{header::CONTENT_TYPE, Url};

struct Client {
  url: Url,
  video_url: String,
  response: response_info::APIResponseInfo,
}


// implement default for Client
impl Default for Client {
    fn default() -> Self {
        Self {
            url: Url::parse("https://9convert.com/api/ajaxSearch/index").unwrap(),
            response: response_info::APIResponseInfo::default(),
            video_url: String::new(),
        }
    }
}
// Implement request function for Client
impl Client {
    pub async fn request(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let res: reqwest::Response = client
            .post(self.url.clone())
            .header(CONTENT_TYPE, "application/json")
            .form(&[("query", self.video_url.clone()), ("vt", String::from("mp4"))])
            .send()
            .await?;
        self.response = res.json::<response_info::APIResponseInfo>().await?;
        Ok(())
    }
}