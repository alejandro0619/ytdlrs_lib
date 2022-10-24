use ytdlrs_lib::api::{client::APIClientBuilder, search::SearchVideo};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let res = APIClientBuilder::new()
    .set_url(String::from("https://www.youtube.com/watch?v=MAyWHFuHJuw"))
    .set_video_type(String::from("mp4"))
    .set_video_id(String::from("MAyWHFuHJuw"))
    .build()?
    .fetch_video_info()
    .await?;

  let searched_videos = SearchVideo::search_video(String::from("Rust CLI")).await?
    .get_items();

    for i in searched_videos {
      for (_, (video_id, video_title)) in i.iter().enumerate() {
        println!("{}: {}", video_id, video_title);
      }
    }

    Ok(())
}