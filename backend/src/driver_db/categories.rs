use std::{
    borrow::Cow,
    collections::BTreeMap,
    fmt::{Debug, Display},
    str::FromStr,
};

use anyhow::bail;
use schemars::{json_schema, JsonSchema};
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
    Timer,

    #[serde(rename = "Timer::RTC")]
    Rtc,
}
