use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::common::{UnitValue, TimeUnit};

use super::automation_target::AutomationTarget;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FromPoints {
    BOOLPOINT(),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Points {
    pub unit: UnitValue,
    pub track: Uuid,
    pub time_unit:  TimeUnit,
    pub id: String,
    pub name: String,
    pub color: Option<String>,
    pub comment: Option<String>,
    pub target: AutomationTarget,
    pub from: Vec<FromPoints>,
}