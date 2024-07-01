use anyhow::bail;
use drivers::driver_db::Driver;
use drivers::{dumpsterbase, FullCrate};
use schemars::schema_for;
use std::collections::HashMap;
use std::{ffi::OsStr, path::Path};

fn main() -> anyhow::Result<()> {
    // Write out schema for easier crate description
    let schema = schema_for!(Driver);
    std::fs::write(
        "driver-db-schema.json",
        serde_json::to_string_pretty(&schema)?,
    )?;

    // Read all drivers we have listed
    let drivers = read_all("driver-db".as_ref())?;
    println!("Found {} drivers", drivers.len());

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
    std::fs::write("full-crate-db.json", serde_json::to_string_pretty(&output)?)?;

    // Write out schema
    std::fs::write(
        "full-crate-db-schema.json",
        serde_json::to_string_pretty(&schema_for!(Vec<FullCrate>))?,
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
