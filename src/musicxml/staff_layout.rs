use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct StaffLayout {
    #[serde(rename = "staff-distance", default = "Option::default")]
    pub staff_distance: Option<f32>,

    #[serde(default = "Option::default")]
    pub number: Option<u8>,
}
