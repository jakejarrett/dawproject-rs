use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumParameter {
    pub value: bool,
    pub count: i32,
    pub labels: Vec<String>,
    pub parameter_id: i32,
    pub id: String,
    pub name: String,
    pub color: String,
    pub comment: String,
}
