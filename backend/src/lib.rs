use crate::driver_db::{DevBoards, Driver, Interfaces, Resource};
use crate::dumpsterbase::Version;
use anyhow::Context;
use chrono::{DateTime, Utc};
use schemars::{json_schema, JsonSchema, Schema, SchemaGenerator};
use serde::{Deserialize, Serialize};
use spdx::ParseMode;
use std::borrow::Cow;
use std::fmt::Display;

pub mod driver_db;

mod description;

pub mod awesome_embedded_rust;

pub mod dumpsterbase;

pub mod website_db;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FullCrate {
    pub name: String,
    pub version: semver::Version,
    pub downloads: u64,
    pub this_version_downloads: u64,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub homepage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub documentation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub repository: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(flatten)]
    pub chip_meta: driver_db::Meta,
    #[serde(skip_serializing_if = "DevBoards::is_empty", default)]
    pub dev_boards: DevBoards,
    #[serde(skip_serializing_if = "Interfaces::is_empty", default)]
    pub interfaces: Interfaces,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub resources: Vec<Resource>,
    pub license: String,
    pub crate_size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub rust_version: Option<semver::Version>,
    pub dependencies: Vec<ShortDependency>,
}

impl FullCrate {
    pub fn new(driver_db: Driver, mut krate: dumpsterbase::Crate) -> anyhow::Result<Self> {
        let version = krate
            .versions
            .pop()
            .with_context(|| format!("No versions found for crate {:?}", krate.name))?;

        Ok(Self {
            name: krate.name,
            version: version.version,
            downloads: krate.downloads,
            this_version_downloads: version.downloads,
            description: krate.description,
            homepage: krate.homepage,
            documentation: krate.documentation,
            repository: krate.repository,
            created_at: krate.created_at,
            updated_at: krate.updated_at,
            chip_meta: driver_db.meta,
            dev_boards: driver_db.dev_boards,
            interfaces: driver_db.interfaces,
            resources: driver_db.resources,
            license: version.license,
            crate_size: version.crate_size,
            rust_version: version.rust_version,
            dependencies: version
                .dependencies
                .into_iter()
                .map(ShortDependency::try_from)
                .collect::<Result<_, _>>()?,
        })
    }

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

#[derive(Debug, Clone, PartialEq)]
pub struct ShortDependency {
    pub name: String,
    pub version: semver::Version,
}

impl TryFrom<dumpsterbase::Dependency> for ShortDependency {
    type Error = anyhow::Error;

    fn try_from(value: dumpsterbase::Dependency) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name.clone(),
            version: value.newest_version.with_context(|| {
                format!(
                    "No matching version found for crate {:?} with requirement {:?}",
                    value.name, value.req,
                )
            })?,
        })
    }
}

impl Display for ShortDependency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{}", self.name, self.version)
    }
}

impl serde::Serialize for ShortDependency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&self)
    }
}

impl<'de> serde::Deserialize<'de> for ShortDependency {
    fn deserialize<D>(deserializer: D) -> Result<ShortDependency, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let (name, version) = s
            .split_once('@')
            .ok_or_else(|| serde::de::Error::custom("Missing '@'"))?;
        Ok(ShortDependency {
            name: name.to_string(),
            version: semver::Version::parse(version).map_err(serde::de::Error::custom)?,
        })
    }
}

impl JsonSchema for ShortDependency {
    fn schema_name() -> Cow<'static, str> {
        "ShortDependency".into()
    }

    fn json_schema(_gen: &mut SchemaGenerator) -> Schema {
        json_schema!({"type": "string", "pattern": r"^[^@]+@(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$"})
    }
}
