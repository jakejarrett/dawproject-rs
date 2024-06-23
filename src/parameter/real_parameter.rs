use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RealParameterValue {
    LINEAR,
    NORMALIZED,
    PERCENT,
    DECIBEL,
    HERTZ,
    SEMITONES,
    SECONDS,
    BEATS,
    BPM
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealParameter {
    pub value: f64,
    pub unit: RealParameterValue,
    pub min: f64,
    pub max: f64,
    pub parameter_id: i32,
    pub id: String,
    pub name: String,
    pub color: String,
    pub comment: String,
}