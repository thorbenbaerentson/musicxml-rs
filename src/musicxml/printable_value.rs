use serde::{Deserialize, Serialize};

// https://www.w3.org/2021/06/musicxml40/musicxml-reference/data-types/text-direction/
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, PartialOrd)]
#[serde(rename = "text-direction")]
pub enum TextDirection {
    #[default]
    #[serde(rename = "ltr")]
    LeftToRightEmbed,

    #[serde(rename = "rtl")]
    RightToLeftEmbed,

    #[serde(rename = "lro")]
    LeftToRightBidiOverride,

    #[serde(rename = "rlo")]
    RightToLeftBidiOverride,
}

// https://www.w3.org/2021/06/musicxml40/musicxml-reference/data-types/enclosure-shape/
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, PartialOrd)]
#[serde(rename = "enclosure-shape")]
pub enum EnclosureShape {
    #[default]
    None,

    #[serde(rename = "rectangle")]
    Rectangle,

    #[serde(rename = "square")]
    Square,

    #[serde(rename = "oval")]
    Oval,

    #[serde(rename = "circle")]
    Circle,

    #[serde(rename = "bracket")]
    Bracket,

    #[serde(rename = "inverted-bracket")]
    InvertedBracket,

    #[serde(rename = "triangle")]
    Triangle,

    #[serde(rename = "diamond")]
    Diamond,

    #[serde(rename = "pentagon")]
    Pentagon,

    #[serde(rename = "hexagon")]
    Hexagon,

    #[serde(rename = "heptagon")]
    Heptagon,

    #[serde(rename = "octagon")]
    Octagon,

    #[serde(rename = "nonagon")]
    Nonagon,

    #[serde(rename = "decagon")]
    Decagon,
}

// https://www.w3.org/2021/06/musicxml40/musicxml-reference/data-types/font-weight/
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, PartialOrd)]
pub enum FontWeight {
    #[serde(rename = "normal")]
    #[default]
    Normal,

    #[serde(rename = "bold")]
    Bold,
}

// https://www.w3.org/2021/06/musicxml40/musicxml-reference/data-types/font-style/
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, PartialOrd)]
pub enum FontStyle {
    #[serde(rename = "normal")]
    #[default]
    Normal,

    #[serde(rename = "italic")]
    Italic,
}

// https://www.w3.org/2021/06/musicxml40/musicxml-reference/data-types/left-center-right/
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, PartialOrd)]
#[serde(rename = "left-center-right")]
pub enum LeftCenterRight {
    #[serde(rename = "left")]
    #[default]
    Left,

    #[serde(rename = "center")]
    Center,

    #[serde(rename = "right")]
    Right,
}

// https://www.w3.org/2021/06/musicxml40/musicxml-reference/data-types/valign/
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, PartialOrd)]
pub enum Valign {
    #[serde(rename = "top")]
    #[default]
    Top,

    #[serde(rename = "middle")]
    Middle,

    #[serde(rename = "bottom")]
    Bottom,

    #[serde(rename = "baseline")]
    Baseline,
}

// https://www.w3.org/2021/06/musicxml40/musicxml-reference/data-types/xml-space/
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, PartialOrd)]
pub enum XmlSpace {
    #[serde(rename = "default")]
    #[default]
    Default,

    #[serde(rename = "preserve")]
    Preserve,
}

/// Describes a value that is annotated with various information about how to print 
/// or display it. 
/// 
// https://www.w3.org/2021/06/musicxml40/musicxml-reference/elements/footnote/
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Default)]
pub struct PrintableValue<T> where 
    T : Default {
    #[serde(rename = "$value", default = "T::default")]
    pub text: T,

    #[serde(default = "Option::default")]
    pub color: Option<String>,

    #[serde(rename = "default-x", default = "Option::default")]
    pub default_x: Option<f32>,

    #[serde(rename = "default-y", default = "Option::default")]
    pub default_y: Option<f32>,

    #[serde(rename = "dir", default = "Option::default")]
    pub text_direction: Option<TextDirection>,

    #[serde(rename = "enclosure", default = "Option::default")]
    pub enclosure: Option<EnclosureShape>,

    #[serde(rename = "font-family", default = "Option::default")]
    pub font_family: Option<String>,

    #[serde(rename = "font-size", default = "Option::default")]
    pub font_size: Option<f32>,

    #[serde(rename = "font-weight", default = "Option::default")]
    pub font_weight: Option<FontWeight>,

    #[serde(rename = "font-style", default = "Option::default")]
    pub font_style: Option<FontStyle>,

    #[serde(rename = "halign", default = "Option::default")]
    pub halign: Option<LeftCenterRight>,

    #[serde(rename = "justify", default = "Option::default")]
    pub justify: Option<LeftCenterRight>,

    #[serde(rename = "letter-spacing", default = "Option::default")]
    pub letter_spacing: Option<String>,

    #[serde(rename = "line-height", default = "Option::default")]
    pub line_height: Option<String>,

    #[serde(rename = "line-through", default = "Option::default")]
    pub line_through: Option<u8>,

    #[serde(rename = "overline", default = "Option::default")]
    pub overline: Option<u8>,

    #[serde(rename = "relative-x", default = "Option::default")]
    pub relative_x: Option<String>,

    #[serde(rename = "relative-y", default = "Option::default")]
    pub relative_y: Option<String>,

    #[serde(rename = "rotation", default = "Option::default")]
    pub rotation: Option<u8>,

    #[serde(rename = "underline", default = "Option::default")]
    pub underline: Option<u8>,

    #[serde(rename = "valign", default = "Option::default")]
    pub valign: Option<Valign>,

    #[serde(rename = "xml:lang", default = "Option::default")]
    pub xml_lang: Option<String>,

    #[serde(rename = "xml:space", default = "Option::default")]
    pub xml_space: Option<XmlSpace>,
}

#[cfg(test)]
mod tests {
    use serde_xml_rs::from_str;

    use crate::musicxml::printable_value::PrintableValue;

    #[test]
    fn footnote() {
        let xml = r#"<footnote xml:lang="de">*) Urspr: = Nicht zu geschwind.</footnote>"#;
        let foot: PrintableValue<String> = from_str(xml).unwrap();

        assert_eq!(foot.text, "*) Urspr: = Nicht zu geschwind.".to_string());
    }
}
