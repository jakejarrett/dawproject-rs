use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutomationExpression {
    GAIN,
    PAN,
    TRANSPOSE,
    TIMBRE,
    FORMANT,
    PRESSURE,
    CHANNELCONTROLLER(Option<i32>),
    CHANNELPRESSURE,
    POLYPRESSURE(Option<i32>),
    PITCHBEND,
    PROGRAMCHANGE
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationTarget {
    pub parameter: Uuid,
    pub expression: AutomationExpression,
    pub channel: i32,

}