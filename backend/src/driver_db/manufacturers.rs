use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub enum ChipManufacturer {
    AnalogDevices,
    /// Also known as ST Micro
    ST,
    TI,
    NXP,
    Toshiba,
    Sensirion,
    SolomonSystech,

    #[default]
    Unknown,
}
