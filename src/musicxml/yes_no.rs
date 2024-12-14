use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Default)]
pub enum YesNo {
    #[serde(rename = "yes")]
    #[default]
    Yes,
    #[serde(rename = "no")]
    No,
}
