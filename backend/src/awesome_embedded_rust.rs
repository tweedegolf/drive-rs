use std::path::Path;
use serde::{Deserialize};
use anyhow::Result;

pub fn from_csv(path: impl AsRef<Path>) -> Result<List> {
    let mut rdr = csv::Reader::from_path(path)?;
    Ok(List(rdr.deserialize::<Entry>().collect::<Result<_, _>>()?))
}

#[derive(Debug, Deserialize)]
pub struct List(pub Vec<Entry>);

#[derive(Debug, Deserialize)]
pub struct Entry {
    pub name: String,
    pub interface: String,
    pub description: String,
    pub links: String,
}
