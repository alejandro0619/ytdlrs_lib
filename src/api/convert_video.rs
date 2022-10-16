use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct APIResponseConvert {
  status: String,
  mess: String,
  c_status: String,
  vid: String,
  title: String,
  #[serde(rename = "ftype")]
  file_type: String,
  #[serde(rename = "fquality")]
  file_quality: String,
  #[serde(rename = "dlink")]
  download_link: String,
}

impl APIResponseConvert {
  pub fn get_download_link(&self) -> String {
    String::from(&self.download_link)
  }
  pub fn file_quality(&self) -> String {
    String::from(&self.file_quality)
  }

  pub fn file_type(&self) -> String {
    String::from(&self.file_type)
  }
}
