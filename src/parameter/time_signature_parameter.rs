use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSignatureParameter {
    pub numerator: i32,
    pub denominator: i32,
    pub min: f64,
    pub max: f64,
    pub parameter_id: i32,
    pub id: String,
    pub name: String,
    pub color: String,
    pub comment: String,
}