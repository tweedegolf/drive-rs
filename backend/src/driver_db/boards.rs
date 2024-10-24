use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct DevBoard {
    name: String,
    manufacturer: BoardManufacturer,
    link: Url,
    connections: Vec<Connection>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum BoardManufacturer {
    Adafruit,
    Sparkfun,
    Mikroe,
    #[serde(rename = "BBC")]
    Bbc,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum Connection {
    StemmaQt,
    MikroBus,
}
