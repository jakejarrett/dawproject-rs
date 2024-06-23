use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSignaturePoint {
    pub numerator: i32,
    pub denominator: i32,
    pub time: f64,
}