use crate::prelude::*;
use serde::{Deserialize, Serialize};

use super::printable_value::PrintableValue;

// https://www.w3.org/2021/06/musicxml40/musicxml-reference/elements/credit/
#[derive(Debug, Serialize, Deserialize)]
pub struct Credit {
    #[serde(rename = "credit-type", default = "Option::default")]
    pub credit_type: Option<String>,
    #[serde(rename = "credit-words", default = "Option::default")]
    pub credit_words: Option<PrintableValue<String>>,
}

#[cfg(test)]
mod tests {
    use super::Credit;
    use crate::musicxml::printable_value::{FontWeight, LeftCenterRight, Valign};
    use serde_xml_rs::from_str;

    #[test]
    fn credit() {
        let xml = r#"
            <credit page="1">
                <credit-type>title</credit-type>
                <credit-words default-x="683" default-y="1725" font-size="24" font-weight="bold" halign="center" valign="top">Sonata, Op. 27, No. 2</credit-words>
            </credit>"#;
        let item: Credit = from_str(xml).unwrap();
        assert_eq!(item.credit_type.unwrap(), "title".to_string());

        let words = item.credit_words.unwrap();
        assert_eq!(words.default_x.unwrap(), 683.0);
        assert_eq!(words.default_y.unwrap(), 1725.0);
        assert_eq!(words.font_size.unwrap(), 24.0);
        assert_eq!(words.font_weight.unwrap(), FontWeight::Bold);
        assert_eq!(words.halign.unwrap(), LeftCenterRight::Center);
        assert_eq!(words.valign.unwrap(), Valign::Top);
        assert_eq!(words.content, "Sonata, Op. 27, No. 2".to_string());
    }
}
