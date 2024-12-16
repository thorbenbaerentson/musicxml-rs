use serde::{Deserialize, Serialize};

use super::{
    note::StartStop,
    printable_value::{LeftRight, PrintableValue},
};

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Default)]
pub struct FirstFret {
    #[serde(rename = "$value", default = "u8::default")]
    pub content: u8,

    #[serde(rename = "location", default = "LeftRight::default")]
    pub location: LeftRight,

    #[serde(rename = "text", default = "String::default")]
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Default)]
pub struct Barre {
    #[serde(rename = "location", default = "StartStop::default")]
    pub r#type: StartStop,

    #[serde(rename = "text", default = "Option::default")]
    pub color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Default)]
pub struct FrameNote {
    #[serde(rename = "string", default = "PrintableValue::default")]
    pub content: PrintableValue<u8>,

    #[serde(rename = "fret", default = "PrintableValue::default")]
    pub fret: PrintableValue<u8>,

    #[serde(rename = "fingering", default = "Option::default")]
    pub fingering: Option<PrintableValue<String>>,

    #[serde(rename = "barre", default = "Option::default")]
    pub barre: Option<PrintableValue<String>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Default)]
pub struct Frame {
    #[serde(rename = "frame-strings", default = "u8::default")]
    pub frame_strings: u8,

    #[serde(rename = "frame-frets", default = "u8::default")]
    pub frame_frets: u8,

    #[serde(rename = "first-fret", default = "Option::default")]
    pub first_fret: Option<FirstFret>,

    #[serde(rename = "frame-note", default = "Vec::default")]
    pub frame_notes: Vec<FrameNote>,
}

#[cfg(test)]
mod tests {
    use serde_xml_rs::from_str;

    use crate::musicxml::harmony::Harmony;

    #[test]
    fn frame() {
        let xml = r#"
        <harmony font-family="Arial">
            <root>
                <root-step>A</root-step>
            </root>
            <kind halign="center" text="">major</kind>
            <frame default-y="93" halign="center" unplayed="x" valign="top">
                <frame-strings>6</frame-strings>
                <frame-frets>4</frame-frets>
                <frame-note>
                    <string>5</string>
                    <fret>0</fret>
                </frame-note>
                <frame-note>
                    <string>4</string>
                    <fret>2</fret>
                    <fingering>2</fingering>
                </frame-note>
                <frame-note>
                    <string>3</string>
                    <fret>2</fret>
                    <fingering>3</fingering>
                </frame-note>
                <frame-note>
                    <string>2</string>
                    <fret>2</fret>
                    <fingering>4</fingering>
                </frame-note>
                <frame-note>
                    <string>1</string>
                    <fret>0</fret>
                </frame-note>
            </frame>
        </harmony>
        "#;

        let item: Harmony = from_str(xml).unwrap();
    }
}
