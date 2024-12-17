use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScoreInstrument {
    #[serde(default = "String::default")]
    pub id: String,

    #[serde(rename = "instrument-name", default = "String::default")]
    pub instrument_name: String,

    #[serde(rename = "instrument-sound", default = "String::default")]
    pub instrument_sound: String,
}
