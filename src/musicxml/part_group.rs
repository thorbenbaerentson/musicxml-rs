use std::fmt;

use super::{
    accidental::Accidental, group_barline::GroupBarline, group_symbol::GroupSymbol, level::Level,
    note::StartStop, printable_value::PrintableValue, yes_no::YesNo,
};
use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct GroupDisplay {
    #[serde(rename = "display-text", default = "String::default")]
    pub display_text: String,

    #[serde(rename = "accidental-text", default = "Option::default")]
    pub accidental_text: Option<Accidental>,

    #[serde(rename = "@print-object", default = "Option::default")]
    pub print_object: Option<YesNo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartGroup {
    #[serde(rename = "group-name", default = "Option::default")]
    group_name: Option<String>,

    #[serde(rename = "group-name-display", default = "Option::default")]
    group_name_display: Option<GroupDisplay>,

    #[serde(rename = "group-abbreviation", default = "Option::default")]
    group_abbreviation: Option<String>,

    #[serde(rename = "group-abbreviation-display", default = "Option::default")]
    group_abbreviation_display: Option<GroupDisplay>,

    #[serde(rename = "group-symbol", default = "Option::default")]
    group_symbol: Option<GroupSymbol>,

    #[serde(rename = "group-barline", default = "Option::default")]
    group_barline: Option<GroupBarline>,

    #[serde(rename = "footnote", default = "Option::default")]
    footnote: Option<String>,

    #[serde(rename = "level", default = "Option::default")]
    level: Option<Level>,

    #[serde(default = "StartStop::default")]
    pub r#type: StartStop,

    #[serde(rename = "number", default = "Option::default")]
    pub number: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::PartGroup;
    use crate::musicxml::{
        accidental::Accidental, barline::Barline, group_barline::GroupBarline,
        group_symbol::GroupSymbol, note::StartStop, part_group::GroupDisplay,
        printable_value::PrintableValue,
    };
    use serde_xml_rs::from_str;

    #[test]
    fn display_text() {
        let xml = r#"<display-text>Trumpet in B</display-text>"#;
        let foot: PrintableValue<String> = from_str(xml).unwrap();

        assert_eq!(foot.content, "Trumpet in B".to_string());
    }

    #[test]
    fn display_accidental() {
        let xml = r#"<accidental-text>flat</accidental-text>"#;
        let foot: PrintableValue<Accidental> = from_str(xml).unwrap();

        assert_eq!(foot.content, Accidental::Flat);
    }

    #[test]
    fn group_name() {
        let xml = r#"
            <group-name-display>
                <display-text>Trumpet in B</display-text>
                <accidental-text>flat</accidental-text>
            </group-name-display>
        "#;
        let item: GroupDisplay = from_str(xml).unwrap();

        assert_eq!(item.display_text, "Trumpet in B".to_string());
        assert_eq!(item.accidental_text.unwrap(), Accidental::Flat);
    }

    #[test]
    fn part_group() {
        let xml = r#"
        <part-group number="3" type="start">
            <group-name>Trumpet in B flat</group-name>
            <group-name-display>
                <display-text>Trumpet in B</display-text>
                <accidental-text>flat</accidental-text>
            </group-name-display>
            <group-abbreviation>Trp. in B flat</group-abbreviation>
            <group-abbreviation-display>
                <display-text>Trp. in B</display-text>
                <accidental-text>flat</accidental-text>
            </group-abbreviation-display>
            <group-symbol default-x="-10">square</group-symbol>
            <group-barline>yes</group-barline>
        </part-group>
        "#;

        let item = from_str::<PartGroup>(xml).unwrap();

        assert_eq!(item.number.unwrap(), "3".to_string());
        assert_eq!(item.r#type, StartStop::Start);

        assert_eq!(&item.group_name.unwrap(), &"Trumpet in B flat".to_string());

        let name = item.group_name_display.unwrap();
        assert_eq!(name.display_text, "Trumpet in B".to_string());
        assert_eq!(name.accidental_text.unwrap(), Accidental::Flat);

        assert_eq!(
            &item.group_abbreviation.unwrap(),
            &"Trp. in B flat".to_string()
        );

        let abbre = item.group_abbreviation_display.unwrap();
        assert_eq!(abbre.display_text, "Trp. in B".to_string());
        assert_eq!(abbre.accidental_text.unwrap(), Accidental::Flat);

        // match &item.content[4] {
        //     PartGroupContent::GroupSymbol(n) => {
        //         assert_eq!(n, &GroupSymbol::Square);
        //     },
        //     _ => { panic!("Expected different content type."); }
        // }

        // match &item.content[5] {
        //     PartGroupContent::GroupBarline(n) => {
        //         assert_eq!(n, &GroupBarline::Yes);
        //     },
        //     _ => { panic!("Expected different content type."); }
        // }
    }
}
