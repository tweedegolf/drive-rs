use drivers::website_db::indexes::Indexes;
use drivers::FullCrate;
use std::fs;

fn main() -> anyhow::Result<()> {
    let full_crates = fs::read_to_string("full-crate-db.json")?;
    let full_crates: Vec<FullCrate> = serde_json::from_str(&full_crates)?;

    let indecies = Indexes::from(full_crates.as_slice());

    println!("{}", serde_json::to_string_pretty(&indecies)?);

    Ok(())
}
