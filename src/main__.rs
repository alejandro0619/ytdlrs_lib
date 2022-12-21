use ytdlrs_lib::api::{
    client::APIClientBuilder, downloader::DownloaderBuilder, search::SearchVideo,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // We create the API Client by providing the url and the video type and video id.
    let res = APIClientBuilder::default()
        .set_video_type(String::from("mp4"))
        .set_video_id(String::from("MAyWHFuHJuw"))
        .build()?;

    // Using the API Client we can fetch the video info. And using a method we can get the key to download the video.
    let video_info = res
        .fetch_video_info()?
        .get_unique_key_by_quality(String::from("480p"))?;

    let video_info = res.fetch_convert_video(&video_info).unwrap();
    // We build the downloader by providing the download link and the video name. And with the method download we can download the video. (Duh)
    DownloaderBuilder::default()
        .set_url(video_info.get_download_link())
        .set_file_name(String::from("test.mp4"))
        .build()?
        .download()?;

    // If we don't know what the direct link of the video is, we can search for a name and get 12 results.
    let searched_videos = SearchVideo::search_video(String::from("Rust CLI"))?.get_items();

    // We can loop over the vector and get the video id and the title.
    for i in searched_videos {
        for (video_id, video_title)in i.iter() {
            println!("{}: {}", video_id, video_title);
        }
    }

    Ok(())
}
