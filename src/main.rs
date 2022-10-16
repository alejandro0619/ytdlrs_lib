// Import fetch.rs
use ytdlrs_lib::api::fetch::APIClient;

// tokio runtime
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut client = APIClient::new(String::from(" https://www.youtube.com/watch?v=4dvf6kM70qM"), String::from("mp4"))?;
    let info = client.fetch_video_info().await;
    // match for Info
    match info {
        Ok(info) => {
            let k = info.get_unique_key_by_quality(String::from("1080p"))?;
            dbg!(k);
        }
        Err(e) => {
            println!("owowowow error {}", e);
        }
    }
    Ok(())
}
