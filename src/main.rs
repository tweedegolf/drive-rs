use std::collections::{BTreeMap, BTreeSet};

use drivers::dumpsterbase::{CrateDb, DependencyKind};

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
            let mut versions: Vec<_> = d.versions.iter().map(|v| v.version.clone()).collect();
            versions.sort_unstable();
            versions.reverse();
            (d.name.clone(), versions)
        })
        .collect();

    let deps = db
        .crates
        .iter()
        .flat_map(|c| &c.versions)
        .flat_map(|v| &v.dependencies)
        // .filter(|d| component_abstractions.contains(&d.name.as_str()))
        .filter(|d| d.kind == DependencyKind::Normal && !d.optional)
        .map(|d| {
            (
                d.name.clone(),
                // Get the newest version that satisfies the requirement
                find_version(deps[d.name.as_str()].as_slice(), &d.req).unwrap(),
            )
        })
        .collect::<BTreeSet<_>>();

    for (name, version) in deps {
        println!("{}@{}", name, version);
    }

    Ok(())
}

fn find_version(versions: &[semver::Version], req: &semver::VersionReq) -> Option<semver::Version> {
    versions.iter().find(|v| req.matches(v)).cloned()
}
