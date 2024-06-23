use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RealPointInterpolation {
    HOLD,
    LINEAR
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealPoint {
    pub value: f64,
    pub interpolation: RealPointInterpolation,
    pub time: f64,
}