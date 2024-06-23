use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub originalartist: String,
    pub composer: String,
    pub songwriter: String,
    pub producer: String,
    pub arranger: String,
    pub year: String,
    pub genre: String,
    pub copyright: String,
    pub website: String,
    pub comment: String,
}