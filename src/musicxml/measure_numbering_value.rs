use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum MeasureNumberingValue {
    #[serde(rename = "none")]
    #[default]
    None,

    #[serde(rename = "measure")]
    Measure,

    #[serde(rename = "system")]
    System,
}
