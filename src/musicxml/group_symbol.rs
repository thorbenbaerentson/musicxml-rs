use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, PartialOrd)]
pub enum GroupSymbol {
    #[serde(rename = "brace")]
    Brace,

    #[serde(rename = "bracket")]
    Bracket,

    #[serde(rename = "line")]
    Line,

    #[serde(rename = "none")]
    #[default]
    None,

    #[serde(rename = "square")]
    Square,
}
