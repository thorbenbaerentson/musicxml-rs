use super::{
    note::StartStop,
    start_stop_single::{self, StartStopSingle},
    symbol_size::SymbolSize,
    yes_no::YesNo,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Default)]
pub struct Level {
    #[serde(rename = "$value", default = "String::default")]
    content: String,

    #[serde(default = "Option::default")]
    bracket: Option<YesNo>,

    #[serde(default = "Option::default")]
    parentheses: Option<YesNo>,

    #[serde(default = "Option::default")]
    reference: Option<YesNo>,

    #[serde(default = "Option::default")]
    size: Option<SymbolSize>,

    #[serde(default = "Option::default")]
    r#type: Option<StartStopSingle>,
}
