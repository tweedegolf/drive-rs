use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use spdx::ParseMode;

use crate::dumpsterbase::{Crate, CrateDb};
use crate::website_db::indexes::Indexes;

pub mod indexes;

#[derive(
    Debug, Copy, Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, JsonSchema,
)]
pub enum Interface {
    I2C,
    SPI,
    UART,
    GPIO,
    OneWire,
    ParallelPort,
    Usb,
}

impl FromStr for Interface {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "I2C" => Ok(Interface::I2C),
            "SPI" => Ok(Interface::SPI),
            "UART" | "USART" | "SERIAL" => Ok(Interface::UART),
            "DIO" | "GPIO" => Ok(Interface::GPIO),
            "1WIRE" | "1-WIRE" => Ok(Interface::OneWire),
            "PARALLEL PORT" => Ok(Interface::ParallelPort),
            "USB" => Ok(Interface::Usb),
            _ => Err(s.into()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct ShortDependency {
    name: String,
    version: Option<semver::Version>,
}

impl Display for ShortDependency {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;

        if let Some(version) = &self.version {
            write!(f, "@{}", version)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsiteCrate {
    pub name: String,
    pub description: String,
    pub version: semver::Version,
    pub license: String,
    pub downloads: u64,
    pub interfaces: Vec<Interface>,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub rust_version: Option<semver::Version>,
    pub dependencies: Vec<ShortDependency>,
}

impl WebsiteCrate {
    pub fn licenses(&self) -> Vec<String> {
        match spdx::Expression::parse_mode(&self.license, ParseMode::LAX) {
            Ok(expr) => expr
                .requirements()
                .map(|req| req.req.to_string())
                .collect::<Vec<_>>(),
            Err(_) => vec!["Other".into()],
        }
    }
}

impl WebsiteCrate {
    fn from_crate_and_interface_db(
        value: Crate,
        interfaces: &BTreeMap<String, Vec<Interface>>,
    ) -> Self {
        let newest_version = value
            .versions
            .into_iter()
            .max_by_key(|v| v.version.clone())
            .unwrap();

        let interfaces = interfaces.get(&value.name).cloned().unwrap_or_default();

        Self {
            name: value.name,
            description: value.description,
            version: newest_version.version,
            license: newest_version.license,
            downloads: value.downloads,
            interfaces,
            updated_at: value.updated_at,
            created_at: value.created_at,
            rust_version: newest_version.rust_version,
            dependencies: newest_version
                .dependencies
                .into_iter()
                .map(|d| ShortDependency {
                    name: d.name,
                    version: d.newest_version,
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsiteDb {
    crates: Vec<WebsiteCrate>,
    indexes: indexes::Indexes,
}

impl WebsiteDb {
    pub fn from_crates_and_interfaces(
        value: CrateDb,
        interfaces: &BTreeMap<String, Vec<Interface>>,
    ) -> Self {
        let crates: Vec<_> = value
            .crates
            .into_iter()
            .map(|c| WebsiteCrate::from_crate_and_interface_db(c, interfaces))
            .collect();
        let indexes = Indexes::from(crates.as_slice());

        Self { crates, indexes }
    }
}
