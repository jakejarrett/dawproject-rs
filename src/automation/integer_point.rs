use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegerPoint {
    pub value: i64,
    pub time: f64,
}