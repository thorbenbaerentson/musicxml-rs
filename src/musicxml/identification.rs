use crate::prelude::*;
use serde::{Deserialize, Serialize};

use super::yes_no::YesNo;

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, PartialOrd)]
pub struct TypedContent {
    #[serde(rename = "type", default = "Option::default")]
    pub r#type: Option<String>,
    #[serde(rename = "$value", default = "String::default")]
    pub content: String,
}

// https://www.w3.org/2021/06/musicxml40/musicxml-reference/elements/supports/
#[derive(Debug, Serialize, Deserialize, Default, PartialEq, PartialOrd)]
pub struct Supports {
    #[serde(default = "String::default")]
    pub element: String,

    #[serde(default = "YesNo::default")]
    r#type: YesNo,

    #[serde(default = "Option::default")]
    pub attribute: Option<String>,

    #[serde(default = "Option::default")]
    pub value: Option<String>,
}

// https://www.w3.org/2021/06/musicxml40/musicxml-reference/elements/encoding/
#[derive(Debug, Serialize, Deserialize, Default, PartialEq, PartialOrd)]
pub struct Encoding {
    #[serde(rename = "encoding-date", default = "Option::default")]
    pub encoding_date: Option<String>,

    #[serde(rename = "encoder", default = "Vec::default")]
    pub encoder: Vec<TypedContent>,

    #[serde(rename = "software", default = "Option::default")]
    pub software: Option<String>,

    #[serde(rename = "encoding-description", default = "Option::default")]
    pub encoding_description: Option<String>,
}

// https://www.w3.org/2021/06/musicxml40/musicxml-reference/elements/miscellaneous-field/
#[derive(Debug, Serialize, Deserialize, Default, PartialEq, PartialOrd, Clone)]
pub struct MiscellaneousField {
    #[serde(default = "String::default")]
    pub name: String,

    #[serde(rename = "$value", default = "String::default")]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, PartialOrd, Clone)]
pub struct Miscellaneous {
    #[serde(rename = "miscellaneous-field", default = "Vec::default")]
    pub miscellaneous_fields: Vec<MiscellaneousField>,
}

// https://www.w3.org/2021/06/musicxml40/musicxml-reference/elements/identification/
#[derive(Debug, Serialize, Deserialize, Default, PartialEq, PartialOrd)]
pub struct Identification {
    #[serde(rename = "creator", default = "Vec::default")]
    pub creators: Vec<TypedContent>,

    #[serde(rename = "rights", default = "Vec::default")]
    pub rights: Vec<TypedContent>,

    #[serde(rename = "encoding", default = "Option::default")]
    pub encoding: Option<Encoding>,

    #[serde(rename = "source", default = "Option::default")]
    pub source: Option<String>,

    #[serde(rename = "relation", default = "Option::default")]
    pub relation: Option<String>,

    #[serde(rename = "miscellaneous", default = "Option::default")]
    pub miscellaneous: Option<Miscellaneous>,
}

#[cfg(test)]
mod tests {
    use crate::musicxml::identification::{Encoding, TypedContent};
    use serde_xml_rs::from_str;

    use super::Identification;

    #[test]
    fn identification() {
        let xml = r#"
            <identification>
                <creator type="composer">Claude Debussy</creator>
                <creator type="lyricist">Paul Bourget</creator>
                <rights>Copyright © 2010 Recordare LLC</rights>
                <encoding>
                    <encoder>Mark D. Lew</encoder>
                    <encoding-date>2010-12-17</encoding-date>
                    <software>Finale for Windows</software>
                    <encoding-description>MusicXML example</encoding-description>
                    <supports element="accidental" type="yes"/>
                    <supports element="beam" type="yes"/>
                    <supports element="stem" type="yes"/>
                </encoding>
                <source>Based on E. Girod edition of 1891, republished by Dover in 1981.</source>
                <relation>urn:ISBN:0-486-24131-9</relation>
                <miscellaneous>
                    <miscellaneous-field name="difficulty-level">3</miscellaneous-field>
                </miscellaneous>
            </identification>
        "#;

        let item: Identification = from_str(xml).unwrap();

        let composer = &item.creators[0];
        let lyricist = &item.creators[1];
        let rights = &item.rights[0];
        let encoding_check = Encoding {
            encoding_date: Some("2010-12-17".to_string()),
            encoder: vec![TypedContent {
                r#type: None,
                content: "Mark D. Lew".to_string(),
            }],
            software: Some("Finale for Windows".to_string()),
            encoding_description: Some("MusicXML example".to_string()),
        };
        let encoding = item.encoding.unwrap();

        assert_eq!(composer.r#type.clone().unwrap(), "composer".to_string());
        assert_eq!(composer.content, "Claude Debussy".to_string());

        assert_eq!(lyricist.r#type.clone().unwrap(), "lyricist".to_string());
        assert_eq!(lyricist.content, "Paul Bourget".to_string());

        assert_eq!(rights.r#type.clone(), None);
        assert_eq!(rights.content, "Copyright © 2010 Recordare LLC".to_string());

        assert_eq!(encoding, encoding_check);

        assert_eq!(
            item.source.unwrap(),
            "Based on E. Girod edition of 1891, republished by Dover in 1981.".to_string()
        );
        assert_eq!(item.relation.unwrap(), "urn:ISBN:0-486-24131-9".to_string());

        let misc = &item.miscellaneous.unwrap();
        assert_eq!(misc.miscellaneous_fields.len(), 1);
        assert_eq!(
            misc.miscellaneous_fields[0].name,
            "difficulty-level".to_string()
        );
        assert_eq!(misc.miscellaneous_fields[0].content, "3".to_string());
    }
}
