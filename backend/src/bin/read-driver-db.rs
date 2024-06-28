use std::{ffi::OsStr, path::Path};

use anyhow::bail;
use drivers::driver_db::{self, Driver};
use schemars::schema_for;

fn main() -> anyhow::Result<()> {
    let schema = schema_for!(driver_db::Driver);
    std::fs::write("schema.json", serde_json::to_string_pretty(&schema)?)?;

    for info in std::fs::read_dir("driver-db/")? {
        match parse_crate(info?.path().as_path()) {
            Ok((name, driver)) => {
                println!("{}: {:?}", name, driver);
            }
            Err(e) => {
                eprintln!("Error parsing driver: {e}");
            }
        }
    }

    Ok(())
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
