use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

// https://www.w3.org/2021/06/musicxml40/musicxml-reference/data-types/right-left-middle/
#[derive(Debug, EnumString, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum LeftRightMiddle {
    #[strum(serialize = "left")]
    #[serde(rename = "left")]
    Left,

    #[strum(serialize = "right")]
    #[serde(rename = "right")]
    Right,

    #[strum(serialize = "middle")]
    #[serde(rename = "middle")]
    Middle,
}
