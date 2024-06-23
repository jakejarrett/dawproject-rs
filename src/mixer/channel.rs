use serde::{Deserialize, Serialize};

use crate::parameter::{BoolParameter, RealParameter};

use super::send::Send;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    REGULAR,
    MASTER,
    EFFECT,
    SUBMIX,
    VCA
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub role: Role,
    pub audio_channels: i32,
    pub solo: bool,
    pub destination: uuid::Uuid,
    pub id: String,
    pub name: String,
    pub color: String,
    pub comment: String,

    volume: RealParameter,
    pan: RealParameter,
    mute: BoolParameter,
    sends: Vec<Send>,
    // MORE...
}