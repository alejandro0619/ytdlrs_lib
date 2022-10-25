
use super::convert_video::APIResponseConvert;
use super::error::Error;
use super::quality::FileType;
use super::response_info::APIResponseInfo;
use super::client::APIClient;

use reqwest::header::CONTENT_TYPE;


impl APIClient {
    pub async fn fetch_video_info(&mut self) -> Result<APIResponseInfo, Error> {
        let url_base = self.get_env().url_base_info;

        // we check if the file type is mp3 or mp4.
        let file_type = FileType::get_file_type(&self.get_vt())?;

        let params: [(&str, String); 2] = [("query", self.get_url()), ("vt", file_type)]; // params for the request
        let client = &self.get_client(); // Initialize the client

        let response_data = client
            .post(url_base)
            .form(&params)
            .header(CONTENT_TYPE, "application/json")
            .send()
            .await
            .map_err(Error::Request)? // Returns error if request fails
            .json::<APIResponseInfo>()
            .await
            .map_err(|_| Error::Deserialize)?; // Tries to deserialize the response data into APIResponseInfo

        self.set_id(response_data.get_video_id()); // Get the video id and set it to self.id

        // If the mess is empty means it all went well.
        if !response_data.mess.is_empty() {
            Err(Error::InvalidResponse)
        } else {
            Ok(response_data)
        }
    }
}

impl APIClient {
    pub async fn fetch_convert_video(&self, k: &str) -> Result<APIResponseConvert, Error> {
        let url_base = self.get_env().url_base_download;

        // These are the params, vt = video_type.

        let params = [("vid", self.get_id()), ("k", String::from(k))];

        let client = &self.get_client(); // Initialize the client

        let response_data = client
            .post(url_base)
            .form(&params)
            .header(CONTENT_TYPE, "application/json")
            .send()
            .await
            .map_err(Error::Request)? // Returns error if request fails
            .json::<APIResponseConvert>()
            .await
            .map_err(|_| Error::Deserialize)?; // Tries to deserialize the response data into APIResponseInfo

        if !response_data.get_mess().is_empty() {
            Err(Error::ConvertFailed)
        } else {
            Ok(response_data)
        }
    }
}



