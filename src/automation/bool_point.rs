use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoolPoint {
    pub value: bool,
    pub time: f64,
}