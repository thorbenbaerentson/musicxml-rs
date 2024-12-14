use serde::{Deserialize, Serialize};
use crate::prelude::*;
use super::{
    attributes::Attributes, barline::Barline, core::Duration, direction::Direction,
    harmony::Harmony, left_right_middle::LeftRightMiddle, level::Level, note::Note,
    page_margins::PageMargins
};

#[derive(Debug, Serialize, Deserialize)]
pub struct PageLayout {
    #[serde(rename = "page-height", default = "Option::default")]
    pub page_height: Option<f32>,

    #[serde(rename = "page-width", default = "Option::default")]
    pub page_width: Option<f32>,

    #[serde(rename = "page-margins", default = "Vec::default")]
    pub page_margins: Vec<PageMargins>,
}

#[cfg(test)]
mod tests {
    use super::PageLayout;
    use serde_xml_rs::from_str;

    #[test]
    fn page_layout() {
        let xml = r#"
        <page-layout>
            <page-height>1553</page-height>
            <page-width>1200</page-width>
            <page-margins type="both">
                <left-margin>70</left-margin>
                <right-margin>70</right-margin>
                <top-margin>88</top-margin>
                <bottom-margin>88</bottom-margin>
            </page-margins>
        </page-layout>
        "#;

        let item: PageLayout = from_str(xml).unwrap();
        assert_eq!(item.page_height.unwrap(), 1553.0);
        assert_eq!(item.page_width.unwrap(), 1200.0);

        let margins = &item.page_margins[0];
        assert_eq!(margins.left_margin, 70.0);
        assert_eq!(margins.right_margin, 70.0);
        assert_eq!(margins.top_margin, 88.0);
        assert_eq!(margins.bottom_margin, 88.0);
    }
}
