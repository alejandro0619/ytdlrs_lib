// Import fetch.rs
use ytdlrs_lib::api::fetch::{APIClient};

// tokio runtime
#[tokio::main]
async fn main() {
  let client = APIClient::new(String::from("https://www.youtube.com/watch?v=QH2-TGUlwu4"));
  let info = client.request().await;
  // match for Info
  match info {
    Ok(info) => {
      println!("{:#?}", info.get_key_by_quality());
    }
    Err(e) => {
      println!("Error: {}", e);
    }
  }

}
