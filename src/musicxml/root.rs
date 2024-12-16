use super::{harmony::Step, printable_value::PrintableValue};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Default)]
pub struct Root {
    #[serde(rename = "root-step", default = "PrintableValue::default")]
    pub step: PrintableValue<Step>,

    #[serde(rename = "root-alter", default = "Option::default")]
    pub alter: Option<PrintableValue<u8>>,
}

#[cfg(test)]
mod tests {
    use super::Root;
    use crate::musicxml::harmony::Step;
    use serde_xml_rs::from_str;

    #[test]
    fn root() {
        let xml = r#"
            <root>
                <root-step>C</root-step>
                <root-alter>1</root-alter>
            </root>
        "#;

        let item: Root = from_str(xml).unwrap();

        assert_eq!(item.step.content, Step::C);
        assert_eq!(item.alter.unwrap().content, 1);
    }
}
