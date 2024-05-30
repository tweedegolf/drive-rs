use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use super::{Interface, WebsiteCrate};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index<T: Ord>(pub BTreeMap<T, BTreeSet<usize>>);

impl<T: Ord + Clone> Index<T> {
    fn new() -> Self {
        Self(BTreeMap::new())
    }

    fn add(&mut self, key: T, value: usize) {
        self.0.entry(key).or_default().insert(value);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Indexes {
    pub license: Index<String>,
    pub rust_version: Index<String>,
    pub dependencies: Index<String>,
    pub interfaces: Index<Interface>,
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
            license,
            rust_version,
            dependencies,
            interfaces,
        }
    }
}
