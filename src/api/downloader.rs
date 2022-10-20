use super::error::Error;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Clone)]
pub struct DownloaderBuilder {
    pub url: Option<String>,
    pub file_name: Option<String>,
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
                Ok(Downloader {
                    url: url.clone(),
                    file_name: file_name.clone(),
                })
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
    pub async fn download(&self) -> Result<(), Error> {
        let response = reqwest::get(self.url.clone()).await?;
        let mut file = std::fs::File::create(self.file_name.clone())
            .map_err(|e| Error::CreateFileError(e.to_string()))?;
        let mut content = Cursor::new(response.bytes().await?);
        std::io::copy(&mut content, &mut file).unwrap();
        Ok(())
    }
}
