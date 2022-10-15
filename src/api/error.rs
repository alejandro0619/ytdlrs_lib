use envy;
use reqwest;
use thiserror;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error parsing the response data: Check the URL")]
    ParseError,
    #[error("Invalid response")]
    InvalidResponse,
    #[error("Error deserializing the environment variables")]
    EnvError(#[from] envy::Error),
    #[error("Error initializing the client")]
    ClientError,
    #[error("Request went wrong")]
    RequestError(#[from] reqwest::Error),
    #[error("Could not deserialize the data into JSON")]
    DeserializeError,
    #[error("The status code is not 200")]
    StatusCodeError(reqwest::StatusCode),
}
