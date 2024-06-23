use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoolParameter {
    pub value: bool,
    pub parameter_id: i32,
    pub id: String,
    pub name: String,
    pub color: String,
    pub comment: String,
}
