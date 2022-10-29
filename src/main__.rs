use ytdlrs_lib::api::{client::APIClientBuilder, search::SearchVideo, downloader::DownloaderBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // We create the API Client by providing the url and the video type and video id.
  let mut res = APIClientBuilder::default()
    .set_url(String::from("https://www.youtube.com/watch?v=MAyWHFuHJuw"))
    .set_video_type(String::from("mp4"))
    .set_video_id(String::from("MAyWHFuHJuw"))
    .build()?;
    
  // Using the API Client we can fetch the video info. And using a method we can get the key to download the video.
  let video_key = res
    .fetch_video_info()
    .await?
    .get_unique_key_by_quality(String::from("720p"))?;
  
  // Now we can get information about the conversion of the video by it's key, such as the download link.
  let video_info = res
    .fetch_convert_video(&video_key)
    .await?;

  // We build the downloader by providing the download link and the video name. And with the method download we can download the video. (Duh)
  DownloaderBuilder::default()
    .set_url(video_info.get_download_link())
    .set_file_name(String::from("test.mp4"))
    .build()?
    .download()
    .await?;

  // If we don't know what the direct link of the video is, we can search for a name and get a couple (12) of results.
  let searched_videos = SearchVideo::search_video(String::from("Rust CLI"))
                                                      .await?
                                                      .get_items();

    // We can loop over the vector and get the video id and the title.
    for i in searched_videos {
      for (_, (video_id, video_title)) in i.iter().enumerate() {
        println!("{}: {}", video_id, video_title);
      }
    }

    Ok(())
}
