use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumPoint {
    pub value: i64,
    pub time: f64,
}