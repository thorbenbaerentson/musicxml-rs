use super::yes_no::YesNo;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, PartialOrd)]
pub enum SymbolSize {
    #[serde(rename = "cue")]
    #[default]
    Cue,

    #[serde(rename = "full")]
    Full,

    #[serde(rename = "grace-cue")]
    GraceCue,

    #[serde(rename = "large")]
    Large,
}
