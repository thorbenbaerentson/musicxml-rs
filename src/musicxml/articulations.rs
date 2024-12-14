use crate::musicxml::core::Placement;
use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct ArticulationMeta {
    #[serde(default = "Option::default")]
    pub r#type: Option<String>,

    #[serde(default = "Option::default")]
    pub color: Option<String>,

    #[serde(default = "Option::default", rename = "default-x")]
    pub default_x: Option<f32>,

    #[serde(default = "Option::default", rename = "default-y")]
    pub default_y: Option<f32>,

    #[serde(default = "Option::default", rename = "font-family")]
    pub font_family: Option<String>,

    #[serde(default = "Option::default", rename = "font-size")]
    pub font_size: Option<f32>,

    #[serde(default = "Option::default", rename = "font-style")]
    pub font_style: Option<String>,

    #[serde(default = "Option::default", rename = "font-weight")]
    pub font_weigth: Option<String>,

    #[serde(default = "Option::default")]
    pub placement: Option<Placement>,

    #[serde(default = "Option::default", rename = "relative-x")]
    pub relative_x: Option<f32>,

    #[serde(default = "Option::default", rename = "relative-y")]
    pub relative_y: Option<f32>,
}

#[derive(Debug, EnumString, PartialEq, Clone, Serialize, Deserialize)]
pub enum ArticulationType {
    #[strum(serialize = "accent")]
    #[serde(rename = "accent")]
    Accent(ArticulationMeta),

    #[strum(serialize = "strong-accent")]
    #[serde(rename = "strong-accent")]
    StrongAccent(ArticulationMeta),

    #[strum(serialize = "staccato")]
    #[serde(rename = "staccato")]
    Staccato(ArticulationMeta),

    #[strum(serialize = "tenuto")]
    #[serde(rename = "tenuto")]
    Tenuto(ArticulationMeta),

    #[strum(serialize = "detached-legato")]
    #[serde(rename = "detached-legato")]
    DetachedLegato(ArticulationMeta),

    #[strum(serialize = "staccatissimo")]
    #[serde(rename = "staccatissimo")]
    Staccatissimo(ArticulationMeta),

    #[strum(serialize = "spiccato")]
    #[serde(rename = "spiccato")]
    Spiccato(ArticulationMeta),

    #[strum(serialize = "scoop")]
    #[serde(rename = "scoop")]
    Scoop(ArticulationMeta),

    #[strum(serialize = "plop")]
    #[serde(rename = "plop")]
    Plop(ArticulationMeta),

    #[strum(serialize = "doit")]
    #[serde(rename = "doit")]
    Doit(ArticulationMeta),

    #[strum(serialize = "falloff")]
    #[serde(rename = "falloff")]
    Falloff(ArticulationMeta),

    #[strum(serialize = "breath-mark")]
    #[serde(rename = "breath-mark")]
    BreathMark(ArticulationMeta),

    #[strum(serialize = "caesura")]
    #[serde(rename = "caesura")]
    Caesura(ArticulationMeta),

    #[strum(serialize = "stress")]
    #[serde(rename = "stress")]
    Stress(ArticulationMeta),

    #[strum(serialize = "unstress")]
    #[serde(rename = "unstress")]
    Unstress(ArticulationMeta),

    #[strum(serialize = "soft-accent")]
    #[serde(rename = "soft-accent")]
    SoftAccent(ArticulationMeta),

    #[strum(serialize = "other-articulation")]
    #[serde(rename = "sother-articulation")]
    OtherArticulation(ArticulationMeta),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Articulations {
    #[serde(rename = "$value")]
    pub articulations: Vec<ArticulationType>,
}

#[cfg(test)]
mod tests {
    use super::{ArticulationType, Articulations};
    use crate::musicxml::articulations::ArticulationMeta;
    use crate::musicxml::core::Placement;
    use crate::musicxml::note::Note;
    use crate::prelude::*;
    use roxmltree::Document;
    use serde::{Deserialize, Serialize};
    use serde_xml_rs::{from_str, to_string};

    #[test]
    fn articluation() {
        let mut xml = r#"<staccato placement="below"/>"#;
        let mut art: ArticulationType = from_str(xml).unwrap();
        assert_eq!(
            art,
            ArticulationType::Staccato(ArticulationMeta {
                placement: Some(Placement::Below),
                ..ArticulationMeta::default()
            })
        );

        xml = r#"<tenuto placement="below"/>"#;
        art = from_str(xml).unwrap();
        assert_eq!(
            art,
            ArticulationType::Tenuto(ArticulationMeta {
                placement: Some(Placement::Below),
                ..ArticulationMeta::default()
            })
        );

        xml = r#"<accent placement="below"/>"#;
        art = from_str(xml).unwrap();
        assert_eq!(
            art,
            ArticulationType::Accent(ArticulationMeta {
                placement: Some(Placement::Below),
                ..ArticulationMeta::default()
            })
        );

        xml = r#"<strong-accent placement="above" type="up"/>"#;
        art = from_str(xml).unwrap();
        assert_eq!(
            art,
            ArticulationType::StrongAccent(ArticulationMeta {
                placement: Some(Placement::Above),
                r#type: Some("up".to_string()),
                ..ArticulationMeta::default()
            })
        );
    }

    #[test]
    fn accent() {
        let xml = r#"
        <note default-x="36">
            <pitch>
                <step>A</step>
                <octave>4</octave>
            </pitch>
            <duration>4</duration>
            <voice>1</voice>
            <type>half</type>
            <stem default-y="10">up</stem>
            <notations>
                <articulations>
                    <accent default-x="-1" default-y="-55" placement="below"/>
                </articulations>
            </notations>
        </note>"#;

        let arts: Note = from_str(xml).unwrap();
    }
    #[test]
    fn articulations() -> Result<()> {
        let xml = r#"<articulations>
            <staccato placement="below"/>
            <tenuto placement="below"/>
            <accent placement="below"/>
            <strong-accent placement="above" type="up"/>
        </articulations>"#;
        let arts: Articulations = from_str(xml).unwrap();

        let mut art = &arts.articulations[0];
        assert_eq!(
            art,
            &ArticulationType::Staccato(ArticulationMeta {
                placement: Some(Placement::Below),
                ..ArticulationMeta::default()
            })
        );

        art = &arts.articulations[1];
        assert_eq!(
            art,
            &ArticulationType::Tenuto(ArticulationMeta {
                placement: Some(Placement::Below),
                ..ArticulationMeta::default()
            })
        );

        art = &arts.articulations[2];
        assert_eq!(
            art,
            &ArticulationType::Accent(ArticulationMeta {
                placement: Some(Placement::Below),
                ..ArticulationMeta::default()
            })
        );

        art = &arts.articulations[3];
        assert_eq!(
            art,
            &ArticulationType::StrongAccent(ArticulationMeta {
                placement: Some(Placement::Above),
                r#type: Some("up".to_string()),
                ..ArticulationMeta::default()
            })
        );

        Ok(())
    }
}
