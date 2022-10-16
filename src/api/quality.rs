use super::error::Error;
pub trait Quality {
    fn get_quality(quality: &str) -> Result<String, Error>;
}
pub enum QualityVideo { }

pub enum QualityAudio {}

pub enum FileType {
    Video(QualityVideo),
    Audio(QualityAudio),
    Other(String),
}

impl Quality for QualityVideo {
    fn get_quality(quality: &str) -> Result<String, Error> {
        match quality {
            "144p" => Ok(String::from("144p")),
            "240p" => Ok(String::from("240p")),
            "360p" => Ok(String::from("360p")),
            "480p" => Ok(String::from("480p")),
            "720p" => Ok(String::from("720p")),
            "1080p" => Ok(String::from("1080p")),
            "auto" => Ok(String::from("auto")),
            _ => Err(Error::InvalidQuality(String::from(quality))),
        }
    }
}

impl Quality for  QualityAudio {
    fn get_quality(quality: &str) -> Result<String, Error> {
        match quality {
            "64kbps" => Ok(String::from("64kbps")),
            "96kbps" => Ok(String::from("96kbps")),
            "128kbps" => Ok(String::from("128kbps")),
            "192kbps" => Ok(String::from("192kbps")),
            "256kbps" => Ok(String::from("256kbps")),
            "320kbps" => Ok(String::from("320kbps")),
            "auto" => Ok(String::from("auto")),
            _ => Err(Error::InvalidQuality(String::from(quality))),
        }
    }
}
impl FileType {
    pub fn get_file_type(file_type: &str) -> Result<String, Error> {
        match file_type {
            "mp3" => Ok(String::from("mp3")),
            "mp4" => Ok(String::from("mp4")),
            _ => Err(Error::InvalidFileType(file_type.to_string())),
        }
    }
}
