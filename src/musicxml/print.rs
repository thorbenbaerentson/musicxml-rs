use std::str;
use serde::{Deserialize, Serialize};
use crate::prelude::*;

use super::{
    attributes::Attributes, barline::Barline, core::Duration, direction::Direction, harmony::Harmony, left_right_middle::LeftRightMiddle, level::Level, measure_layout::MeasureLayout, measure_numbering_value::MeasureNumberingValue, note::Note, page_layout::PageLayout, part_display::PartDisplay, staff_layout::StaffLayout, system_layout::SystemLayout, printable_value::PrintableValue, yes_no::YesNo
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Print {
    #[serde(rename = "page-layout", default = "Option::default")]
    pub page_layout: Option<PageLayout>,

    #[serde(rename = "system-layout", default = "Option::default")]
    pub system_layout: Option<SystemLayout>,

    #[serde(rename = "staff-layout", default = "Option::default")]
    pub staff_layout: Option<StaffLayout>,

    #[serde(rename = "measure-layout", default = "Option::default")]
    pub measure_layout: Option<MeasureLayout>,

    #[serde(rename = "measure-numbering", default = "Option::default")]
    pub measure_numbering : Option<PrintableValue<MeasureNumberingValue>>,

    #[serde(rename = "part-name-display", default = "Option::default")]
    pub part_name_display : Option<PartDisplay>,

    #[serde(rename = "part-abbreviation-display", default = "Option::default")]
    pub part_abbreviation_display : Option<PartDisplay>,

    #[serde(rename = "blank-page", default = "Option::default")]
    pub blank_page : Option<u8>,

    #[serde(default = "Option::default")]
    pub id : Option<String>,

    #[serde(rename = "new-page", default = "Option::default")]
    pub new_page : Option<YesNo>,

    #[serde(rename = "new-system", default = "Option::default")]
    pub new_system : Option<YesNo>,

    #[serde(rename = "page-number", default = "Option::default")]
    pub page_number : Option<String>,

    #[serde(rename = "staff-spacing", default = "Option::default")]
    pub staff_spacing : Option<f32>,
}
