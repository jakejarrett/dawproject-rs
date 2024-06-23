use serde::{Deserialize, Serialize};
use crate::{mixer::channel::Channel, parameter::{real_parameter::RealParameter, time_signature_parameter::TimeSignatureParameter}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub version: String,
    pub application: Application,
    pub transport: Transport,
    pub structure: Vec<ProjectStructure>,
    pub arrangement: Arrangement,
    pub scenes: Vec<Scene>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectStructure {
    Channel(Channel),
    Track(Track),
}

// MOVE THESE TO OTHER.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Application {
    /** User friendly name, try not to use the app identifier. */
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transport {
    pub tempo: RealParameter,
    pub time_signature: TimeSignatureParameter,
}