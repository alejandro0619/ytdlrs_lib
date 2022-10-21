// Import fetch.rs
use ytdlrs_lib::api::{downloader::DownloaderBuilder, fetch::APIClientBuilder};

// tokio runtime
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut client = 
    APIClientBuilder::new()
        .set_url("https://www.youtube.com/watch?v=9bZkp7q19f0".to_string())
        .set_video_type("mp4".to_string())
        .set_video_id("9bZkp7q19f0".to_string())
        .build()?;
        
    let info = client.fetch_video_info().await;
    // match for Info
    match info {
        Ok(info) => {
            let k = info.get_unique_key_by_quality(String::from("1080p"))?;
            let convert = client.fetch_convert_video(&k).await?;
            println!("{:?}", convert.get_download_link());
            
        }
        Err(e) => {
            println!("owowowow error {}", e);
        }
    }
    Ok(())
}
