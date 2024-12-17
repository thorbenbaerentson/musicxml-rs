use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScoreInstrument {
    #[serde(default = "String::default")]
    id: String,

    #[serde(default = "String::default")]
    instrument_name: String,
}
