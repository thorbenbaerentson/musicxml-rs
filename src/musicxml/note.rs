use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

use crate::musicxml::{
    core::{DirectionUD, Duration, DurationType, Placement},
    pitch::parse_option_pitch,
};
use crate::prelude::*;
use std::str::FromStr;

use super::articulations::{ArticulationType, Articulations};
use super::harmony::Pitch;
use super::lyric::Lyric;
use super::stem::Stem;

#[derive(Debug, Serialize, Deserialize)]
pub struct Dot {}

// https://www.w3.org/2021/06/musicxml40/musicxml-reference/elements/notations/
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Notations {
    #[serde(rename = "$value", default = "Vec::default")]
    pub notations: Vec<NotationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    #[serde(default = "Duration::default")]
    pub duration: Duration,

    #[serde(rename = "type")]
    pub notetype: DurationType,

    #[serde(rename = "pitch")]
    pub pitch: Option<Pitch>,

    #[serde(default = "u8::default")]
    pub voice: u8,

    #[serde(default = "u8::default")]
    pub staff: u8,

    #[serde(default = "bool::default")]
    pub rest: bool,

    #[serde(default = "Vec::default")]
    pub dot: Vec<Dot>,

    #[serde(default = "Option::default")]
    pub stem: Option<Stem>,

    #[serde(default = "usize::default")]
    pub position: usize,

    #[serde(default = "bool::default")]
    pub chord: bool,

    #[serde(default = "Vec::default")]
    pub chord_notes: Vec<Note>,

    #[serde(default = "Vec::default")]
    pub lyrics_above: Vec<Lyric>,

    #[serde(default = "Vec::default")]
    pub lyrics_below: Vec<Lyric>,

    #[serde(default = "Option::default")]
    pub notations: Option<Notations>,

    // Attributes
    #[serde(rename = "attack", default = "Option::default")]
    pub attack: Option<f32>,

    #[serde(rename = "color", default = "Option::default")]
    pub color: Option<String>,

    #[serde(rename = "default-x", default = "Option::default")]
    pub default_x: Option<f32>,

    #[serde(rename = "default-y", default = "Option::default")]
    pub default_y: Option<f32>,
}

#[derive(Debug, EnumString, PartialEq, Serialize, Deserialize, Default)]
pub enum StartStop {
    #[strum(serialize = "start")]
    #[serde(rename = "start")]
    #[default]
    Start,

    #[strum(serialize = "stop")]
    #[serde(rename = "stop")]
    Stop,
}

// https://www.w3.org/2021/06/musicxml40/musicxml-reference/elements/accidental-mark/

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum NotationType {
    #[serde(rename = "tied")]
    Tied {
        #[serde(default = "StartStop::default")]
        r#type: StartStop,

        #[serde(rename = "bezier-offset", default = "Option::default")]
        bezier_offset: Option<f32>,

        #[serde(rename = "bezier-offset2", default = "Option::default")]
        bezier_offset2: Option<f32>,

        #[serde(rename = "bezier-x", default = "Option::default")]
        bezier_x: Option<f32>,

        #[serde(rename = "bezier-x2", default = "Option::default")]
        bezier_x2: Option<f32>,

        #[serde(rename = "bezier-y", default = "Option::default")]
        bezier_y: Option<f32>,

        #[serde(rename = "bezier-y2", default = "Option::default")]
        bezier_y2: Option<f32>,

        #[serde(rename = "color", default = "Option::default")]
        color: Option<String>,
    },

    #[serde(rename = "slur")]
    Slur {
        #[serde(default = "StartStop::default")]
        r#type: StartStop,

        #[serde(default = "u8::default")]
        number: u8,
    },

    #[serde(rename = "tuplet")]
    Tuplet {
        #[serde(default = "StartStop::default")]
        r#type: StartStop,

        #[serde(default = "u8::default")]
        number: u8,
    },

    #[serde(rename = "glissando")]
    Glissando {
        #[serde(default = "StartStop::default")]
        r#type: StartStop,

        #[serde(default = "u8::default")]
        number: u8,
    },

    #[serde(rename = "slide")]
    Slide {
        #[serde(default = "StartStop::default")]
        r#type: StartStop,

        #[serde(default = "u8::default")]
        number: u8,
    },

    #[serde(rename = "ornaments")]
    Ornaments {
        #[serde(default = "StartStop::default")]
        r#type: StartStop,

        #[serde(default = "u8::default")]
        number: u8,
    },

    #[serde(rename = "technical")]
    Technical {
        #[serde(default = "StartStop::default")]
        r#type: StartStop,

        #[serde(default = "u8::default")]
        number: u8,
    },

    #[serde(rename = "articulations")]
    Articulations(Articulations),

    #[serde(rename = "dynamics")]
    Dynamics {
        #[serde(default = "StartStop::default")]
        r#type: StartStop,

        #[serde(default = "u8::default")]
        number: u8,
    },

    #[serde(rename = "fermata")]
    Fermata {
        #[serde(default = "StartStop::default")]
        r#type: StartStop,

        #[serde(default = "u8::default")]
        number: u8,
    },

    #[serde(rename = "arpeggiate")]
    Arpeggiate {
        #[serde(default = "StartStop::default")]
        r#type: StartStop,

        #[serde(default = "u8::default")]
        number: u8,
    },

    #[serde(rename = "non-arpeggiate")]
    NonArpeggiate {
        #[serde(default = "StartStop::default")]
        r#type: StartStop,

        #[serde(default = "u8::default")]
        number: u8,
    },

    #[serde(rename = "accidental-mark")]
    AccidentalMark {
        #[serde(default = "StartStop::default")]
        r#type: StartStop,

        #[serde(default = "u8::default")]
        number: u8,
    },

    #[serde(rename = "other-notation")]
    OtherNotation {
        #[serde(default = "StartStop::default")]
        r#type: StartStop,

        #[serde(default = "u8::default")]
        number: u8,
    },
}

#[cfg(test)]
mod tests_note {
    use std::default;

    use crate::musicxml::{
        articulations::{ArticulationMeta, ArticulationType, Articulations},
        core::{DirectionUD, DurationType, Placement},
        harmony::Step,
        note::{NotationType, Note, StartStop},
        stem::Stem,
    };
    use roxmltree::Document;
    use serde_xml_rs::from_str;

    #[test]
    fn note() {
        let xml = r#"
            <note>
                <pitch>
                    <step>C</step>
                    <octave>4</octave>
                </pitch>
                <duration>4</duration>
                <type>whole</type>
            </note>"#;
        let note: Note = from_str(xml).unwrap();

        assert_eq!(4, note.duration);

        let pitch = &note.pitch.unwrap();
        assert_eq!(Step::C, pitch.step);
        assert_eq!(4, pitch.octave);
        assert_eq!(4, note.duration);
        assert_eq!(DurationType::Whole, note.notetype);
    }

    #[test]
    fn doubledot() {
        let xml = r#"
            <note>
                <pitch>
                    <step>G</step>
                    <octave>4</octave>
                </pitch>
                <duration>7</duration>
                <voice>1</voice>
                <type>half</type>
                <dot/>
                <dot/>
                <stem>up</stem>
            </note>"#;

        let note: Note = from_str(xml).unwrap();

        assert_eq!(7, note.duration);
        assert_eq!(2, note.dot.len());
        assert_eq!(1, note.voice);
        assert_eq!(DurationType::Half, note.notetype);
        assert_eq!(
            Some(Stem {
                content: DirectionUD::Up,
                ..Stem::default()
            }),
            note.stem
        );

        let pitch = note.pitch.unwrap();
        assert_eq!(Step::G, pitch.step);
        assert_eq!(4, pitch.octave);
    }

    #[test]
    fn notations() {
        let xml = r#"
        <note>
            <pitch>
                <step>G</step>
                <octave>4</octave>
            </pitch>
            <duration>4</duration>
            <voice>1</voice>
            <type>quarter</type>
            <stem>up</stem>
            <staff>1</staff>
            <notations>
                <slur type="start" number="1"/>
                <tied type="stop"/>
            </notations>
        </note>"#;

        let note: Note = from_str(xml).unwrap();

        assert_eq!(4, note.duration);
        assert_eq!(0, note.dot.len());
        assert_eq!(1, note.voice);
        assert_eq!(
            Stem {
                content: DirectionUD::Up,
                ..Stem::default()
            },
            note.stem.unwrap()
        );
        assert_eq!(DurationType::Quarter, note.notetype);

        let pitch = note.pitch.unwrap();
        assert_eq!(Step::G, pitch.step);
        assert_eq!(4, pitch.octave);

        let notations = note.notations.unwrap();
        assert_eq!(
            NotationType::Slur {
                r#type: StartStop::Start,
                number: 1
            },
            notations.notations[0]
        );
        assert_eq!(
            NotationType::Tied {
                r#type: StartStop::Stop,
                bezier_offset: Option::default(),
                bezier_offset2: Option::default(),
                bezier_x: Option::default(),
                bezier_x2: Option::default(),
                bezier_y: Option::default(),
                bezier_y2: Option::default(),
                color: Option::default(),
            },
            notations.notations[1]
        );
    }

    #[test]
    fn accidental_mark() {
        let xml = r#"
        <note default-x="84">
            <pitch>
                <step>D</step>
                <octave>5</octave>
            </pitch>
            <duration>2</duration>
            <voice>1</voice>
            <type>quarter</type>
            <stem default-y="-45">down</stem>
            <notations>
                <accidental-mark default-y="22">sharp</accidental-mark>
            </notations>
        </note>"#;
        let mut note: Note = from_str(xml).unwrap();

        assert_eq!(note.default_x.unwrap(), 84.0);
    }

    // https://www.w3.org/2021/06/musicxml40/musicxml-reference/examples/accent-element/
    #[test]
    fn test_accent() {
        let xml = r#"
            <note default-x="36">
                <pitch>
                    <step>A</step>
                    <octave>4</octave>
                </pitch>
                <duration>4</duration>
                <voice>1</voice>
                <type>half</type>
                <stem default-y="10">up</stem>
                <notations>
                    <articulations>
                        <accent default-x="-1" default-y="-55" placement="below"/>
                    </articulations>
                </notations>
            </note>
        "#;
        let item: Note = from_str(xml).unwrap();

        assert_eq!(item.default_x.unwrap(), 36.0);
        assert_eq!(item.duration, 4);
        assert_eq!(item.voice, 1);
        assert_eq!(item.notetype, DurationType::Half);

        let stem = &item.stem.unwrap();
        assert_eq!(
            stem,
            &Stem {
                content: DirectionUD::Up,
                default_y: Some(10.0),
                ..Stem::default()
            }
        );

        let pitch = &item.pitch.unwrap();
        assert_eq!(pitch.step, Step::A);
        assert_eq!(pitch.octave, 4);

        let notations = &item.notations.unwrap();
        let notationtype = &notations.notations[0];
        NotationType::Articulations(Articulations {
            articulations: vec![ArticulationType::Accent(ArticulationMeta {
                default_x: Some(-1.0),
                default_y: Some(-55.0),
                placement: Some(Placement::Below),
                ..ArticulationMeta::default()
            })],
        });
    }

    #[test]
    fn test_accidental_mark() {
        let xml = r#"
            <note default-x="84">
                <pitch>
                    <step>D</step>
                    <octave>5</octave>
                </pitch>
                <duration>2</duration>
                <voice>1</voice>
                <type>quarter</type>
                <stem default-y="-45">down</stem>
                <notations>
                    <accidental-mark default-y="22">sharp</accidental-mark>
                </notations>
            </note>
        "#;
        let item: Note = from_str(xml).unwrap();

        assert_eq!(item.default_x.unwrap(), 84.0);
        assert_eq!(item.duration, 2);
        assert_eq!(item.voice, 1);
        assert_eq!(item.notetype, DurationType::Quarter);

        let stem = &item.stem.unwrap();
        assert_eq!(
            stem,
            &Stem {
                content: DirectionUD::Down,
                default_y: Some(-45.0),
                ..Stem::default()
            }
        );

        let pitch = &item.pitch.unwrap();
        assert_eq!(pitch.step, Step::D);
        assert_eq!(pitch.octave, 5);

        let notations = &item.notations.unwrap();
    }
}
