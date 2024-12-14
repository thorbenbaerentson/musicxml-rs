use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Pitch {
    pub step: Step,
    pub octave: u8,
}

#[derive(Debug, EnumString, PartialEq, Serialize, Deserialize, Default, PartialOrd)]
pub enum Step {
    #[default]
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(Debug, EnumString, Serialize, Deserialize, PartialEq, PartialOrd, Default)]
pub enum HarmonyKind {
    #[strum(serialize = "major")]
    #[serde(rename = "major")]
    #[default]
    Major,

    #[strum(serialize = "minor")]
    #[serde(rename = "minor")]
    Minor,

    #[strum(serialize = "augmented")]
    #[serde(rename = "augmented")]
    Augmented,

    #[strum(serialize = "diminished")]
    #[serde(rename = "diminished")]
    Diminished,

    #[strum(serialize = "dominant")]
    #[serde(rename = "dominant")]
    Dominant,

    #[strum(serialize = "major-seventh")]
    #[serde(rename = "major-seventh")]
    MajorSeventh,

    #[strum(serialize = "minor-seventh")]
    #[serde(rename = "minor-seventh")]
    MinorSeventh,

    #[strum(serialize = "diminished-seventh")]
    #[serde(rename = "diminished-seventh")]
    DiminishedSeventh,

    #[strum(serialize = "half-diminished-seventh")]
    #[serde(rename = "half-diminished-seventh")]
    HalfDiminishedSeventh,

    #[strum(serialize = "augmented-seventh")]
    #[serde(rename = "augmented-seventh")]
    AugmentedSeventh,

    #[strum(serialize = "major-minor-seventh")]
    #[serde(rename = "major-minor-seventh")]
    MajorMinorSeventh,

    #[strum(serialize = "dominant-ninth")]
    #[serde(rename = "dominant-ninth")]
    DominantNinth,

    #[strum(serialize = "major-ninth")]
    #[serde(rename = "major-ninth")]
    MajorNinth,

    #[strum(serialize = "minor-ninth")]
    #[serde(rename = "minor-ninth")]
    MinorNinth,

    #[strum(serialize = "dominant-11th")]
    #[serde(rename = "dominant-11th")]
    Dominant11th,

    #[strum(serialize = "major-11th")]
    #[serde(rename = "major-11th")]
    Major11th,

    #[strum(serialize = "minor-11th")]
    #[serde(rename = "minor-11th")]
    Minor11th,

    #[strum(serialize = "dominant-13th")]
    #[serde(rename = "dominant-13th")]
    Dominant13th,

    #[strum(serialize = "major-13th")]
    #[serde(rename = "major-13th")]
    Major13th,

    #[strum(serialize = "minor-13th")]
    #[serde(rename = "minor-13th")]
    Minor13th,

    #[strum(serialize = "suspended-second")]
    #[serde(rename = "suspended-second")]
    SuspendedSecond,

    #[strum(serialize = "suspended-fourth")]
    #[serde(rename = "suspended-fourth")]
    SuspendedFourth,

    #[strum(serialize = "neapolitan")]
    #[serde(rename = "neapolitan")]
    Neapolitan,

    #[strum(serialize = "italian")]
    #[serde(rename = "italian")]
    Italian,

    #[strum(serialize = "french")]
    #[serde(rename = "french")]
    French,

    #[strum(serialize = "german")]
    #[serde(rename = "german")]
    German,

    #[strum(serialize = "pedal")]
    #[serde(rename = "pedal")]
    Pedal,

    #[strum(serialize = "power")]
    #[serde(rename = "power")]
    Power,

    #[strum(serialize = "tristan")]
    #[serde(rename = "tristan")]
    Tristan,

    #[strum(serialize = "other")]
    #[serde(rename = "other")]
    Other,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum HarmonyItem {
    #[serde(rename = "root")]
    Root {
        #[serde(rename = "root-step", default = "Step::default")]
        step: Step,
        #[serde(rename = "root-alter", default = "Option::default")]
        alter: Option<u8>,
    },

    #[serde(rename = "kind")]
    Kind {
        #[serde(default = "String::default")]
        text: String,
        #[serde(rename = "$value", default = "HarmonyKind::default")]
        kind: HarmonyKind,
    },

    #[serde(rename = "bass")]
    Bass {
        #[serde(rename = "bass-step", default = "String::default")]
        step: String,
        #[serde(rename = "bass-alter", default = "Option::default")]
        alter: Option<u8>,
    },
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Default)]
pub struct Harmony {
    #[serde(rename = "$value", default = "Vec::default")]
    items: Vec<HarmonyItem>,
    #[serde(rename = "position", default = "usize::default")]
    position: usize,
}

#[cfg(test)]
mod tests {
    use crate::musicxml::harmony::{Harmony, HarmonyItem, HarmonyKind, Step};
    use roxmltree::Document;
    use serde_xml_rs::from_str;

    #[test]
    fn example() {
        let xml = r#"
            <harmony>
                <root>
                    <root-step>A</root-step>
                    <root-alter>0</root-alter>
                </root>
                <kind text="7">dominant</kind>
                <bass>
                    <bass-step>E</bass-step>
                    <bass-alter>0</bass-alter>
                </bass>
            </harmony>"#;

        let item: Harmony = from_str(xml).unwrap();

        let mut item_a = &item.items[0];
        let mut check_a = HarmonyItem::Root {
            step: Step::A,
            alter: Some(0),
        };
        assert_eq!(*item_a, check_a);

        item_a = &item.items[1];
        check_a = HarmonyItem::Kind {
            text: "7".to_string(),
            kind: HarmonyKind::Dominant,
        };
        assert_eq!(*item_a, check_a);

        item_a = &item.items[2];
        check_a = HarmonyItem::Bass {
            step: "E".to_string(),
            alter: Some(0),
        };
        assert_eq!(*item_a, check_a);
    }

    #[test]
    fn harmony_kind() {
        let xml = r#"
            <harmony>
                <root>
                    <root-step>D</root-step>
                    <root-alter>0</root-alter>
                </root>
                <kind text="">major</kind>
            </harmony>
        "#;
        let item: Harmony = from_str(xml).unwrap();
    }
}
