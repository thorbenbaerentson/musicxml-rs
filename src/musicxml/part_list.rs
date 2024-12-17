use super::{part_group::PartGroup, printable_value::PrintableValue, score_part::ScorePart};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum PartListContent {
    #[serde(rename = "part-group")]
    PartGroup(#[serde(default = "PartGroup::default")] PartGroup),

    #[serde(rename = "score-part")]
    ScorePart(#[serde(default = "ScorePart::default")] ScorePart),

    #[serde(rename = "part-name")]
    PartName(#[serde(default = "PrintableValue::default")] PrintableValue<String>),
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PartList {
    #[serde(rename = "$value", default = "Vec::default")]
    pub parts: Vec<PartListContent>,
}

#[cfg(test)]
mod tests {
    use crate::musicxml::{part_list::PartListContent, score_part::ScorePartContent};

    use super::PartList;
    use serde_xml_rs::from_str;

    #[test]
    fn part_list() {
        let xml = r#"
        <part-list>
            <score-part id="P1">
                <part-name>Soprano Alto</part-name>
                <part-name-display>
                    <display-text xml:space="preserve">Soprano Alto</display-text>
                </part-name-display>
                <score-instrument id="P1-I1">
                    <instrument-name>ARIA Player</instrument-name>
                    <instrument-sound>voice.soprano</instrument-sound>
                </score-instrument>
                <score-instrument id="P1-I2">
                    <instrument-name>ARIA Player</instrument-name>
                    <instrument-sound>voice.alto</instrument-sound>
                </score-instrument>
                <player id="P1-M1">
                    <player-name>Soprano 1</player-name>
                </player>
                <player id="P1-M2">
                    <player-name>Soprano 2</player-name>
                </player>
                <player id="P1-M3">
                    <player-name>Alto 1</player-name>
                </player>
                <player id="P1-M4">
                    <player-name>Alto 2</player-name>
                </player>
            </score-part>
        </part-list>
        "#;

        let item: PartList = from_str(xml).unwrap();
        assert_eq!(item.parts.len(), 1);

        let part = &item.parts[0];
        match &item.parts[0] {
            PartListContent::ScorePart(p) => {
                match &p.content[0] {
                    ScorePartContent::PartName(n) => {
                        assert_eq!(n, &"Soprano Alto".to_string());
                    },
                    _ => { panic!("Expected different item"); }
                }

                match &p.content[1] {
                    ScorePartContent::PartNameDisplay(n) => {
                        assert_eq!(n.display_text, "Soprano Alto".to_string());
                    },
                    _ => { panic!("Expected different item"); }
                }

                match &p.content[2] {
                    ScorePartContent::ScoreInstrument(inst_1) => {
                        assert_eq!(inst_1.id, "P1-I1".to_string());
                        assert_eq!(inst_1.instrument_name, "ARIA Player".to_string());
                        assert_eq!(inst_1.instrument_sound, "voice.soprano".to_string());
                    },
                    _ => { panic!("Expected different item"); }
                }

                match &p.content[3] {
                    ScorePartContent::ScoreInstrument(inst_2) => {
                        assert_eq!(inst_2.id, "P1-I2".to_string());
                        assert_eq!(inst_2.instrument_name, "ARIA Player".to_string());
                        assert_eq!(inst_2.instrument_sound, "voice.alto".to_string());
                    },
                    _ => { panic!("Expected different item"); }
                }

                match &p.content[4] {
                    ScorePartContent::Player(p) => {
                        assert_eq!(p.id, "P1-M1".to_string());
                        assert_eq!(p.player_name, "Soprano 1".to_string());
                    },
                    _ => { panic!("Expected different item"); }
                }

                match &p.content[5] {
                    ScorePartContent::Player(p) => {
                        assert_eq!(p.id, "P1-M2".to_string());
                        assert_eq!(p.player_name, "Soprano 2".to_string());
                    },
                    _ => { panic!("Expected different item"); }
                }

                match &p.content[6] {
                    ScorePartContent::Player(p) => {
                        assert_eq!(p.id, "P1-M3".to_string());
                        assert_eq!(p.player_name, "Alto 1".to_string());
                    },
                    _ => { panic!("Expected different item"); }
                }

                match &p.content[7] {
                    ScorePartContent::Player(p) => {
                        assert_eq!(p.id, "P1-M4".to_string());
                        assert_eq!(p.player_name, "Alto 2".to_string());
                    },
                    _ => { panic!("Expected different item"); }
                }
            }

            _ => {
                panic!("Unexpected item");
            }
        }
    }
}
