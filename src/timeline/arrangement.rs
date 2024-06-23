use serde::{Deserialize, Serialize};

use crate::automation::Points;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arrangement {
    pub time_signature_automation: Option<Points>,
    pub tempo_automation: Option<Points>,
    pub markers: Option<Points>,
    pub lanes: Option<Lanes>,

    // Copy paste of generic attributes.
    pub id: String,
    pub name: String,
    pub color: String,
    pub comment: String,
}
