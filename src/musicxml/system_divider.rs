use serde::{Deserialize, Serialize};
use super::{
    printable_value::{FontStyle, FontWeight, LeftCenterRight, Valign},
    yes_no::YesNo,
};

// https://www.w3.org/2021/06/musicxml40/musicxml-reference/elements/left-divider/
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Default)]
pub struct SystemDivider {
    #[serde(default = "Option::default")]
    pub color: Option<String>,

    #[serde(rename = "default-x", default = "Option::default")]
    pub default_x: Option<f32>,

    #[serde(rename = "default-y", default = "Option::default")]
    pub default_y: Option<f32>,

    #[serde(rename = "font-family", default = "Option::default")]
    pub font_family: Option<String>,

    #[serde(rename = "font-size", default = "Option::default")]
    pub font_size: Option<f32>,

    #[serde(rename = "font-style", default = "Option::default")]
    pub font_style: Option<FontStyle>,

    #[serde(rename = "font-weight", default = "Option::default")]
    pub font_weight: Option<FontWeight>,

    #[serde(rename = "halign", default = "Option::default")]
    pub halign: Option<LeftCenterRight>,

    #[serde(rename = "print-object", default = "Option::default")]
    pub print_object: Option<YesNo>,

    #[serde(rename = "relative-x", default = "Option::default")]
    pub relative_x: Option<String>,

    #[serde(rename = "relative-y", default = "Option::default")]
    pub relative_y: Option<String>,

    #[serde(rename = "valign", default = "Option::default")]
    pub valign: Option<Valign>,
}
