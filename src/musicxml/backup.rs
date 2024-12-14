use super::{core::Duration, level::Level, printable_value::PrintableValue};
use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Backup {
    #[serde(default = "Duration::default")]
    duration: Duration,

    #[serde(default = "Option::default")]
    footnote: Option<PrintableValue<String>>,

    level: Option<Level>,
}

#[cfg(test)]
mod tests {
    use super::Backup;
    use serde_xml_rs::from_str;

    #[test]
    fn backup() {
        let xml = r#"
            <backup>
              <duration>3</duration>
            </backup>
        "#;
        let item: Backup = from_str(xml).unwrap();
        
        assert_eq!(item.duration, 3);
        assert_eq!(item.footnote, None);
        assert_eq!(item.level, None);
    }
}
