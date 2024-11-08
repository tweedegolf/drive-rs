use std::fmt::Debug;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize, JsonSchema,
)]
pub enum Category {
    // Analog
    /// Devices interacting with analog signals
    Analog,

    /// Analog to digital converters
    #[serde(rename = "Analog::ADC")]
    Adc,

    /// Digital to analog converters
    #[serde(rename = "Analog::DAC")]
    Dac,

    // Sensor
    /// Devices measuring things about their environment
    Sensor,

    /// Sensors measuring electric power
    #[serde(rename = "Sensor::PowerMeter")]
    PowerMeter,

    /// Sensors measuring acceleration
    ///
    /// These can also be used to determine where "down" is,
    /// using the gravitational acceleration.
    #[serde(rename = "Sensor::Accelerometer")]
    Accelerometer,

    /// Sensors measuring rotational acceleration
    #[serde(rename = "Sensor::Gyroscope")]
    Gyroscope,

    /// Sensors measuring air CO2 concentration
    #[serde(rename = "Sensor::CO2")]
    CO2,

    /// Sensors measuring air temperature
    #[serde(rename = "Sensor::Temperature")]
    Temperature,

    /// Sensors measuring air humidity
    #[serde(rename = "Sensor::Humidity")]
    Humidity,

    /// Sensors measuring magnetic fields
    ///
    /// These are commonly used as compasses, measuring the
    /// magnetic field of the earth.
    #[serde(rename = "Sensor::Magnetometer")]
    Magnetometer,

    // IO Expander
    /// Devices that provide more input and/or output signals
    IoExpander,

    /// Devices that provide PWM input and/or output signals
    #[serde(rename = "IoExpander::PWM")]
    PwmExpander,

    // Actor
    /// Things that move things in the real world
    Actor,

    /// Chips for driving motors
    #[serde(rename = "Actor::MotorController")]
    MotorController,

    // Display
    /// Optical displays
    Display,

    /// OLED Screens
    #[serde(rename = "Display::OLED")]
    Oled,

    // Timer
    /// Chips measuring time
    Timer,

    #[serde(rename = "Timer::RTC")]
    /// Clocks that keep track of wall time
    ///
    /// Often allow to measure time with an external battery.
    Rtc,
}

impl Category {
    /// Get a list of all categories that contain this category
    pub fn parents(&self) -> Vec<Self> {
        let mut parents = vec![];

        let mut me = *self;
        while let Some(parent) = me.parent() {
            parents.push(parent);
            me = parent;
        }

        parents
    }

    fn parent(&self) -> Option<Self> {
        Some(match self {
            Self::Analog => {
                return None;
            }
            Self::Adc => Self::Analog,
            Self::Dac => Self::Analog,
            Self::Sensor => {
                return None;
            }
            Self::PowerMeter => Self::Sensor,
            Self::Accelerometer => Self::Sensor,
            Self::Gyroscope => Self::Sensor,
            Self::Magnetometer => Self::Sensor,
            Category::CO2 => Self::Sensor,
            Category::Temperature => Self::Sensor,
            Category::Humidity => Self::Sensor,
            Self::IoExpander => {
                return None;
            }
            Self::PwmExpander => Self::IoExpander,
            Self::Actor => {
                return None;
            }
            Self::MotorController => Self::Actor,
            Self::Display => {
                return None;
            }
            Self::Oled => Self::Display,
            Self::Timer => {
                return None;
            }
            Self::Rtc => Self::Timer,
        })
    }
}
