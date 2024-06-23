use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegerParameter {
    pub value: bool,
    pub min: i32,
    pub max: i32,
    pub parameter_id: i32,
    pub id: String,
    pub name: String,
    pub color: String,
    pub comment: String,
}