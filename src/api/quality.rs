#![allow(dead_code)]
use super::error::Error;
use std::fmt::Display;

pub trait Quality {
    type Quality;
    fn get_quality(quality: &str) -> Result<Self::Quality, Error>;
}
pub enum QualityVideo {
    Q144p,
    Q240p,
    Q360p,
    Q480p,
    Q720p,
    Q1080p,
    Qauto,
}
// implement DIsplay trait for QualityVideo
impl Display for QualityVideo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Q144p => write!(f, "144p"),
            Self::Q240p => write!(f, "240p"),
            Self::Q360p => write!(f, "360p"),
            Self::Q480p => write!(f, "480p"),
            Self::Q720p => write!(f, "720p"),
            Self::Q1080p => write!(f, "1080p"),
            Self::Qauto => write!(f, "auto"),
        }
    }
}
pub enum QualityAudio {
    Q64kbps,
    Q96kbps,
    Q128kbps,
    Q192kbps,
    Q256kbps,
    Q320kbps,
    Qauto,
}

pub enum FileType {
    Video,
    Audio,
}

impl Quality for QualityVideo {
    type Quality = Self;
    fn get_quality(quality: &str) -> Result<Self::Quality, Error> {
        match quality {
            "144p" => Ok(Self::Q144p),
            "240p" => Ok(Self::Q240p),
            "360p" => Ok(Self::Q360p),
            "480p" => Ok(Self::Q480p),
            "720p" => Ok(Self::Q720p),
            "1080p" => Ok(Self::Q1080p),
            "auto" => Ok(Self::Qauto),
            _ => Err(Error::InvalidQuality(String::from(quality))),
        }
    }
}

impl Quality for QualityAudio {
    type Quality = Self;
    fn get_quality(quality: &str) -> Result<Self::Quality, Error> {
        match quality {
            "64kbps" => Ok(Self::Q64kbps),
            "96kbps" => Ok(Self::Q96kbps),
            "128kbps" => Ok(Self::Q128kbps),
            "192kbps" => Ok(Self::Q192kbps),
            "256kbps" => Ok(Self::Q256kbps),
            "320kbps" => Ok(Self::Q320kbps),
            "auto" => Ok(Self::Qauto),
            _ => Err(Error::InvalidQuality(String::from(quality))),
        }
    }
}
impl Display for QualityAudio {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
         match self {
            QualityAudio::Q64kbps => write!(f, "64kbps"),
            QualityAudio::Q96kbps => write!(f, "96kbps"),
            QualityAudio::Q128kbps => write!(f, "128kbps"),
            QualityAudio::Q192kbps => write!(f, "192kbps"),
            QualityAudio::Q256kbps => write!(f, "256kbps"),
            QualityAudio::Q320kbps => write!(f, "320kbps"),
            QualityAudio::Qauto => write!(f, "auto"),
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
