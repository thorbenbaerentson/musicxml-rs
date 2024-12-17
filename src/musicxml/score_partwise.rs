use serde::{Deserialize, Serialize};

use crate::musicxml::{
    credit::Credit, defaults::Defaults, identification::Identification, score_part::ScorePart,
    work::Work,
};
use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScorePartwise {
    #[serde(default = "Option::default")]
    pub work: Option<Work>,

    #[serde(rename = "movement-number", default = "Option::default")]
    pub movement_number: Option<String>,

    #[serde(rename = "movement-title", default = "Option::default")]
    pub movement_title: Option<String>,

    #[serde(default = "String::default")]
    pub version: String,

    #[serde(rename = "part", default = "Vec::default")]
    pub parts: Vec<ScorePart>,

    #[serde(default = "Option::default")]
    pub identification: Option<Identification>,

    #[serde(default = "Option::default")]
    pub defaults: Option<Defaults>,

    #[serde(default = "Vec::default")]
    pub credits: Vec<Credit>,
}

#[cfg(test)]
mod tests {
    use super::ScorePartwise;
    use serde_xml_rs::from_str;
    use std::fs;

    #[test]
    fn score_partwise_1() {
        let xml = fs::read_to_string("resources/xml-test-files/01a-Pitches-Pitches.xml").unwrap();
        let item: ScorePartwise = from_str(&xml).unwrap();

        assert_eq!(
            item.movement_title.unwrap(),
            "Pitches and accidentals".to_string()
        );

        let ident = &item.identification.unwrap();
        let misc = ident.miscellaneous.clone().unwrap();
        assert_eq!(misc.miscellaneous_fields[0].name, "description".to_string());
        assert!(misc.miscellaneous_fields[0]
            .content
            .starts_with("All pitches"));
    }
}
