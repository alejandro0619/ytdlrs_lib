// Import fetch.rs
use ytdlrs_lib::api::fetch::APIClient;

// tokio runtime
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = APIClient::new(String::from("https://www.youtube.com/watch?v=x4eNrwR4FyI"))?;
    let info = client.request().await;
    // match for Info
    match info {
        Ok(info) => {
            println!("{:#?}", info.get_key_by_quality());
        }
        Err(e) => {
            println!("Molleja de error hermano: {}", e);
        }
    }
    Ok(())
}
