use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use url::Url;

mod categories;
mod manufacturers;
mod packages;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Driver {
    pub manifest_version: semver::Version,
    pub meta: Meta,
    #[serde(skip_serializing_if = "DevBoards::is_empty", default)]
    pub dev_boards: DevBoards,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub i2c: Option<I2c>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub spi: Option<Spi>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub resources: Vec<Resource>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
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
    pub used_in: Vec<String>,
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
}

impl DevBoards {
    fn is_empty(&self) -> bool {
        self.adafruit.is_none()
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
    pub can_share_bus: bool,
    pub interrupt: bool,
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
