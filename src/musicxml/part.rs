use crate::musicxml::measure::Measure;
use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Part {
    #[serde(default = "String::default")]
    pub id: String,

    #[serde(rename = "measure", default = "Vec::default")]
    pub measures: Vec<Measure>,
}
