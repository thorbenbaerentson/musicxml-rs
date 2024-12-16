use super::{
    attributes::Attributes, barline::Barline, core::Duration, direction::Direction,
    harmony::Harmony, left_right_middle::LeftRightMiddle, level::Level, note::Note,
    system_divider::SystemDivider,
};
use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemMargins {
    #[serde(rename = "left-margin", default = "f32::default")]
    pub left_margin: f32,

    #[serde(rename = "right-margin", default = "f32::default")]
    pub right_margin: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemDividers {
    #[serde(rename = "left-divider", default = "SystemDivider::default")]
    pub left_divider: SystemDivider,

    #[serde(rename = "right-divider", default = "SystemDivider::default")]
    pub right_divider: SystemDivider,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemLayout {
    #[serde(rename = "system-margins", default = "Option::default")]
    pub system_margins: Option<SystemMargins>,

    #[serde(rename = "system-distance", default = "Option::default")]
    pub system_distance: Option<f32>,

    #[serde(rename = "top-system-distance", default = "Option::default")]
    pub top_system_distance: Option<f32>,

    #[serde(rename = "system-dividers", default = "Option::default")]
    pub system_dividers: Option<SystemDividers>,
}

#[cfg(test)]
mod tests {
    use super::SystemLayout;
    use serde_xml_rs::from_str;

    #[test]
    fn system_layout_1() {
        let xml = r#"
            <system-layout>
                <system-margins>
                    <left-margin>70</left-margin>
                    <right-margin>0</right-margin>
                </system-margins>
                <top-system-distance>211</top-system-distance>
            </system-layout>
        "#;

        let item: SystemLayout = from_str(xml).unwrap();
        let margins = &item.system_margins.unwrap();

        assert_eq!(margins.left_margin, 70.0);
        assert_eq!(margins.right_margin, 0.0);
        assert_eq!(item.top_system_distance.unwrap(), 211.0);
    }

    #[test]
    fn system_layout_2() {
        let xml = r#"
            <system-layout>
                <system-margins>
                    <left-margin>0</left-margin>
                    <right-margin>0</right-margin>
                </system-margins>
                <system-distance>121</system-distance>
            </system-layout>
        "#;

        let item: SystemLayout = from_str(xml).unwrap();
        let margins = &item.system_margins.unwrap();

        assert_eq!(margins.left_margin, 0.0);
        assert_eq!(margins.right_margin, 0.0);
        assert_eq!(item.system_distance.unwrap(), 121.0);
    }
}
