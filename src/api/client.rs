use super::error::Error;
use dotenv;
use envy;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct APIConfig {
    pub url_base_info: String,
    pub url_base_download: String,
}
// Implement new for APIConfig
impl APIConfig {
    pub fn new() -> Result<Self, Error> {
        dotenv::dotenv().ok();
        envy::from_env::<APIConfig>().map_err(Error::Env)
    }
}

pub struct APIClient {
    client: reqwest::Client,
    url: String,
    base_url: APIConfig,
    vt: String,
    id: String,
}
// implement new for APiCient
impl APIClient {
  pub fn new(url: String, video_type: String, id: String) -> Result<Self, Error> {
      let base_url: APIConfig = APIConfig::new()?;
      let client = reqwest::Client::new();
      Ok(Self {
          client,
          url,
          base_url,
          vt: video_type,
          id, 
      })
  }
}
//setter for id:
impl APIClient {
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }
}
// getter for every field:
impl APIClient {
    pub fn get_client(&self) -> reqwest::Client {
        self.client.clone()
    }
    pub fn get_url(&self) -> String {
        self.url.clone()
    }
    pub fn get_base_url(&self) -> APIConfig {
        self.base_url.clone()
    }
    pub fn get_vt(&self) -> String {
        self.vt.clone()
    }
    pub fn get_id(&self) -> String {
        self.id.clone()
    }
}
pub struct APIClientBuilder {
    pub url: Option<String>,
    pub vt: Option<String>,
    pub id: Option<String>,
}
impl Default for APIClientBuilder {
    fn default() -> Self {
       Self::new()
    }
}
impl APIClientBuilder {
  pub fn new() -> Self {
      Self {
          url: None,
          vt: None,
          id: None,
      }
  }
  pub fn set_url(&mut self, url: String) -> &mut Self {
      self.url = Some(url);
      self
  }
  pub fn set_video_type(&mut self, vt: String) -> &mut Self {
      self.vt = Some(vt);
      self
  }
  pub fn set_video_id(&mut self, id: String) -> &mut Self {
      self.id = Some(id);
      self
  }
  pub fn build(&self) -> Result<APIClient, Error> {
      if let Some(url) = &self.url {
          if let Some(vt) = &self.vt {
              if let Some(id) = &self.id {
                  APIClient::new(url.clone(), vt.clone(), id.clone())
              } else {
                  Err(Error::MissingVideoId)
              }
          } else {
              Err(Error::MissingVideoType)
          }
      } else {
          Err(Error::MissingUrl)
      }
  }
}