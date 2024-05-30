use std::collections::BTreeMap;

use drivers::dumpsterbase::CrateDb;
use drivers::website_db::{Interface, WebsiteDb};

fn main() -> anyhow::Result<()> {
    let list = drivers::awesome_embedded_rust::from_csv("aer.csv")?;

    let interfaces = list
        .0
        .iter()
        .map(|entry| {
            (
                entry.name.to_lowercase(),
                entry
                    .interface
                    .split(['/', '+'])
                    .map(str::trim)
                    .map(str::parse::<Interface>)
                    .filter_map(Result::ok)
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<BTreeMap<_, _>>();

    let crates = list
        .0
        .iter()
        .map(|entry| entry.name.to_lowercase())
        .collect::<Vec<_>>();

    let db = CrateDb::from_dump(crates)?;

    let website_db = WebsiteDb::from_crates_and_interfaces(db, &interfaces);

    std::fs::write("website_db.json", serde_json::to_string(&website_db)?)?;

    Ok(())
}
