// Import fetch.rs
use ytdlrs_lib::api::{fetch::APIClient, download::Download};

// tokio runtime
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut client = APIClient::new(String::from(" https://www.youtube.com/watch?v=4dvf6kM70qM"), String::from("mp4"))?;
    let info = client.fetch_video_info().await;
    // match for Info
    match info {
        Ok(info) => {
            let k = info.get_unique_key_by_quality(String::from("1080p"))?;
            let convert = client.fetch_convert_video(&k).await?;
            let download = Download {
                url: convert.get_download_link(),
                file_name: String::from("test.mp4")
            };
            download.download().await?;
        }
        Err(e) => {
            println!("owowowow error {}", e);
        }
    }
    Ok(())
}
