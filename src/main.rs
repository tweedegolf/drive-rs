use std::collections::{BTreeMap, BTreeSet};

use drivers::dumpsterbase::CrateDb;

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

    let db = CrateDb::from_dump(crates)?;

    let deps: BTreeMap<_, Vec<_>> = db
        .dependenants
        .iter()
        .map(|d| {
            (
                d.name.clone(),
                d.versions.iter().map(|v| v.version.to_string()).collect(),
            )
        })
        .collect();

    dbg!(deps);

    let deps = db
        .crates
        .iter()
        .flat_map(|c| &c.versions)
        .flat_map(|v| &v.dependencies)
        .filter(|d| component_abstractions.contains(&d.name.as_str()))
        .map(|d| format!("{}@{}", d.name, d.req))
        .collect::<BTreeSet<_>>();

    println!("{deps:?}");

    Ok(())
}
