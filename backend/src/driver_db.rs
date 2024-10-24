use boards::DevBoard;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use url::Url;

pub mod boards;
pub mod categories;
pub mod manufacturers;
pub mod packages;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Driver {
    /// Version of this driver description TOML schema
    pub manifest_version: semver::Version,
    /// Metadata about the driver
    pub meta: Meta,
    /// List of development boards that house this chip
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub dev_boards: Vec<DevBoard>,
    /// Interfaces used by this chip
    #[serde(skip_serializing_if = "Interfaces::is_empty", default)]
    pub interfaces: Interfaces,
    /// Blog articles and similar covering this driver and its usage
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub resources: Vec<Resource>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Meta {
    /// Names of the chips this driver supports
    pub names: Vec<String>,
    /// Manufacturer that produces devices supported by this driver
    pub manufacturer: manufacturers::ChipManufacturer,
    /// Links to datasheets of chips that are supported by this driver
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub datasheets: Vec<Url>,
    /// Functionalities this driver provides
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub categories: Vec<categories::Category>,
    /// Part numbers of chips this driver supports
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub part_numbers: Vec<String>,
    /// Names of KiCAD symbold for chips this driver supports
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub kicad_symbol: Vec<String>,
    /// Packages or footprints in which chips are available
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub packages: Vec<packages::Package>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Interfaces {
    /// Information about the I2C interface (if present)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub i2c: Option<I2c>,
    /// Information about the SPI interface (if present)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub spi: Option<Spi>,
}

impl Interfaces {
    pub fn is_empty(&self) -> bool {
        matches!(
            self,
            Interfaces {
                i2c: None,
                spi: None,
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct I2c {
    /// Addresses that can be used by this device
    pub addrs: Vec<u8>,
    /// Does this device have an interrupt line?
    pub interrupt: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Spi {
    /// Whether this device can be used on a shared bus or only on an exclusive device
    pub bus_type: SpiDeviceType,
    /// Does this device have an interrupt line?
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
    pub title: String,
    pub link: Url,
}
