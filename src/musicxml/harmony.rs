use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum_macros::EnumString;

use super::{
    bass::Bass, degree::Degree, frame::Frame, level::Level, numeral::Numeral,
    printable_value::PrintableValue, root::Root, yes_no::YesNo,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Clone)]
pub struct Pitch {
    pub step: Step,
    pub octave: u8,
}

#[derive(Debug, EnumString, PartialEq, Serialize, Deserialize, Default, PartialOrd, Clone)]
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

#[derive(Debug, EnumString, PartialEq, Serialize, Deserialize, Default, PartialOrd, Clone)]
pub enum HarmonyArrangement {
    #[serde(rename = "horizontal")]
    #[default]
    Horizontal,

    #[serde(rename = "vertical")]
    Vertical,

    #[serde(rename = "diagonal")]
    Diagonal,
}

#[derive(Debug, EnumString, Serialize, Deserialize, PartialEq, PartialOrd, Default, Clone)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Default, Clone)]
pub struct HarmonyOffset {
    #[serde(rename = "$value", default = "f32::default")]
    content: f32,

    #[serde(rename = "sound", default = "Option::default")]
    sound: Option<YesNo>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum HarmonyItem {
    #[serde(rename = "root")]
    Root(Root),

    #[serde(rename = "numeral")]
    Numeral(Numeral),

    #[serde(rename = "kind")]
    Kind(PrintableValue<HarmonyKind>),

    #[serde(rename = "inversion")]
    Inversion(PrintableValue<u8>),

    #[serde(rename = "bass")]
    Bass(Bass),

    #[serde(rename = "degree")]
    Degree(Degree),

    #[serde(rename = "frame")]
    Frame(Frame),

    #[serde(rename = "offset")]
    Offset(HarmonyOffset),

    #[serde(rename = "footnote")]
    Footnote(PrintableValue<String>),

    #[serde(rename = "level")]
    Level(Level),

    #[serde(rename = "staff")]
    Staff(u8),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Default)]
pub struct Harmony {
    #[serde(rename = "$value", default = "Vec::default")]
    pub items: Vec<HarmonyItem>,
}

#[cfg(test)]
mod tests {
    use crate::musicxml::{
        harmony::{Harmony, HarmonyItem, HarmonyKind, Step},
        numeral::{self, Numeral},
        printable_value::LeftCenterRight,
    };
    use roxmltree::Document;
    use serde_xml_rs::from_str;

    #[test]
    fn nano_sec_error() {
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

        match &item.items[0] {
            HarmonyItem::Root(r) => {
                assert_eq!(r.step.content, Step::D);
                assert_eq!(r.alter.clone().unwrap().content, 0);
            }
            _ => {
                panic!("Expected a root.");
            }
        }

        match &item.items[1] {
            HarmonyItem::Kind(k) => {
                assert_eq!(k.content, HarmonyKind::Major);
            }
            _ => {
                panic!("Expected a harmony kind.");
            }
        }
    }

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

        match &item.items[0] {
            HarmonyItem::Root(r) => {
                assert_eq!(r.step.content, Step::A);
                assert_eq!(r.alter.clone().unwrap().content, 0);
            }
            _ => {
                panic!("Expected a root.");
            }
        }

        match &item.items[1] {
            HarmonyItem::Kind(k) => {
                assert_eq!(k.content, HarmonyKind::Dominant);
                assert_eq!(k.text.clone().unwrap(), "7".to_string());
            }
            _ => {
                panic!("Expected a harmony kind.");
            }
        }

        match &item.items[2] {
            HarmonyItem::Bass(b) => {
                assert_eq!(b.step.content, Step::E);
                assert_eq!(b.alter.clone().unwrap().content, 0.0);
            }
            _ => {
                panic!("Expected a harmony kind.");
            }
        }
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

        match &item.items[0] {
            HarmonyItem::Root(r) => {
                assert_eq!(r.step.content, Step::D);
                assert_eq!(r.alter.clone().unwrap().content, 0);
            }
            _ => {
                panic!("Expected a root.");
            }
        }

        match &item.items[1] {
            HarmonyItem::Kind(k) => {
                assert_eq!(k.content, HarmonyKind::Major);
            }
            _ => {
                panic!("Expected a harmony kind.");
            }
        }
    }

    #[test]
    fn harmony_item() {
        let xml = r#"
            <harmony default-y="-80">
                <numeral>
                    <numeral-root text="IV">4</numeral-root>
                </numeral>
                <kind halign="center" text="">minor</kind>
                <inversion>1</inversion>
            </harmony>
        "#;

        let item: Harmony = from_str(xml).unwrap();
        assert_eq!(item.items.len(), 3);

        match &item.items[0] {
            HarmonyItem::Numeral(num) => {
                assert_eq!(num.root.text.clone().unwrap(), "IV".to_string());
                assert_eq!(num.root.content, 4);
            }
            _ => {
                panic!("Expected a numeral.");
            }
        }

        match &item.items[1] {
            HarmonyItem::Kind(k) => {
                assert_eq!(k.halign.clone().unwrap(), LeftCenterRight::Center);
                assert_eq!(k.content, HarmonyKind::Minor);
            }
            _ => {
                panic!("Expected a kind.");
            }
        }

        match &item.items[2] {
            HarmonyItem::Inversion(i) => {
                assert_eq!(i.content, 1);
            }
            _ => {
                panic!("Expected a kind.");
            }
        }
    }

    #[test]
    fn harmony_item_2() {
        let xml = r#"
            <harmony>
                <root>
                    <root-step>C</root-step>
                </root>
                <kind halign="center" text="">major</kind>
                <bass arrangement="horizontal">
                    <bass-step>G</bass-step>
                    <bass-alter>1</bass-alter>
                </bass>
            </harmony>
        "#;

        let item: Harmony = from_str(xml).unwrap();

        match &item.items[0] {
            HarmonyItem::Root(r) => {
                assert_eq!(r.step.content, Step::C);
            }
            _ => {
                panic!("Expected a root.");
            }
        }

        match &item.items[1] {
            HarmonyItem::Kind(k) => {
                assert_eq!(k.halign.clone().unwrap(), LeftCenterRight::Center);
                assert_eq!(k.content, HarmonyKind::Major);
            }
            _ => {
                panic!("Expected a harmony kind.");
            }
        }

        match &item.items[2] {
            HarmonyItem::Bass(b) => {
                assert_eq!(b.step.content, Step::G);
                assert_eq!(b.alter.clone().unwrap().content, 1.0);
            }
            _ => {
                panic!("Expected a harmony kind.");
            }
        }
    }

    #[test]
    fn harmony_3() {
        let xml = r#"
            <harmony>
                <root>
                    <root-step>C</root-step>
                    <root-alter>0</root-alter>
                </root>
                <kind>major</kind>
                <frame>
                    <frame-strings>6</frame-strings>
                    <frame-frets>5</frame-frets>
                    <first-fret>1</first-fret>
                    <frame-note>
                        <string>5</string>
                        <fret>3</fret>
                    </frame-note>
                    <frame-note>
                        <string>4</string>
                        <fret>5</fret>
                    </frame-note>
                        <frame-note>
                        <string>3</string>
                        <fret>5</fret>
                    </frame-note>
                    <frame-note>
                        <string>2</string>
                        <fret>5</fret>
                    </frame-note>
                </frame>
            </harmony>
        "#;
        let item : Harmony = from_str(xml).unwrap();

        match &item.items[0] {
            HarmonyItem::Root(root) => {
                assert_eq!(root.step.content, Step::C);
                assert_eq!(root.alter.clone().unwrap().content, 0);
            },
            _ => {
                panic!("Expected different item");
            }
        }


    }
}
