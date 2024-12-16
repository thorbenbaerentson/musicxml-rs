use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MeasureLayout {
    #[serde(rename = "measure-distance", default = "Option::default")]
    measure_distance: Option<f32>,
}
