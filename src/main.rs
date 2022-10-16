// Import fetch.rs
use ytdlrs_lib::api::fetch::APIClient;

// tokio runtime
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut client = APIClient::new(String::from(" https://www.youtube.com/watch?v=4dvf6kM70qM"), String::from("mp3"))?;
    let info = client.fetch_video_info().await;
    // match for Info
    match info {
        Ok(info) => {
            let k = info.get_key_by_quality()?;
            let k = k.get("320kbps").unwrap();
            let convert = client.fetch_convert_video(k).await?;
            println!("{:?}", convert.get_download_link());
        }
        Err(e) => {
            println!("owowowow error {}", e);
        }
    }
    Ok(())
}
