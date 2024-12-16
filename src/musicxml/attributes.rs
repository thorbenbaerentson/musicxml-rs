use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Clone)]
pub struct Attributes {
    #[serde(default = "Option::default")]
    pub divisions: Option<usize>,

    #[serde(default = "Option::default")]
    pub staves: Option<u8>,

    #[serde(default = "Option::default")]
    pub key: Option<Key>,

    #[serde(default = "Option::default")]
    pub time: Option<Time>,

    #[serde(default = "Option::default")]
    pub clef: Option<Clef>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Default, Clone)]
pub enum KeyMode {
    #[serde(rename = "none")]
    #[default]
    None,

    #[serde(rename = "major")]
    Mayjor,

    #[serde(rename = "minor")]
    Minor,

    #[serde(rename = "dorian")]
    Dorian,

    #[serde(rename = "phrygian")]
    Phrygian,

    #[serde(rename = "lydian")]
    Lydian,

    #[serde(rename = "mixolydian")]
    Mixolydian,

    #[serde(rename = "aeolian")]
    Aeolian,

    #[serde(rename = "ionian")]
    Ionian,

    #[serde(rename = "locrian")]
    Locrian,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Clone)]
pub struct Key {
    #[serde(default = "i8::default")]
    pub number: i8,

    #[serde(default = "i8::default")]
    pub fifths: i8,

    #[serde(default = "KeyMode::default")]
    pub mode: KeyMode,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Clone)]
pub struct Time {
    #[serde(default = "u8::default")]
    pub beats: u8,

    #[serde(default = "u8::default", rename = "beat-type")]
    pub beat_type: u8,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Clone)]
pub struct Clef {
    #[serde(default = "char::default")]
    pub sign: char,

    #[serde(default = "i8::default")]
    pub line: i8,

    #[serde(default = "i8::default")]
    pub number: i8,
}

impl Attributes {
    pub fn empty() -> Attributes {
        Attributes {
            divisions: None,
            staves: None,
            key: None,
            time: None,
            clef: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::musicxml::attributes::KeyMode;
    use serde_xml_rs::from_str;

    use super::Attributes;

    #[test]
    fn attributes() {
        let xml = r#"
            <attributes>
                <divisions>4</divisions>
                <key number="1">
                    <fifths>0</fifths>
                    <mode>none</mode>
                </key>
                <time>
                    <beats>4</beats>
                    <beat-type>4</beat-type>
                </time>
                <staves>1</staves>
                <clef number="1">
                    <sign>G</sign>
                    <line>2</line>
                </clef>
            </attributes>"#;

        let attribs: Attributes = from_str(xml).unwrap();

        assert_eq!(attribs.divisions, Some(4));
        assert_eq!(attribs.staves, Some(1));

        let key = attribs.key.unwrap();
        assert_eq!(key.number, 1);
        assert_eq!(key.fifths, 0);
        assert_eq!(key.mode, KeyMode::None);

        let time = attribs.time.unwrap();
        assert_eq!(time.beats, 4);
        assert_eq!(time.beat_type, 4);

        let clef = attribs.clef.unwrap();
        assert_eq!(clef.sign, 'G');
        assert_eq!(clef.line, 2);
        assert_eq!(clef.number, 1);
    }
}
