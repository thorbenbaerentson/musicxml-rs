use serde::{Deserialize, Serialize};
use crate::prelude::*;
use super::{
    attributes::Attributes, barline::Barline, core::Duration, direction::Direction,
    harmony::Harmony, left_right_middle::LeftRightMiddle, level::Level, note::Note,
};

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum MarginType {
    #[serde(rename = "both")]
    #[default]
    Both,

    #[serde(rename = "even")]
    Even,

    #[serde(rename = "odd")]
    Odd,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PageMargins {
    #[serde(rename = "left-margin", default = "f32::default")]
    pub left_margin: f32,

    #[serde(rename = "right-margin", default = "f32::default")]
    pub right_margin: f32,

    #[serde(rename = "top-margin", default = "f32::default")]
    pub top_margin: f32,

    #[serde(rename = "bottom-margin", default = "f32::default")]
    pub bottom_margin: f32,

    #[serde(default = "Option::default")]
    pub r#type: Option<MarginType>,
}
