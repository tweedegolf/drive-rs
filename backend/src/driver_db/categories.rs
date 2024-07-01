use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    str::FromStr,
};

use anyhow::bail;
use schemars::{json_schema, JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum Category {
    Analog(Option<Analog>),
    Sensor(Option<Sensor>),
    IoExpander(Option<IoExpander>),
}

impl FromStr for Category {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s
            .split_once("::")
            .map(|(prefix, rest)| (prefix, Some(rest)))
            .unwrap_or((s, None))
        {
            ("Analog", rest) => Ok(Category::Analog(rest.map(FromStr::from_str).transpose()?)),
            ("Sensor", rest) => Ok(Category::Sensor(rest.map(FromStr::from_str).transpose()?)),
            ("IoExpander", rest) => Ok(Category::IoExpander(
                rest.map(FromStr::from_str).transpose()?,
            )),
            _ => bail!("Unknown category: {s}"),
        }
    }
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (name, inner): (&str, Option<&dyn Display>) = match self {
            Category::Analog(ref inner) => ("Analog", opt_dyn(inner)),
            Category::Sensor(ref inner) => ("Sensor", opt_dyn(inner)),
            Category::IoExpander(ref inner) => ("IoExpander", opt_dyn(inner)),
        };

        if let Some(inner) = inner {
            write!(f, "{}::{}", name, inner)
        } else {
            write!(f, "{}", name)
        }
    }
}

impl Serialize for Category {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'de> Deserialize<'de> for Category {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Category::from_str(s.as_str()).map_err(serde::de::Error::custom)
    }
}

impl JsonSchema for Category {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        Cow::Borrowed("Category")
    }

    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        json_schema!({"enum": ["Analog", "Analog::ADC", "Analog::DAC", "Sensor", "Sensor::PowerMeter"]})
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum Analog {
    ADC,
    DAC,
}

impl FromStr for Analog {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ADC" => Ok(Analog::ADC),
            "DAC" => Ok(Analog::DAC),
            _ => bail!("Unknown analog category: {s}"),
        }
    }
}

impl Display for Analog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum Sensor {
    PowerMeter,
    Accelerometer,
    Gyroscope,
    Magnetometer,
}

impl FromStr for Sensor {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PowerMeter" => Ok(Sensor::PowerMeter),
            "Accelerometer" => Ok(Sensor::Accelerometer),
            "Gyroscope" => Ok(Sensor::Gyroscope),
            "Magnetometer" => Ok(Sensor::Magnetometer),
            _ => bail!("Unknown sensor category: {s}"),
        }
    }
}

impl Display for Sensor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum IoExpander {
    PWM,
}

impl FromStr for IoExpander {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PWM" => Ok(IoExpander::PWM),
            _ => bail!("Unknown io expander category: {s}"),
        }
    }
}

impl Display for IoExpander {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

fn opt_dyn(opt: &Option<impl Display>) -> Option<&dyn Display> {
    opt.as_ref().map(|i| i as &dyn Display)
}
