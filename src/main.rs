use std::collections::{BTreeMap, HashMap, HashSet};

use serde::{Deserialize, Serialize};

fn main() -> anyhow::Result<()> {
    let list = drivers::awesome_embedded_rust::from_csv("aer.csv")?;

    let component_abstractions = [
        "embedded-hal",
        "embedded-hal-async",
        "accelerometer",
        "embedded-graphics",
        "radio",
        "smart-leds",
        "usb-device",
        "atat",
        "embedded-nal",
        "embedded-storage",
        "switch-hal",
    ];

    let crates = list
        .0
        .iter()
        .map(|entry| entry.name.to_lowercase())
        .collect::<Vec<_>>();

    let mut crate_name_to_id = HashMap::new();
    let mut crate_id_to_name = HashMap::new();
    let mut crate_rows = vec![];

    println!("Loading crate info");
    db_dump::Loader::new()
        .crates(|row| {
            if crates.contains(&row.name) || component_abstractions.contains(&row.name.as_str()) {
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

    let mut dependencies = HashMap::new();
    let mut dependency_crate_ids = HashSet::new();

    println!("Loading dependency info");
    db_dump::Loader::new()
        .dependencies(|row| {
            if versions.contains_key(&row.version_id) {
                dependency_crate_ids.insert(row.crate_id);
                dependencies.insert(row.version_id, row);
            }
        })
        .load("db-dump.tar.gz")?;

    println!("Adding dependencies to crate db");
    db_dump::Loader::new()
        .crates(|row| {
            if dependency_crate_ids.contains(&row.id) {
                crate_name_to_id.entry(row.name.clone()).or_insert(row.id);
                crate_id_to_name.entry(row.id).or_insert(row.name.clone());
                crate_rows.push(row);
            }
        })
        .load("db-dump.tar.gz")?;

    let db: Vec<_> = component_abstractions
        .iter()
        .map(|name| {
            let row = crate_rows.iter().find(|row| &row.name == name).unwrap();
            let crate_id = row.id;
            let downloads = crate_downloads.get(&row.id).copied().unwrap();
            let versions = versions
                .values()
                .filter(|row| row.crate_id == crate_id)
                .map(|row| {
                    let version_id = row.id;

                    let dependencies = dependencies
                        .values()
                        .filter(|dep| dep.version_id == version_id)
                        .map(|dep| {
                            let crate_name = crate_id_to_name.get(&crate_id).unwrap();
                            let target = if dep.target.is_empty() {
                                None
                            } else {
                                Some(dep.target.clone())
                            };
                            Dependency {
                                name: crate_name.clone(),
                                req: dep.req.clone(),
                                optional: dep.optional,
                                kind: dep.kind.into(),
                                default_features: dep.default_features,
                                features: dep.features.clone(),
                                target,
                            }
                        })
                        .collect();

                    Version {
                        version: row.num.clone(),
                        downloads: row.downloads,
                        features: row.features.clone(),
                        yanked: row.yanked,
                        license: row.license.clone(),
                        crate_size: row.crate_size,
                        rust_version: row.rust_version.clone(),
                        dependencies,
                    }
                })
                .collect();

            Crate {
                name: name.to_string(),
                downloads,
                versions,
            }
        })
        .collect();

    println!("{}", serde_json::to_string_pretty(&db)?);

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum DependencyKind {
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
struct Dependency {
    name: String,
    req: semver::VersionReq,
    optional: bool,
    kind: DependencyKind,
    default_features: bool,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    features: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    target: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Version {
    version: semver::Version,
    downloads: u64,
    #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
    features: BTreeMap<String, Vec<String>>,
    yanked: bool,
    license: String,
    crate_size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    rust_version: Option<semver::Version>,
    dependencies: Vec<Dependency>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Crate {
    name: String,
    downloads: u64,
    versions: Vec<Version>,
}
