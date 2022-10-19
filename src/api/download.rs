use serde::{Deserialize, Serialize};
use reqwest::Client;
// Import Cursor
use std::io::Cursor;
use super::error::Error;


#[derive(Debug, Serialize, Deserialize)]
pub struct Downloader {
    pub url: String,
    pub file_name: String
}

impl Downloader {
    pub async fn download(&self) -> Result<(), Error> {
      let response = reqwest::get(self.url.clone()).await?;
      let mut file = std::fs::File::create(self.file_name.clone()).map_err(|e| Error::CreateFileError(e.to_string()))?;
      let mut content = Cursor::new(response.bytes().await?);
      std::io::copy(&mut content, &mut file).unwrap();


        Ok(())
    }
}
 