use std::fmt::{Display, Formatter};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use spdx::ParseMode;

use crate::dumpsterbase::{Crate, CrateDb};
use crate::website_db::indexes::Indexes;

pub mod indexes;

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

impl From<Crate> for WebsiteCrate {
    fn from(value: Crate) -> Self {
        let newest_version = value
            .versions
            .into_iter()
            .max_by_key(|v| v.version.clone())
            .unwrap();

        Self {
            name: value.name,
            description: value.description,
            version: newest_version.version,
            license: newest_version.license,
            downloads: value.downloads,
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

impl From<CrateDb> for WebsiteDb {
    fn from(value: CrateDb) -> Self {
        let crates: Vec<_> = value.crates.into_iter().map(WebsiteCrate::from).collect();
        let indexes = Indexes::from(crates.as_slice());

        Self { crates, indexes }
    }
}
