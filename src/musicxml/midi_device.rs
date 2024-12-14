use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MidiDevice {
    #[serde(default = "String::default")]
    pub id: String,

    #[serde(default = "u8::default")]
    pub port: u8,
}
