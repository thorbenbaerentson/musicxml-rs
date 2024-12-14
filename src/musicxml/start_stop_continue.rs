use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(Debug, EnumString, PartialEq, Serialize, Deserialize, PartialOrd, Default)]
pub enum StartStopContinue {
    #[strum(serialize = "start")]
    #[serde(rename = "start")]
    #[default]
    Start,

    #[strum(serialize = "stop")]
    #[serde(rename = "stop")]
    Stop,

    #[strum(serialize = "continue")]
    #[serde(rename = "continue")]
    Continue,
}
