use super::error::Error;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Clone)]
pub struct DownloaderBuilder {
    pub url: Option<String>,
    pub file_name: Option<String>,
}

impl Default for DownloaderBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl DownloaderBuilder {
    pub fn new() -> Self {
        Self {
            url: None,
            file_name: None,
        }
    }
    pub fn set_url(&mut self, url: String) -> &mut Self {
        self.url = Some(url);
        self
    }
    pub fn set_file_name(&mut self, file_name: String) -> &mut Self {
        self.file_name = Some(file_name);
        self
    }
    pub fn build(&self) -> Result<Downloader, Error> {
        if let Some(url) = &self.url {
            if let Some(file_name) = &self.file_name {
                Ok(Downloader::new(url.clone(), file_name.clone()))
            } else {
                Err(Error::MissingFileName)
            }
        } else {
            Err(Error::MissingUrl)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Downloader {
    pub url: String,
    pub file_name: String,
}

impl Downloader {
    fn new(url: String, file_name: String) -> Self {
        Self { url, file_name }
    }
    pub fn download(&self) -> Result<(), Error> {
        let response = reqwest::blocking::Client::builder()
            .danger_accept_invalid_certs(true) // Workaround because the server is using a self-signed certificate
            .build()?
            .get(&self.url.clone())
            .send()?;

        // Checks if the response is succesfully made
        if response.status().is_success() {
            let bytes = response.bytes()?; // Gets the bytes from the response

            let mut file = std::fs::File::create(self.file_name.clone())
                .map_err(|e| Error::CreateFileFailed(e.to_string()))?;

            let mut content = Cursor::new(bytes);
            std::io::copy(&mut content, &mut file).unwrap();

            Ok(())
        } else {
            Err(Error::DownloadFailed)
        }
    }
}
