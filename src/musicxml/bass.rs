use super::{
    harmony::{HarmonyArrangement, Step},
    printable_value::PrintableValue,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Default)]
pub struct Bass {
    #[serde(rename = "bass-separator", default = "Option::default")]
    pub separator: Option<PrintableValue<String>>,

    #[serde(rename = "bass-step", default = "PrintableValue::default")]
    pub step: PrintableValue<Step>,

    #[serde(rename = "bass-alter", default = "Option::default")]
    pub alter: Option<PrintableValue<f32>>,

    #[serde(rename = "arrangement", default = "Option::default")]
    pub arrangement: Option<HarmonyArrangement>,
}
