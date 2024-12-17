use serde::{Deserialize, Serialize};

use crate::musicxml::{
    midi_device::MidiDevice, midi_instrument::MidiInstrument, score_instrument::ScoreInstrument,
};
use crate::prelude::*;

use super::identification::Identification;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScorePart {
    #[serde(default = "Option::default")]
    pub identification: Option<Identification>,

    #[serde(default = "String::default")]
    pub id: String,

    #[serde(default = "String::default")]
    pub part_name: String,

    #[serde(rename = "part-abbrevieation", default = "String::default")]
    pub part_abbrevieation: String,

    #[serde(rename = "score-instrument", default = "Option::default")]
    pub score_instrument: Option<ScoreInstrument>,

    #[serde(rename = "midi-device", default = "Option::default")]
    pub midi_device: Option<MidiDevice>,

    #[serde(rename = "midi-instrument", default = "Option::default")]
    pub midi_instrument: Option<MidiInstrument>,
}
