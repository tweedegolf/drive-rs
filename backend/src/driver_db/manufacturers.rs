use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum Manufacturer {
    AnalogDevices,
    ST,
    TI,
    NXP,
}
