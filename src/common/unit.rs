use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnitValue {
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