use serde::{Deserialize, Serialize};

use crate::musicxml::{
    midi_device::MidiDevice, midi_instrument::MidiInstrument, score_instrument::ScoreInstrument,
};
use crate::prelude::*;

use super::identification::Identification;
use super::part_group::GroupDisplay;

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    #[serde(rename = "player-name", default = "String::default")]
    pub player_name: String,

    #[serde(rename = "id", default = "String::default")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ScorePartContent {
    #[serde(rename = "score-instrument")]
    ScoreInstrument(ScoreInstrument),

    #[serde(rename = "player")]
    Player(Player),

    #[serde(rename = "midi-device")]
    MidiDevice(MidiDevice),

    #[serde(rename = "midi-instrument")]
    MidiInstrument(MidiInstrument),

    #[serde(rename = "part-name")]
    PartName(String),

    #[serde(rename = "part-name-display")]
    PartNameDisplay(GroupDisplay),

    #[serde(rename = "part-abbreviation")]
    PartAbbreviation(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScorePart {
    #[serde(default = "Option::default")]
    pub identification: Option<Identification>,

    #[serde(default = "String::default")]
    pub id: String,

    #[serde(rename = "$value", default = "Vec::default")]
    pub content : Vec<ScorePartContent>
}

#[cfg(test)]
mod tests {
    use serde_xml_rs::from_str;

    use super::ScorePart;

    #[test]
    fn score_part_1() {
        let xml = r#"
            <score-part id="P2">
                <part-name>Electric Bass</part-name>
                <part-abbreviation>el.bs.</part-abbreviation>
                <score-instrument id="P2-I1">
                    <instrument-name></instrument-name>
                    <instrument-sound>pluck.bass.electric</instrument-sound>
                    </score-instrument>
                <midi-device id="P2-I1" port="1"></midi-device>
                <midi-instrument id="P2-I1">
                    <midi-channel>7</midi-channel>
                    <midi-program>34</midi-program>
                    <volume>82.6772</volume>
                    <pan>0</pan>
                </midi-instrument>
            </score-part>
        "#;

        let item : ScorePart = from_str(xml).unwrap();
    }

    #[test]
    fn score_part_2() {
        let xml = r#"
            <score-part id="P7">
                <part-name>Hand Clap</part-name>
                <part-abbreviation>h.clap.</part-abbreviation>
                <score-instrument id="P7-I40">
                    <instrument-name>Hand Clap</instrument-name>
                    </score-instrument>
                <midi-device port="2"></midi-device>
                <midi-instrument id="P7-I40">
                    <midi-channel>10</midi-channel>
                    <midi-program>1</midi-program>
                    <midi-unpitched>40</midi-unpitched>
                    <volume>62.2047</volume>
                    <pan>0</pan>
                </midi-instrument>
            </score-part>
        "#;

        let item : ScorePart = from_str(xml).unwrap();
    }

    #[test]
    fn score_part_3() {
        let xml = r#"
            <score-part id="P5">
                <part-name>Melody</part-name>
                <part-abbreviation>od.guit.</part-abbreviation>
                <score-instrument id="P5-I1">
                    <instrument-name></instrument-name>
                    <instrument-sound>pluck.guitar.electric</instrument-sound>
                </score-instrument>
                <score-instrument id="P5-I2">
                    <instrument-name></instrument-name>
                    <instrument-sound>pluck.guitar.electric</instrument-sound>
                </score-instrument>
                <score-instrument id="P5-I3">
                    <instrument-name></instrument-name>
                    <instrument-sound>pluck.guitar.electric</instrument-sound>
                </score-instrument>
                <midi-device id="P5-I1" port="2"></midi-device>
                <midi-instrument id="P5-I1">
                    <midi-channel>5</midi-channel>
                    <midi-program>30</midi-program>
                    <volume>80.315</volume>
                    <pan>0</pan>
                </midi-instrument>
                <midi-device id="P5-I2" port="1"></midi-device>
                <midi-instrument id="P5-I2">
                    <midi-channel>13</midi-channel>
                    <midi-program>30</midi-program>
                    <volume>78.7402</volume>
                    <pan>0</pan>
                </midi-instrument>
                <midi-device id="P5-I3" port="1"></midi-device>
                <midi-instrument id="P5-I3">
                    <midi-channel>14</midi-channel>
                    <midi-program>30</midi-program>
                    <volume>78.7402</volume>
                    <pan>0</pan>
                </midi-instrument>
            </score-part>
        "#;

        let item : ScorePart = from_str(xml).unwrap();
    }
}
