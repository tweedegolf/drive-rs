use super::{Interface, WebsiteCrate};
use crate::driver_db::categories::Category;
use crate::driver_db::packages::PackageType;
use crate::driver_db::Interfaces;
use crate::FullCrate;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Index<T: Ord>(pub BTreeMap<T, BTreeSet<usize>>);

impl<T: Ord + Clone> Index<T> {
    fn new() -> Self {
        Self(BTreeMap::new())
    }

    fn add(&mut self, key: T, value: usize) {
        self.0.entry(key).or_default().insert(value);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Indexes {
    pub category: Index<Category>,
    pub license: Index<String>,
    pub rust_version: Index<String>,
    pub dependencies: Index<String>,
    pub interfaces: Index<Interface>,
    pub package: Index<PackageType>,
    pub has_kicad: BTreeSet<usize>,
    pub has_dev_board: BTreeSet<usize>,
}

impl From<&[FullCrate]> for Indexes {
    fn from(value: &[FullCrate]) -> Self {
        let mut category = Index::new();
        let mut license = Index::new();
        let mut rust_version = Index::new();
        let mut dependencies = Index::new();
        let mut interfaces = Index::new();
        let mut package = Index::new();
        let mut has_kicad = BTreeSet::new();
        let mut has_dev_board = BTreeSet::new();

        for (i, krate) in value.iter().enumerate() {
            for cat in &krate.chip_meta.categories {
                category.add(*cat, i);

                for parent in cat.parents() {
                    category.add(parent, i);
                }
            }

            for l in krate.licenses() {
                license.add(l, i);
            }

            rust_version.add(
                krate
                    .rust_version
                    .as_ref()
                    .map(|v| v.to_string())
                    .unwrap_or(String::new()),
                i,
            );

            for dep in &krate.dependencies {
                dependencies.add(dep.to_string(), i);
            }

            let Interfaces { i2c, spi } = &krate.interfaces;
            if i2c.is_some() {
                interfaces.add(Interface::I2C, i);
            }
            if spi.is_some() {
                interfaces.add(Interface::SPI, i);
            }

            for p in &krate.chip_meta.packages {
                package.add(p.ty, i);
            }

            if !krate.chip_meta.kicad_symbol.is_empty() {
                has_kicad.insert(i);
            }

            if !krate.dev_boards.is_empty() {
                has_dev_board.insert(i);
            }
        }

        Self {
            category,
            license,
            rust_version,
            dependencies,
            interfaces,
            package,
            has_kicad,
            has_dev_board,
        }
    }
}

impl From<&[WebsiteCrate]> for Indexes {
    fn from(value: &[WebsiteCrate]) -> Self {
        let mut license = Index::new();
        let mut rust_version = Index::new();
        let mut dependencies = Index::new();
        let mut interfaces = Index::new();

        for (i, krate) in value.iter().enumerate() {
            for l in krate.licenses() {
                license.add(l, i);
            }

            rust_version.add(
                krate
                    .rust_version
                    .as_ref()
                    .map(|v| v.to_string())
                    .unwrap_or(String::new()),
                i,
            );

            for dep in &krate.dependencies {
                dependencies.add(dep.to_string(), i);
            }

            for interface in &krate.interfaces {
                interfaces.add(*interface, i);
            }
        }

        Self {
            category: Index::new(),
            license,
            rust_version,
            dependencies,
            interfaces,
            package: Index::new(),
            has_kicad: BTreeSet::new(),
            has_dev_board: BTreeSet::new(),
        }
    }
}
