use super::{part_group::PartGroup, score_part::ScorePart};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum PartListContent {
    #[serde(rename = "part-group")]
    PartGroup(#[serde(default = "PartGroup::default")] PartGroup),

    #[serde(rename = "score-part")]
    ScorePart(#[serde(default = "ScorePart::default")] ScorePart),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartList {
    #[serde(rename = "$value", default = "Vec::default")]
    pub parts: Vec<PartListContent>,
}

#[cfg(test)]
mod tests {
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
            crate::musicxml::part_list::PartListContent::ScorePart(p) => {
                assert_eq!(p.part_name, "Soprano Alto".to_string());
                assert_eq!(
                    p.part_name_display.clone().unwrap().display_text,
                    "Soprano Alto".to_string()
                );

                let inst_1 = &p.score_instruments[0];
                assert_eq!(inst_1.id, "P1-I1".to_string());
                assert_eq!(inst_1.instrument_name, "ARIA Player".to_string());
                assert_eq!(inst_1.instrument_sound, "voice.soprano".to_string());

                let inst_2 = &p.score_instruments[1];
                assert_eq!(inst_2.id, "P1-I2".to_string());
                assert_eq!(inst_2.instrument_name, "ARIA Player".to_string());
                assert_eq!(inst_2.instrument_sound, "voice.alto".to_string());

                assert_eq!(p.players[0].id, "P1-M1".to_string());
                assert_eq!(p.players[0].player_name, "Soprano 1".to_string());

                assert_eq!(p.players[1].id, "P1-M2".to_string());
                assert_eq!(p.players[1].player_name, "Soprano 2".to_string());

                assert_eq!(p.players[2].id, "P1-M3".to_string());
                assert_eq!(p.players[2].player_name, "Alto 1".to_string());

                assert_eq!(p.players[3].id, "P1-M4".to_string());
                assert_eq!(p.players[3].player_name, "Alto 2".to_string());
            }

            _ => {
                panic!("Unexpected item");
            }
        }
    }
}
