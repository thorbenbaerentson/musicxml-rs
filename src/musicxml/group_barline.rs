use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, PartialOrd)]
pub enum GroupBarline {
    #[default]
    None,

    #[serde(rename = "yes")]
    Yes,

    #[serde(rename = "no")]
    No,

    #[serde(rename = "Mensurstrich")]
    Mensurstrich,
}
