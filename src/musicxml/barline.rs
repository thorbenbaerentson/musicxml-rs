use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

use super::core::RepeatDirection;
use super::left_right_middle::LeftRightMiddle;
use super::printable_value::PrintableValue;
use crate::prelude::*;
use std::str::FromStr;

// https://www.w3.org/2021/06/musicxml40/musicxml-reference/data-types/bar-style/
#[derive(Debug, EnumString, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum BarStyle {
    #[strum(serialize = "standard")]
    #[serde(rename = "standard")]
    Standard,

    #[strum(serialize = "dashed")]
    #[serde(rename = "dashed")]
    Dashed,

    #[strum(serialize = "dotted")]
    #[serde(rename = "dotted")]
    Dotted,

    #[strum(serialize = "heavy")]
    #[serde(rename = "heavy")]
    Heavy,

    #[strum(serialize = "heavy-heavy")]
    #[serde(rename = "heavy-heavy")]
    HeavyHeavy,

    #[strum(serialize = "heavy-light")]
    #[serde(rename = "heavy-light")]
    HeavyLight,

    #[strum(serialize = "light-heavy")]
    #[serde(rename = "light-heavy")]
    LightHeavy,

    #[strum(serialize = "light-light")]
    #[serde(rename = "light-light")]
    LightLight,

    #[strum(serialize = "none")]
    #[serde(rename = "none")]
    None,

    #[strum(serialize = "regular")]
    #[serde(rename = "regular")]
    Regular,

    #[strum(serialize = "short")]
    #[serde(rename = "short")]
    Short,

    #[strum(serialize = "tick")]
    #[serde(rename = "tick")]
    Tick,
}

// https://www.w3.org/2021/06/musicxml40/musicxml-reference/elements/barline/
#[derive(Debug, Serialize, Deserialize)]
pub struct Barline {
    #[serde(rename = "bar-style", default = "Option::default")]
    pub barstyle: Option<BarStyle>,

    #[serde(default = "Option::default")]
    pub footnote: Option<PrintableValue<String>>,

    pub location: LeftRightMiddle,
    pub repeatdirection: Option<RepeatDirection>,
}

#[cfg(test)]
mod tests {
    use super::Barline;
    use crate::musicxml::barline::{BarStyle, LeftRightMiddle};
    use serde_xml_rs::from_str;

    #[test]
    fn barline() {
        let mut xml = r#"
            <barline location="middle">
                <bar-style>dashed</bar-style>
            </barline>"#;
        let barline: Barline = from_str(xml).unwrap();

        assert_eq!(barline.location, LeftRightMiddle::Middle);
        assert_eq!(barline.barstyle.unwrap(), BarStyle::Dashed);

        xml = r#"
            <barline location="right">
                <bar-style>light-light</bar-style>
            </barline>"#;
        let barline: Barline = from_str(xml).unwrap();

        assert_eq!(barline.location, LeftRightMiddle::Right);
        assert_eq!(barline.barstyle.unwrap(), BarStyle::LightLight);
    }
}
