use std::time::Duration;

use serde::{Deserialize, Serialize};

use super::{level::Level, printable_value::PrintableValue};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Forward {
    #[serde(default = "Duration::default")]
    duration: Duration,

    #[serde(default = "Option::default")]
    footnote: Option<PrintableValue<String>>,

    #[serde(default = "Option::default")]
    level: Option<Level>,

    #[serde(default = "Option::default")]
    voice: Option<String>,

    #[serde(default = "Option::default")]
    staff: Option<u8>,
}
