use super::{harmony::Step, numeral_key::NumeralKey, printable_value::PrintableValue};
use serde::{Deserialize, Serialize};

// https://www.w3.org/2021/06/musicxml40/musicxml-reference/elements/numeral/
#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Default, Clone)]
pub struct Numeral {
    // https://www.w3.org/2021/06/musicxml40/musicxml-reference/elements/numeral-root/
    #[serde(rename = "numeral-root", default = "PrintableValue::default")]
    pub root: PrintableValue<u8>,

    #[serde(rename = "numeral-alter", default = "Option::default")]
    pub alter: Option<PrintableValue<i8>>,

    #[serde(rename = "numeral-key", default = "Option::default")]
    pub key: Option<NumeralKey>,
}

#[cfg(test)]
mod tests {
    use super::Numeral;
    use crate::musicxml::{
        harmony::Step,
        numeral_key::{NumeralKey, NumeralMode},
        printable_value::LeftRight,
    };
    use serde_xml_rs::from_str;

    #[test]
    fn numeral_1() {
        let xml = r#"
            <numeral>
                <numeral-root text="IV">4</numeral-root>
            </numeral>
        "#;

        let item: Numeral = from_str(xml).unwrap();

        assert_eq!(item.root.content, 4);
        assert_eq!(item.root.text.unwrap(), "IV");
    }

    #[test]
    fn numeral_2() {
        let xml = r#"
            <numeral>
                <numeral-root text="3">3</numeral-root>
                <numeral-alter location="right">-1</numeral-alter>
            </numeral>
        "#;

        let item: Numeral = from_str(xml).unwrap();

        let alter = item.alter.unwrap();

        assert_eq!(item.root.text.unwrap(), "3");
        assert_eq!(item.root.content, 3);
        assert_eq!(alter.content, -1);
        assert_eq!(alter.location.unwrap(), LeftRight::Right);
    }

    #[test]
    fn numeral_3() {
        let xml = r#"
            <numeral>
                <numeral-root text="IV">4</numeral-root>
                <numeral-key>
                    <numeral-fifths>1</numeral-fifths>
                    <numeral-mode>major</numeral-mode>
                </numeral-key>
            </numeral>
        "#;

        let item: Numeral = from_str(xml).unwrap();
        let key = item.key.unwrap();

        assert_eq!(item.root.text.unwrap(), "IV");
        assert_eq!(item.root.content, 4);
        assert_eq!(key.fifths, 1);
        assert_eq!(key.mode, NumeralMode::Major);
    }

    #[test]
    fn numeral_4() {
        let xml = r#"
            <numeral>
                <numeral-root text="III">3</numeral-root>
                <numeral-alter location="left">-1</numeral-alter>
            </numeral>
        "#;

        let item: Numeral = from_str(xml).unwrap();
        let alter = item.alter.unwrap();

        assert_eq!(item.root.text.unwrap(), "III");
        assert_eq!(alter.location.unwrap(), LeftRight::Left);
        assert_eq!(alter.content, -1);
    }
}
