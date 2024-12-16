use super::{
    core::{Placement, SyllabicType},
    printable_value::LeftCenterRight,
    start_stop_continue::StartStopContinue,
    yes_no::YesNo,
};

use crate::prelude::*;
use roxmltree::NodeType;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Extend {
    #[serde(default = "Option::default")]
    color: Option<String>,

    #[serde(rename = "default-x", default = "Option::default")]
    default_x: Option<f32>,

    #[serde(rename = "default-y", default = "Option::default")]
    default_y: Option<f32>,

    #[serde(rename = "relative-x", default = "Option::default")]
    relative_x: Option<f32>,

    #[serde(rename = "relative-y", default = "Option::default")]
    relative_y: Option<f32>,

    #[serde(rename = "type", default = "Option::default")]
    r#type: Option<StartStopContinue>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Lyric {
    // Attributes
    #[serde(default = "Option::default")]
    pub color: Option<String>,

    #[serde(rename = "default-x", default = "Option::default")]
    pub default_x: Option<f32>,

    #[serde(rename = "default-y", default = "Option::default")]
    pub default_y: Option<f32>,

    #[serde(default = "Option::default")]
    pub id: Option<String>,

    #[serde(default = "Option::default")]
    pub justify: Option<LeftCenterRight>,

    #[serde(default = "Option::default")]
    pub name: Option<String>,

    #[serde(default = "Option::default")]
    pub number: Option<u8>,

    #[serde(default = "Option::default")]
    pub placement: Option<Placement>,

    #[serde(rename = "print-object", default = "Option::default")]
    pub print_object: Option<YesNo>,

    #[serde(rename = "relative-x", default = "Option::default")]
    pub relative_x: Option<f32>,

    #[serde(rename = "relative-y", default = "Option::default")]
    pub relative_y: Option<f32>,

    #[serde(rename = "time-only", default = "Option::default")]
    pub time_only: Option<String>,

    #[serde(default = "Option::default")]
    pub syllabic: Option<SyllabicType>,

    #[serde(default = "String::default")]
    pub text: String,

    #[serde(default = "Option::default")]
    pub extend: Option<Extend>,
}

#[cfg(test)]
mod tests {
    use crate::musicxml::{
        core::{Placement, SyllabicType},
        lyric::Lyric,
        printable_value::LeftCenterRight,
        start_stop_continue::StartStopContinue,
    };
    use roxmltree::Document;
    use serde_xml_rs::from_str;

    #[test]
    fn lyric_1() {
        let xml = r#"
            <lyric number="1" placement="below">
                <syllabic>begin</syllabic>
                <text>Hej</text>
            </lyric>"#;

        let item: Lyric = from_str(xml).unwrap();

        assert_eq!(Placement::Below, item.placement.unwrap());
        assert_eq!(SyllabicType::Begin, item.syllabic.unwrap());
        assert_eq!("Hej", item.text);
    }

    #[test]
    fn lyric_2() {
        let xml = r#"
            <lyric default-y="-62" justify="left" number="1">
                <syllabic>single</syllabic>
                <text>Ah</text>
                <extend type="start"/>
            </lyric>
        "#;

        let item: Lyric = from_str(xml).unwrap();

        assert_eq!(item.default_y.unwrap(), -62.0);
        assert_eq!(item.justify.unwrap(), LeftCenterRight::Left);
        assert_eq!(item.number.unwrap(), 1);

        assert_eq!(item.number, Some(1));
        assert_eq!(item.syllabic.unwrap(), SyllabicType::Single);
        assert_eq!(item.text, "Ah");
        assert_eq!(
            item.extend.unwrap().r#type.unwrap(),
            StartStopContinue::Start
        );
    }
}
