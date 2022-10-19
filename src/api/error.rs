use envy;
use reqwest;
use thiserror;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error parsing the response data: Check the URL")]
    ParseError,
    #[error("Invalid response from the server")]
    InvalidResponse,
    #[error("Error deserializing the environment variables")]
    EnvError(#[from] envy::Error),
    #[error("Error initializing the client")]
    ClientError,
    #[error("Request went wrong")]
    RequestError(#[from] reqwest::Error),
    #[error("Could not deserialize the data into JSON")]
    DeserializeError,
    #[error("Error getting the video keys")]
    VideoKeysError,
    #[error("Invalidd video: Does not exist or is private")]
    InvalidVideo,
    #[error("Error converting the video")]
    ConvertError,
    #[error("Invalid quality: {0}")]
    InvalidQuality(String),
    #[error("Invalid file type: {0}")]
    InvalidFileType(String),
    #[error("Error downloading the {0}")]
    DownloadError(String),
    #[error("Error creating the directory at: {0}")]
    CreateDirError(String),
    #[error("Error creating the file at: {0}")]
    CreateFileError(String),
}
