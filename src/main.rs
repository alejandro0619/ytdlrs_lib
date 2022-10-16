// Import fetch.rs
use ytdlrs_lib::api::fetch::APIClient;

// tokio runtime
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = APIClient::new(String::from(" https://www.youtube.com/watch?v=4dvf6kM70qM"), String::from("mp3"))?;
    let info = client.fetch_video_info().await;
    // match for Info
    match info {
        Ok(info) => {
            match info.get_key_by_quality() {
                Ok(keys) => {
                    for (k, v) in keys.iter() {
                        println!("Quality: {} Key: {}", k, v);
                    }
                }
                Err(e) => println!("Errorrr: {}", e),
            }
        }
        Err(e) => {
            println!("owowowow error {}", e);
        }
    }
    Ok(())
}
