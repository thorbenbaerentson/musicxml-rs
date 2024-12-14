use serde::{Deserialize, Serialize};

use super::core::DirectionUD;

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Stem {
    #[serde(rename = "$value", default = "DirectionUD::default")]
    pub content: DirectionUD,

    #[serde(default = "Option::default")]
    pub color: Option<String>,

    #[serde(rename = "default-x", default = "Option::default")]
    pub default_x: Option<f32>,

    #[serde(rename = "default-y", default = "Option::default")]
    pub default_y: Option<f32>,

    #[serde(rename = "relative-x", default = "Option::default")]
    pub relative_x: Option<String>,

    #[serde(rename = "relative-y", default = "Option::default")]
    pub relative_y: Option<String>,
}
