use envy;
use reqwest;
use thiserror;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error parsing the response data: Check the URL")]
    Parse,
    #[error("Invalid response from the server")]
    InvalidResponse,
    #[error("Error deserializing the environment variables")]
    Env(#[from] envy::Error),
    #[error("Error initializing the client")]
    Client,
    #[error("Request went wrong")]
    Request(#[from] reqwest::Error),
    #[error("Could not deserialize the data into JSON")]
    Deserialize,
    #[error("Error getting the video keys")]
    VideoKeys,
    #[error("Invalidd video: Does not exist or is private")]
    InvalidVideo,
    #[error("Error converting the video")]
    ConvertFailed,
    #[error("Invalid quality: {0}")]
    InvalidQuality(String),
    #[error("Invalid file type: {0}")]
    InvalidFileType(String),
    #[error("Error downloading the file")]
    DownloadFailed,
    #[error("Error creating the directory at: {0}")]
    CreateDirFailed(String),
    #[error("Error creating the file at: {0}")]
    CreateFileFailed(String),
    #[error("Cannot build Downloader without a url")]
    MissingUrl,
    #[error("Cannot build Downloader without a file name")]
    MissingFileName,
    #[error("Cannot build the Client without the video id")]
    MissingVideoId,
    #[error("Cannot build the Client without the video type")]
    MissingVideoType,
    #[error("Video's Id is missing")]
    MissingId,
    #[error("Video's Title is missing")]
    MissingTitle,
    #[error("Video does not have the quality requested")]
    MissingQuality,
}
