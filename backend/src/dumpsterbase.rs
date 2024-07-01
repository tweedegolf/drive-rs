use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use spdx::ParseMode;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum DependencyKind {
    Normal,
    Build,
    Dev,
}

impl From<db_dump::dependencies::DependencyKind> for DependencyKind {
    fn from(value: db_dump::dependencies::DependencyKind) -> Self {
        match value {
            db_dump::dependencies::DependencyKind::Normal => DependencyKind::Normal,
            db_dump::dependencies::DependencyKind::Build => DependencyKind::Build,
            db_dump::dependencies::DependencyKind::Dev => DependencyKind::Dev,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub req: semver::VersionReq,
    pub newest_version: Option<semver::Version>,
    pub optional: bool,
    pub kind: DependencyKind,
    pub default_features: bool,
    // #[serde(skip_serializing_if = "Vec::is_empty", default)]
    // features: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub target: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    pub version: semver::Version,
    pub downloads: u64,
    // #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
    // features: BTreeMap<String, Vec<String>>,
    pub yanked: bool,
    pub license: String,
    pub crate_size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub rust_version: Option<semver::Version>,
    pub dependencies: Vec<Dependency>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Crate {
    pub name: String,
    pub downloads: u64,
    pub versions: Vec<Version>,
    pub description: String,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub repository: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrateDb {
    pub crates: Vec<Crate>,
    pub dependenants: Vec<Crate>,
}

impl CrateDb {
    pub fn licenses(&self) -> BTreeSet<String> {
        self.crates
            .iter()
            .flat_map(|krate| krate.versions.iter().map(|version| &version.license))
            .flat_map(
                |license| match spdx::Expression::parse_mode(license, ParseMode::LAX) {
                    Ok(expr) => expr
                        .requirements()
                        .map(|req| req.req.to_string())
                        .collect::<Vec<_>>(),
                    Err(_) => vec![],
                },
            )
            .collect()
    }
    pub fn from_dump(crate_names_of_interest: Vec<String>) -> anyhow::Result<CrateDb> {
        let mut crates = Self::load_crates(crate_names_of_interest, true)?;
        let dependenants = dependencies(&crates)?;

        let dep_versions: BTreeMap<_, Vec<_>> = dependenants
            .iter()
            .map(|d| {
                let mut versions: Vec<_> = d.versions.iter().map(|v| v.version.clone()).collect();
                versions.sort_unstable();
                versions.reverse();
                (d.name.clone(), versions)
            })
            .collect();

        for dependency in crates
            .iter_mut()
            .flat_map(|c| &mut c.versions)
            .flat_map(|v| &mut v.dependencies)
        {
            dependency.newest_version = dep_versions
                .get(&dependency.name)
                .and_then(|versions| versions.iter().find(|v| dependency.req.matches(v)))
                .cloned();
        }

        Ok(CrateDb {
            crates,
            dependenants,
        })
    }
    pub fn load_crates(crate_names: Vec<String>, only_newest: bool) -> anyhow::Result<Vec<Crate>> {
        let mut crate_name_to_id = HashMap::new();
        let mut crate_id_to_name = HashMap::new();
        let mut crate_rows = vec![];

        println!("Loading crate info");
        db_dump::Loader::new()
            .crates(|row| {
                if crate_names.contains(&row.name) {
                    crate_name_to_id.entry(row.name.clone()).or_insert(row.id);
                    crate_id_to_name.entry(row.id).or_insert(row.name.clone());
                    crate_rows.push(row);
                }
            })
            .load("db-dump.tar.gz")?;

        let mut default_versions = HashMap::new();
        let mut crate_downloads = HashMap::new();
        let mut versions = HashMap::new();

        println!("Loading version info");
        db_dump::Loader::new()
            .default_versions(|row| {
                if crate_id_to_name.contains_key(&row.crate_id) {
                    default_versions.insert(row.crate_id, row.version_id);
                }
            })
            .crate_downloads(|row| {
                if crate_id_to_name.contains_key(&row.crate_id) {
                    crate_downloads.insert(row.crate_id, row.downloads);
                }
            })
            .versions(|row| {
                if crate_id_to_name.contains_key(&row.crate_id) {
                    versions.insert(row.id, row);
                }
            })
            .load("db-dump.tar.gz")?;

        let mut dependencies: HashMap<_, Vec<_>> = HashMap::new();
        let mut dependency_crate_ids = HashSet::new();

        println!("Loading dependency info");
        db_dump::Loader::new()
            .dependencies(|row| {
                if versions.contains_key(&row.version_id) {
                    dependency_crate_ids.insert(row.crate_id);
                    dependencies.entry(row.version_id).or_default().push(row);
                }
            })
            .load("db-dump.tar.gz")?;

        db_dump::Loader::new()
            .crates(|row| {
                if dependency_crate_ids.contains(&row.id) {
                    crate_id_to_name.entry(row.id).or_insert(row.name.clone());
                }
            })
            .load("db-dump.tar.gz")?;

        let db: Vec<_> = crate_names
            .iter()
            .flat_map(|name| {
                let row = crate_rows.iter().find(|row| &row.name == name)?;
                let crate_id = row.id;
                let downloads = crate_downloads.get(&row.id).copied()?;
                let versions = versions
                    .values()
                    .filter(|row| row.crate_id == crate_id)
                    .filter(|row| !only_newest || row.id == default_versions[&crate_id])
                    .map(|row| {
                        let version_id = row.id;

                        let dependencies = dependencies
                            .get(&version_id)
                            .unwrap_or(&vec![])
                            .iter()
                            .map(|dep| {
                                let crate_name = crate_id_to_name.get(&dep.crate_id).unwrap();
                                let target = if dep.target.is_empty() {
                                    None
                                } else {
                                    Some(dep.target.clone())
                                };
                                Dependency {
                                    name: crate_name.clone(),
                                    req: dep.req.clone(),
                                    newest_version: None,
                                    optional: dep.optional,
                                    kind: dep.kind.into(),
                                    default_features: dep.default_features,
                                    // features: dep.features.clone(),
                                    target,
                                }
                            })
                            .collect();

                        Version {
                            version: row.num.clone(),
                            downloads: row.downloads,
                            // features: row.features.clone(),
                            yanked: row.yanked,
                            license: row.license.clone(),
                            crate_size: row.crate_size,
                            rust_version: row.rust_version.clone(),
                            dependencies,
                        }
                    })
                    .collect();

                Some(Crate {
                    name: name.to_string(),
                    downloads,
                    versions,
                    description: row.description.clone(),
                    homepage: row.homepage.clone(),
                    documentation: row.documentation.clone(),
                    repository: row.repository.clone(),
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                })
            })
            .collect();

        Ok(db)
    }
}

fn dependencies(crates: &[Crate]) -> anyhow::Result<Vec<Crate>> {
    let names = crates
        .iter()
        .flat_map(|c| &c.versions)
        .flat_map(|v| &v.dependencies)
        .map(|d| d.name.clone())
        .collect();

    CrateDb::load_crates(names, false)
}
