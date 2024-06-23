use serde::{Deserialize, Serialize};

use super::{
    bool_parameter::BoolParameter,
    enum_parameter::EnumParameter,
    integer_parameter::IntegerParameter,
    real_parameter::RealParameter,
    time_signature_parameter::TimeSignatureParameter
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Parameter {
    BoolParameter(BoolParameter),
    EnumParameter(EnumParameter),
    IntegerParameter(IntegerParameter),
    RealParameter(RealParameter),
    TimeSignatureParameter(TimeSignatureParameter),
}