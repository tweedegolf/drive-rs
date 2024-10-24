use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use anyhow::{bail, Context};
use schemars::{json_schema, JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
pub struct Package {
    pub ty: PackageType,
    pub pins: u8,
}

impl FromStr for Package {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ty, pins) = s.split_once("-").context("Package is missing pin count")?;
        let ty = ty.parse()?;
        let pins = pins.parse()?;
        Ok(Package { ty, pins })
    }
}

impl Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}-{}", self.ty, self.pins)
    }
}

impl Serialize for Package {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Package {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl JsonSchema for Package {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Package".into()
    }

    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let packages = PackageType::all()
            .iter()
            .map(|ty| ty.to_string())
            .collect::<Vec<_>>()
            .join("|");
        let regex = format!("^({packages})-\\d+$");
        json_schema!({"type": "string", "pattern": regex})
    }
}

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize, JsonSchema,
)]
pub enum PackageType {
    TSSOP,
    LGA,
    HVQFN,
    SOIC,
    SOT,
    SSOP,
    SO,
    PDIP,
}

impl PackageType {
    pub fn all() -> &'static [Self] {
        &[
            PackageType::TSSOP,
            PackageType::LGA,
            PackageType::HVQFN,
            PackageType::SOIC,
            PackageType::SOT,
            PackageType::SSOP,
            PackageType::SO,
            PackageType::PDIP,
        ]
    }
}

impl FromStr for PackageType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TSSOP" => Ok(PackageType::TSSOP),
            "LGA" => Ok(PackageType::LGA),
            "HVQFN" => Ok(PackageType::HVQFN),
            "SOIC" => Ok(PackageType::SOIC),
            "SOT" => Ok(PackageType::SOT),
            "SSOP" => Ok(PackageType::SSOP),
            "SO" => Ok(PackageType::SO),
            "PDIP" => Ok(PackageType::PDIP),
            _ => bail!("Unknown package type: {s}"),
        }
    }
}

impl Display for PackageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
