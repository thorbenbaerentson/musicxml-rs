use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Work {
    #[serde(default = "String::default")]
    title: String,
}
