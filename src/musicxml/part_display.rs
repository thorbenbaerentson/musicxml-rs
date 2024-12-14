use serde::{Deserialize, Serialize};

use super::printable_value::PrintableValue;

// https://www.w3.org/2021/06/musicxml40/musicxml-reference/elements/part-name-display/
#[derive(Debug, Serialize, Deserialize)]
pub struct PartDisplay {
    #[serde(rename = "display-text", default = "Option::default")]
    display_text : Option<PrintableValue<String>>,

    #[serde(rename = "accidental-text", default = "Option::default")]
    accidental_text : Option<PrintableValue<String>>,
}