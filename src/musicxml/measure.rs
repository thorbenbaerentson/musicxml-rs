use super::{
    attributes::Attributes, backup::Backup, barline::Barline, core::Duration, direction::Direction,
    forward::Forward, harmony::Harmony, left_right_middle::LeftRightMiddle, level::Level,
    note::Note, print::Print, yes_no::YesNo,
};
use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum MeasureContent {
    #[serde(rename = "note")]
    Note(Note),

    #[serde(rename = "backup")]
    Backup(Backup),

    #[serde(rename = "barline")]
    Barline(Barline),

    #[serde(rename = "attributes")]
    Attributes(Attributes),

    #[serde(rename = "harmony")]
    Harmony(Harmony),

    #[serde(rename = "forward")]
    Forward(Forward),

    #[serde(rename = "print")]
    Print(Print),

    #[serde(rename = "direction")]
    Direction(Direction),

    #[serde(rename = "non-controlling")]
    NonControlling(YesNo),

    #[serde(rename = "implicit")]
    Implicit(YesNo),

    #[serde(rename = "id")]
    Id(String),

    #[serde(rename = "width")]
    Width(f32),

    #[serde(rename = "number")]
    Number(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Measure {
    #[serde(rename = "$value", default = "Vec::default")]
    pub content: Vec<MeasureContent>,
}

impl Measure {
    pub fn get_voice(&self, voice_idx: u8) -> Vec<&Note> {
        let mut voice: Vec<&Note> = vec![];
        for n in &self.content {
            match n {
                MeasureContent::Note(note) => {
                    if note.voice == voice_idx {
                        voice.push(note)
                    };
                }
                _ => {}
            }
        }
        voice
    }

    pub fn get_attributes(&self) -> Option<&Attributes> {
        let r: Vec<&Attributes> = self
            .content
            .iter()
            .filter_map(|x| match x {
                MeasureContent::Attributes(attributes) => Some(attributes),
                _ => None,
            })
            .collect();

        if r.len() != 1 {
            return None;
        }

        Some(r[0])
    }
}

#[cfg(test)]
mod test_measure {
    use crate::musicxml::attributes::{Key, KeyMode};
    use crate::musicxml::core::DurationType;
    use crate::musicxml::harmony::{Pitch, Step};
    use crate::musicxml::measure::{Measure, MeasureContent};
    use crate::prelude::*;
    use roxmltree::Document;
    use serde_xml_rs::from_str;
    use std::fs;

    #[test]
    fn example() {
        let xml: &str = r#"
          <measure number="1">
            <attributes>
              <divisions>1</divisions>
              <key>
                <fifths>0</fifths>
              </key>
              <time>
                <beats>4</beats>
                <beat-type>4</beat-type>
              </time>
              <clef>
                <sign>G</sign>
                <line>2</line>
              </clef>
            </attributes>

            <note>
              <pitch>
                <step>C</step>
                <octave>4</octave>
              </pitch>
              <duration>4</duration>
              <type>whole</type>
            </note>

          </measure>"#;

        let item: Measure = from_str(xml).unwrap();

        assert_eq!(2, item.content.len());

        match &item.content[0] {
            MeasureContent::Attributes(a) => {
                assert_eq!(a.divisions.unwrap(), 1);
                assert_eq!(
                    a.key.clone().unwrap(),
                    Key {
                        number: 0,
                        fifths: 0,
                        mode: KeyMode::None
                    }
                );

                assert_eq!(a.time.clone().unwrap().beats, 4);
                assert_eq!(a.time.clone().unwrap().beat_type, 4);

                assert_eq!(a.clef.clone().unwrap().sign, 'G');
                assert_eq!(a.clef.clone().unwrap().line, 2);
            }

            _ => {
                panic!("Expected attributes")
            }
        }

        match &item.content[1] {
            MeasureContent::Note(n) => {
                assert_eq!(
                    n.pitch.clone().unwrap(),
                    Pitch {
                        step: Step::C,
                        octave: 4
                    }
                );
                assert_eq!(n.duration, 4);
                assert_eq!(n.notetype, DurationType::Whole);
            }
            _ => {
                panic!("Expected note")
            }
        }
    }

    #[test]
    fn test_multiple_notes() {
        let xml = r#"
        <measure number="1" width="309.95">
          <note>
            <pitch>
              <step>G</step>
              <octave>4</octave>
            </pitch>
            <duration>2</duration>
            <voice>1</voice>
            <type>half</type>
            <stem>up</stem>
          </note>

          <note>
            <pitch>
              <step>G</step>
              <octave>4</octave>
            </pitch>
            <duration>1</duration>
            <voice>1</voice>
            <type>quarter</type>
            <stem>up</stem>
          </note>

          <backup>
              <duration>3</duration>
          </backup>

          <note>
            <pitch>
              <step>E</step>
              <octave>4</octave>
            </pitch>
            <duration>1</duration>
            <voice>2</voice>
            <type>quarter</type>
            <stem>down</stem>
          </note>

          <note>
            <pitch>
              <step>E</step>
              <octave>4</octave>
            </pitch>
            <duration>2</duration>
            <voice>2</voice>
            <type>half</type>
            <stem>down</stem>
          </note>

          <barline location="right">
            <bar-style>light-heavy</bar-style>
          </barline>

        </measure>"#;

        let item: Measure = from_str(&xml).unwrap();
    }

    #[test]
    fn test_voices() {
        let xml = fs::read_to_string("xml-files/measure/test-voices.xml").unwrap();
        let item: Measure = from_str(&xml).unwrap();

        assert_eq!(6, item.content.len());

        let voice1 = item.get_voice(1);
        let voice2 = item.get_voice(2);

        assert_eq!(2, voice1.len());
        assert_eq!(2, voice2.len());

        assert_eq!(voice1[0].position, 0);
        assert_eq!(voice1[1].position, 0);
        assert_eq!(voice1[0].duration, 2);
        assert_eq!(voice1[1].duration, 1);

        assert_eq!(voice2[0].position, 0);
        assert_eq!(voice2[1].position, 0);
    }

    #[test]
    fn test_lyrics1() {
        let xml = fs::read_to_string("xml-files/measure/test-lyrics1.musicxml").unwrap();
        let item: Measure = from_str(&xml).unwrap();
    }

    #[test]
    fn test_lyrics_placement() {
        let xml = fs::read_to_string("xml-files/measure/test-lyrics-placement.xml").unwrap();
        let item: Measure = from_str(&xml).unwrap();
    }

    #[test]
    fn test_lyrics_placement_voices() {
        let xml = fs::read_to_string("xml-files/measure/test-lyrics-placement2.xml").unwrap();
        let item: Measure = from_str(&xml).unwrap();
    }

    #[test]
    fn test_directions_dynamics() {
        let xml = fs::read_to_string("xml-files/measure/test-directions-dynamics.xml").unwrap();
        let item: Measure = from_str(&xml).unwrap();
    }

    #[test]
    fn test_note_values() {
        let xml = r#"
        <measure number="1">
        <attributes>
          <divisions>16</divisions>
          <key number="1">
            <fifths>0</fifths>
            <mode>none</mode>
          </key>
          <staves>1</staves>
          <clef number="1">
            <sign>G</sign>
            <line>2</line>
          </clef>
        </attributes>
        <note>
          <pitch>
            <step>G</step>
            <octave>4</octave>
          </pitch>
          <duration>128</duration>
          <voice>1</voice>
          <type>breve</type>
          <stem>up</stem>
          <staff>1</staff>
        </note>
        <note>
          <pitch>
            <step>G</step>
            <octave>4</octave>
          </pitch>
          <duration>64</duration>
          <voice>1</voice>
          <type>whole</type>
          <stem>up</stem>
          <staff>1</staff>
        </note>
        <note>
          <pitch>
            <step>G</step>
            <octave>4</octave>
          </pitch>
          <duration>32</duration>
          <voice>1</voice>
          <type>half</type>
          <stem>up</stem>
          <staff>1</staff>
        </note>
        <note>
          <pitch>
            <step>G</step>
            <octave>4</octave>
          </pitch>
          <duration>16</duration>
          <voice>1</voice>
          <type>quarter</type>
          <stem>up</stem>
          <staff>1</staff>
        </note>
        <note>
          <pitch>
            <step>G</step>
            <octave>4</octave>
          </pitch>
          <duration>8</duration>
          <voice>1</voice>
          <type>eighth</type>
          <stem>up</stem>
          <staff>1</staff>
          <beam number="1">begin</beam>
        </note>
        <note>
          <pitch>
            <step>G</step>
            <octave>4</octave>
          </pitch>
          <duration>4</duration>
          <voice>1</voice>
          <type>16th</type>
          <stem>up</stem>
          <staff>1</staff>
          <beam number="1">continue</beam>
          <beam number="2">begin</beam>
        </note>
        <note>
          <pitch>
            <step>G</step>
            <octave>4</octave>
          </pitch>
          <duration>2</duration>
          <voice>1</voice>
          <type>32nd</type>
          <stem>up</stem>
          <staff>1</staff>
          <beam number="1">continue</beam>
          <beam number="2">continue</beam>
          <beam number="3">begin</beam>
        </note>
        <note>
          <pitch>
            <step>G</step>
            <octave>4</octave>
          </pitch>
          <duration>1</duration>
          <voice>1</voice>
          <type>64th</type>
          <stem>up</stem>
          <staff>1</staff>
          <beam number="1">end</beam>
          <beam number="2">end</beam>
          <beam number="3">end</beam>
          <beam number="4">backward hook</beam>
        </note>
        <barline location="right">
          <bar-style>light-heavy</bar-style>
        </barline>
      </measure>"#;

        let item: Measure = from_str(&xml).unwrap();
    }

    #[test]
    fn test_harmonies() {
        let xml = r#"
        <measure number="1">
        <attributes>
          <divisions>4</divisions>

          <key number="1">
            <fifths>2</fifths>
            <mode>major</mode>
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

        </attributes>

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
        </note>

        <backup>
          <duration>4</duration>
        </backup>

        <harmony>
          <root>
            <root-step>D</root-step>
            <root-alter>0</root-alter>
          </root>
          <kind text="">major</kind>
        </harmony>

        <forward>
          <duration>4</duration>
        </forward>

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
        </note>

        <backup>
          <duration>4</duration>
        </backup>

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
        </harmony>

        <forward>
          <duration>4</duration>
        </forward>

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
        </note>

        <backup>
          <duration>4</duration>
        </backup>

        <harmony>
          <root>
            <root-step>D</root-step>
            <root-alter>0</root-alter>
          </root>
          <kind text="">major</kind>
          <bass>
            <bass-step>F</bass-step>
            <bass-alter>1</bass-alter>
          </bass>
        </harmony>

        <forward>
          <duration>4</duration>
        </forward>

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
        </note>
      </measure>"#;
        let item: Measure = from_str(&xml).unwrap();
    }
}
