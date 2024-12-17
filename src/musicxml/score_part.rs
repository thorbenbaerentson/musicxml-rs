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
pub struct ScorePart {
    #[serde(default = "Option::default")]
    pub identification: Option<Identification>,

    #[serde(default = "String::default")]
    pub id: String,

    #[serde(rename = "part-name", default = "String::default")]
    pub part_name: String,

    #[serde(rename = "part-name-display", default = "Option::default")]
    pub part_name_display: Option<GroupDisplay>,

    #[serde(rename = "part-abbreviation", default = "String::default")]
    pub part_abbreviation: String,

    #[serde(rename = "score-instrument", default = "Vec::default")]
    pub score_instruments: Vec<ScoreInstrument>,

    #[serde(rename = "player", default = "Vec::default")]
    pub players: Vec<Player>,

    #[serde(rename = "midi-device", default = "Vec::default")]
    pub midi_devices: Vec<MidiDevice>,

    #[serde(rename = "midi-instrument", default = "Vec::default")]
    pub midi_instruments: Vec<MidiInstrument>,
}
