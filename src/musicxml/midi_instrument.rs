use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MidiInstrument {
    #[serde(default = "String::default")]
    pub id: String,

    #[serde(default = "u8::default")]
    pub midi_channel: u8,

    #[serde(default = "u8::default")]
    pub midi_program: u8,

    #[serde(default = "f32::default")]
    pub volume: f32,

    #[serde(default = "f32::default")]
    pub pan: f32,
}
