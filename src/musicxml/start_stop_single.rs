use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(Debug, EnumString, PartialEq, Serialize, Deserialize, Default, PartialOrd)]
pub enum StartStopSingle {
    #[strum(serialize = "start")]
    #[serde(rename = "start")]
    #[default]
    Start,

    #[strum(serialize = "stop")]
    #[serde(rename = "stop")]
    Stop,

    #[strum(serialize = "single")]
    #[serde(rename = "single")]
    Single,
}
