use serde::{Deserialize, Serialize};

use crate::musicxml::{
    credit::Credit, defaults::Defaults, identification::Identification, score_part::ScorePart,
    work::Work,
};
use crate::prelude::*;

use super::part_list::PartList;

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

    #[serde(rename = "part-list", default = "PartList::default")]
    pub part_list: PartList,

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
    use crate::musicxml::part_list::PartListContent;

    use super::ScorePartwise;
    use serde_xml_rs::from_str;
    use core::panic;
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
        assert!(misc.miscellaneous_fields[0].content.starts_with("All pitches"));

        let part_list = item.part_list;
        match &part_list.parts[0] {
            PartListContent::ScorePart(score_part) => {
                assert_eq!(score_part.id, "P1".to_string());
                assert_eq!(score_part.part_name, "MusicXML Part".to_string());
            },
            PartListContent::PartGroup(part_group) => {
                println!("No.: {}", part_list.parts.len());
                panic!("It´s a part group!");
            },
            PartListContent::PartName(printable_value) => {
                println!("No.: {}", part_list.parts.len());
                panic!("It´s a part name!");
            },
        }
    }

    #[test]
    fn score_partwise_2() {
        let xml = fs::read_to_string("resources/xml-test-files/01b-Pitches-Intervals.xml").unwrap();
        let item: ScorePartwise = from_str(&xml).unwrap();
    }

    #[test]
    fn score_partwise_3() {
        let xml = fs::read_to_string("resources/xml-test-files/01c-Pitches-NoVoiceElement.xml").unwrap();
        let item: ScorePartwise = from_str(&xml).unwrap();
    }

    #[test]
    fn score_partwise_4() {
        let xml = fs::read_to_string("resources/xml-test-files/01d-Pitches-Microtones.xml").unwrap();
        let item: ScorePartwise = from_str(&xml).unwrap();        
    }

    #[test]
    fn score_partwise_5() {
        let xml = fs::read_to_string("resources/xml-test-files/01e-Pitches-EditorialCautionaryAccidentals.xml").unwrap();
        let item: ScorePartwise = from_str(&xml).unwrap();        
    }

    #[test]
    fn score_partwise_6() {
        let xml = fs::read_to_string("resources/xml-test-files/01ea-Pitches-Parenthesis-Changed-Accidentals.xml").unwrap();
        let item: ScorePartwise = from_str(&xml).unwrap();        
    }

    // #[test]
    // fn my_bonnie() {
    //     let xml = fs::read_to_string("resources/xml-test-files/my_bonnie.xml").unwrap();
    //     let item: ScorePartwise = from_str(&xml).unwrap();

    // }
}
