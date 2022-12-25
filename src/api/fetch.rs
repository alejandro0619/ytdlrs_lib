use super::client::APIClient;
use super::convert_video::APIResponseConvert;
use super::error::Error;
use super::quality::FileType;
use super::response_info::APIResponseInfo;

use reqwest::header::CONTENT_TYPE;

impl APIClient {
    /// Returns the fetch info of this [`APIClient`].
    ///
    /// # Errors
    ///
    /// This function will return an error if the Request fails | Deserialize fails .
    pub fn fetch_info(&self) -> Result<APIResponseInfo, Error> {
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
            .map_err(Error::Request)? // Returns error if request fails
            .json::<APIResponseInfo>()
            .map_err(|_| Error::DeserializeInfo)?; // Tries to deserialize the response data into APIResponseInfo

        // If the mess is empty means it all went well.
        if !response_data.mess.is_empty() {
            Err(Error::InvalidResponse)
        } else {
            Ok(response_data)
        }
    }
}

impl APIClient {
    pub fn fetch_convert_video(&self, k: &str) -> Result<APIResponseConvert, Error> {
        let url_base = self.get_env().url_base_download;

        // These are the params, vt = video_type.

        let params = [("vid", self.get_id()), ("k", String::from(k))];

        let client = &self.get_client(); // Initialize the client

        let response_data = client
            .post(url_base)
            .form(&params)
            .header(CONTENT_TYPE, "application/json")
            .send()
            .map_err(Error::Request)? // Returns error if request fails
            .json::<APIResponseConvert>()
            .map_err(|_| Error::DeserializeConvertion)?; // Tries to deserialize the response data into APIResponseInfo

        if !response_data.get_mess().is_empty() {
            Err(Error::ConvertFailed)
        } else {
            Ok(response_data)
        }
    }
}
