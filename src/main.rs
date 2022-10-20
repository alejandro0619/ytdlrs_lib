// Import fetch.rs
use ytdlrs_lib::api::{downloader::DownloaderBuilder, fetch::APIClient};

// tokio runtime
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut client = APIClient::new(
        String::from(" https://www.youtube.com/watch?v=4dvf6kM70qM"),
        String::from("mp4"),
    )?;
    let info = client.fetch_video_info().await;
    // match for Info
    match info {
        Ok(info) => {
            let k = info.get_unique_key_by_quality(String::from("1080p"))?;
            let convert = client.fetch_convert_video(&k).await?;
            let downloader = DownloaderBuilder::new()
                .set_url(convert.get_download_link())
                .set_file_name(String::from("Video_test.mp4"))
                .build()?
                .download()
                .await?;
                
        }
        Err(e) => {
            println!("owowowow error {}", e);
        }
    }
    Ok(())
}
