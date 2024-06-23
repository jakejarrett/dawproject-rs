use std::path::PathBuf;
use serde::{Deserialize, Serialize};

use crate::parameter::Parameter;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileReference {
    pub path: PathBuf,
    pub external: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginRole {
    Instrument,
    NoteFX,
    AudioFX,
    Analyzer
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInstance {
    pub plugin_version: String,
    pub device_role: PluginRole,
    pub loaded: bool,
    pub device_name: String,
    pub device_id: String, // TODO: Maybe an enum to differentiate between AU/CLAP (string), VST3 (uuid), VST2 (base-10)
    pub id: uuid::Uuid,
    pub name: String,
    pub params: Vec<Parameter>,

    // Optional values.
    pub device_vendor: Option<String>,
    pub color: Option<String>,
    pub comment: Option<String>,
    pub plugin_state: Option<FileReference>,
}