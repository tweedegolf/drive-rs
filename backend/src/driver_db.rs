use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use url::Url;

pub mod categories;
pub mod manufacturers;
pub mod packages;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Driver {
    pub manifest_version: semver::Version,
    pub meta: Meta,
    #[serde(skip_serializing_if = "DevBoards::is_empty", default)]
    pub dev_boards: DevBoards,
    #[serde(skip_serializing_if = "Interfaces::is_empty", default)]
    pub interfaces: Interfaces,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub resources: Vec<Resource>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Meta {
    pub names: Vec<String>,
    pub manufacturer: manufacturers::Manufacturer,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub datasheets: Vec<Url>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub categories: Vec<categories::Category>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub part_numbers: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub kicad_symbol: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub packages: Vec<packages::Package>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DevBoards {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub adafruit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub sparkfun: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub mikroe: Option<u32>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub other: Vec<GenericDevBoard>,
}

impl DevBoards {
    pub fn is_empty(&self) -> bool {
        match self {
            DevBoards {
                adafruit: None,
                sparkfun: None,
                mikroe: None,
                other: o,
            } if o.is_empty() => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct GenericDevBoard {
    pub name: String,
    pub link: url::Url,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Interfaces {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub i2c: Option<I2c>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub spi: Option<Spi>,
}

impl Interfaces {
    pub fn is_empty(&self) -> bool {
        match self {
            Interfaces {
                i2c: None,
                spi: None,
            } => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct I2c {
    pub addrs: Vec<u8>,
    pub interrupt: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Spi {
    pub bus_type: SpiDeviceType,
    pub interrupt: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum SpiDeviceType {
    SpiBus,
    SpiDevice,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Resource {
    #[serde(rename = "type")]
    pub ty: ResourceType,
    pub link: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum ResourceType {
    BlogPost,
}
