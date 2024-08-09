use anyhow::bail;
use drivers::driver_db::{Driver, I2c, Interfaces, Spi, SpiDeviceType};
use drivers::website_db::indexes::Indexes;
use drivers::{dumpsterbase, FullCrate};
use schemars::{schema_for, JsonSchema};
use semver::Version;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{ffi::OsStr, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct FullCrateDb {
    crates: Vec<FullCrate>,
    indexes: Indexes,
}

fn main() -> anyhow::Result<()> {
    // Write out schema for easier crate description
    let schema = schema_for!(Driver);
    std::fs::write(
        "driver-db-schema.json",
        serde_json::to_string_pretty(&schema)?,
    )?;

    // Read old awesome embedded Rust list
    let list = drivers::awesome_embedded_rust::from_csv("aer.csv")?;
    let old_drivers: HashMap<String, Driver> = list
        .0
        .into_iter()
        .map(|e| {
            (
                e.name.to_lowercase(),
                Driver {
                    manifest_version: Version::new(0, 0, 0),
                    meta: Default::default(),
                    dev_boards: Default::default(),
                    interfaces: Interfaces {
                        i2c: e.interface.contains("I2C").then_some(I2c {
                            addrs: vec![],
                            interrupt: false,
                        }),
                        spi: e.interface.contains("SPI").then_some(Spi {
                            bus_type: SpiDeviceType::SpiBus,
                            interrupt: false,
                        }),
                    },
                    resources: vec![],
                },
            )
        })
        .collect();

    // Read all drivers we have listed
    let new_drivers = read_all("driver-db".as_ref())?;
    println!("Found {} drivers", new_drivers.len());

    let mut drivers = HashMap::new();
    drivers.extend(old_drivers);
    drivers.extend(new_drivers);

    // Fetch info from crates.io
    let crates = drivers.keys().cloned().collect();
    let crate_db = dumpsterbase::CrateDb::from_dump(crates)?;
    let mut crates: HashMap<_, _> = crate_db
        .crates
        .into_iter()
        .map(|krate| (krate.name.clone(), krate))
        .collect();

    // Write out the crate db
    let mut output = Vec::with_capacity(drivers.len());
    for (name, driver) in drivers.into_iter() {
        let krate = match crates.remove(&name) {
            Some(krate) => krate,
            None => {
                eprintln!("No crate found for driver: {}", name);
                continue;
            }
        };
        let full = match FullCrate::new(driver, krate) {
            Ok(full) => full,
            Err(e) => {
                eprintln!("Error creating full crate: {e}");
                continue;
            }
        };

        output.push(full);
    }

    let indexes = Indexes::from(output.as_slice());
    let full_output = FullCrateDb {
        crates: output,
        indexes,
    };

    std::fs::write(
        "full-crate-db.json",
        serde_json::to_string_pretty(&full_output)?,
    )?;

    // Write out schema
    std::fs::write(
        "full-crate-db-schema.json",
        serde_json::to_string_pretty(&schema_for!(FullCrateDb))?,
    )?;

    Ok(())
}

fn read_all(dir: &Path) -> anyhow::Result<HashMap<String, Driver>> {
    let mut drivers = HashMap::new();
    for info in std::fs::read_dir(dir)? {
        match parse_crate(info?.path().as_path()) {
            Ok((name, driver)) => {
                drivers.insert(name, driver);
            }
            Err(e) => {
                eprintln!("Error parsing driver: {e}");
            }
        }
    }
    Ok(drivers)
}

pub fn parse_crate(path: &Path) -> anyhow::Result<(String, Driver)> {
    if !path
        .extension()
        .is_some_and(|ext| ext == OsStr::new("toml"))
    {
        bail!("Driver info has wrong exctension: {path:?}");
    }

    let Some(crate_name) = path.file_stem() else {
        bail!("Driver info has no file stem: {path:?}");
    };

    let driver_info =
        toml::from_str::<drivers::driver_db::Driver>(&std::fs::read_to_string(&path)?)?;

    Ok((crate_name.to_string_lossy().into_owned(), driver_info))
}
