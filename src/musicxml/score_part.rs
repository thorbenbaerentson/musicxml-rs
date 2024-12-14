use crate::musicxml::{
    midi_device::MidiDevice, midi_instrument::MidiInstrument, score_instrument::ScoreInstrument,
};
use crate::prelude::*;

#[derive(Debug)]
pub struct ScorePart {
    pub id: String,
    pub part_name: String,
    pub part_abbrevieation: String,
    pub score_instrument: Option<ScoreInstrument>,
    pub midi_device: Option<MidiDevice>,
    pub midi_instrument: Option<MidiInstrument>,
}
