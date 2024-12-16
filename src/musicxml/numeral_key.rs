use super::yes_no::YesNo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Default, Clone)]
pub enum NumeralMode {
    #[serde(rename = "harmonic minor")]
    #[default]
    HarmonicMinor,

    #[serde(rename = "major")]
    Major,

    #[serde(rename = "melodic minor")]
    MelodicMinor,

    #[serde(rename = "minor")]
    Minor,

    #[serde(rename = "natural minor")]
    NaturalMinor,
}

// https://www.w3.org/2021/06/musicxml40/musicxml-reference/elements/numeral-key/
#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Default, Clone)]
pub struct NumeralKey {
    // https://www.w3.org/2021/06/musicxml40/musicxml-reference/elements/numeral-root/
    #[serde(rename = "numeral-fifths", default = "i8::default")]
    pub fifths: i8,

    #[serde(rename = "numeral-mode", default = "NumeralMode::default")]
    pub mode: NumeralMode,

    #[serde(rename = "print-object", default = "Option::default")]
    pub print_object: Option<YesNo>,
}
