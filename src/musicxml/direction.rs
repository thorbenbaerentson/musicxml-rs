use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

use crate::musicxml::core::Placement;
use crate::prelude::*;
use std::str::FromStr;

use super::core::DurationType;
use super::dynamics::Dynamics;
use super::note::NotationType;
use super::printable_value::PrintableValue;
use super::yes_no::YesNo;

#[derive(Debug, EnumString, Serialize, Deserialize, PartialEq, PartialOrd, Default)]
pub enum WedgeType {
    #[strum(serialize = "none")]
    #[serde(rename = "none")]
    #[default]
    None,

    #[strum(serialize = "crescendo")]
    #[serde(rename = "crescendo")]
    Crescendo,

    #[strum(serialize = "diminuendo")]
    #[serde(rename = "diminuendo")]
    Diminuendo,

    #[strum(serialize = "stop")]
    #[serde(rename = "stop")]
    Stop,

    #[strum(serialize = "continue")]
    #[serde(rename = "continue")]
    Continue,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum DirectionType {
    #[serde(rename = "wedge")]
    Wedge {
        #[serde(default = "WedgeType::default")]
        r#type: WedgeType,
        #[serde(default = "u8::default")]
        number: u8,
    },

    #[serde(rename = "dynamics")]
    Dynamic(PrintableValue<Dynamics>),

    #[serde(rename = "words")]
    Words(PrintableValue<String>),

    #[serde(rename = "metronome")]
    Metronome {
        #[serde(rename = "beat-unit", default = "DurationType::default")]
        beat_unit: DurationType,

        #[serde(rename = "per-minute", default = "u8::default")]
        per_minute: u8,
    },

    #[serde(rename = "rehersal")]
    Rehersal { text: String },

    #[serde(rename = "coda")]
    Coda,

    #[serde(rename = "segno")]
    Segno,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Direction {
    #[serde(default = "usize::default")]
    pub position: usize,

    #[serde(default = "Vec::default", rename = "direction-type")]
    pub directiontypes: Vec<DirectionType>,

    #[serde(default = "u8::default")]
    pub staff: u8,

    #[serde(default = "Option::default")]
    pub placement: Option<Placement>,

    #[serde(default = "YesNo::default")]
    pub directive: YesNo,
}

#[cfg(test)]
mod test_direction {
    use crate::musicxml::{
        core::DurationType,
        direction::{Direction, DirectionType, WedgeType},
        printable_value::{FontStyle, FontWeight, TextDirection, PrintableValue},
    };

    use roxmltree::Document;
    use serde_xml_rs::from_str;

    #[test]
    fn direction_1() {
        let xml = r#"
            <direction>
                <direction-type>
                    <wedge type="crescendo" number="1"/>
                </direction-type>
                <staff>1</staff>
            </direction>"#;
        let item: Direction = from_str(xml).unwrap();

        assert_eq!(item.staff, 1);
        assert_eq!(item.directiontypes.len(), 1);
        assert_eq!(
            item.directiontypes[0],
            DirectionType::Wedge {
                r#type: WedgeType::Crescendo,
                number: 1
            }
        );
    }

    #[test]
    fn direction_2() {
        let xml = r#"
            <direction placement="above" directive="yes">
                <direction-type>
                    <words font-style="normal" font-weight="bold">Allegro moderato</words>
                </direction-type>
                <direction-type>
                    <metronome>
                        <beat-unit>quarter</beat-unit>
                        <per-minute>120</per-minute>
                    </metronome>
                </direction-type>
                <staff>1</staff>
            </direction>"#;

        let item: Direction = from_str(xml).unwrap();

        assert_eq!(item.staff, 1);
        assert_eq!(item.directiontypes.len(), 2);

        let dir_type0 = &item.directiontypes[0];
        let dir_type1 = &item.directiontypes[1];

        let check = PrintableValue::<String> {
            text: "Allegro moderato".to_string(),
            font_style: Some(FontStyle::Normal),
            font_weight: Some(FontWeight::Bold),
            ..Default::default()
        };

        assert_eq!(*dir_type0, DirectionType::Words(check));
        assert_eq!(
            *dir_type1,
            DirectionType::Metronome {
                beat_unit: DurationType::Quarter,
                per_minute: 120
            }
        );
    }
}
