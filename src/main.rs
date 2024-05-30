use drivers::dumpsterbase::CrateDb;
use drivers::website_db::WebsiteDb;

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

    let website_db = WebsiteDb::from(db);

    std::fs::write("website_db.json", serde_json::to_string(&website_db)?)?;

    Ok(())
}
