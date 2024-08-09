use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub enum Manufacturer {
    AnalogDevices,
    ST,
    TI,
    NXP,
    #[default]
    Unknown,
}
