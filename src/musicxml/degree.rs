use super::{
    harmony::{HarmonyArrangement, Step},
    printable_value::PrintableValue,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Default)]
pub enum DegreeTypeValue {
    #[serde(rename = "add")]
    #[default]
    Add,

    #[serde(rename = "add")]
    Alter,

    #[serde(rename = "add")]
    Subtract,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Default)]
pub struct Degree {
    #[serde(rename = "degree-value", default = "PrintableValue::default")]
    pub value: PrintableValue<String>,

    #[serde(rename = "degree-alter", default = "PrintableValue::default")]
    pub alter: PrintableValue<u8>,

    #[serde(rename = "degree-type", default = "PrintableValue::default")]
    pub degree_type: PrintableValue<f32>,
}
