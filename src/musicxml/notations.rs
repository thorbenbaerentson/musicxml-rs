use crate::prelude::*;
use roxmltree::{Node, NodeType};
use std::str::FromStr;

use super::{
    articulations::Articulations,
    note::{NotationType, Notations, StartStop},
};

#[cfg(test)]
mod tests {
    use crate::musicxml::{
        articulations::{ArticulationMeta, Articulations},
        core::Placement,
        note::{NotationType, Notations},
    };
    use roxmltree::Document;
    use serde_xml_rs::from_str;

    #[test]
    fn notations() {
        let xml = r#"
        <notations>
            <articulations>
                <staccato placement="below"/>
                <tenuto placement="below"/>
                <accent placement="below"/>
                <strong-accent placement="above" type="up"/>
            </articulations>
        </notations>"#;

        let items: Notations = from_str(xml).unwrap();
        assert_eq!(items.notations.len(), 1);

        match &items.notations[0] {
            NotationType::Articulations(articulations) => {
                assert_eq!(articulations.articulations.len(), 4);

                match &articulations.articulations[0] {
                    crate::musicxml::articulations::ArticulationType::Staccato(
                        ArticulationMeta {
                            r#type,
                            color,
                            default_x,
                            default_y,
                            font_family,
                            font_size,
                            font_style,
                            font_weigth,
                            placement,
                            relative_x,
                            relative_y,
                        },
                    ) => {
                        assert_eq!(placement.clone().unwrap(), Placement::Below);
                    }
                    _ => {
                        panic!("Expected staccto")
                    }
                }

                match &articulations.articulations[1] {
                    crate::musicxml::articulations::ArticulationType::Tenuto(
                        ArticulationMeta {
                            r#type,
                            color,
                            default_x,
                            default_y,
                            font_family,
                            font_size,
                            font_style,
                            font_weigth,
                            placement,
                            relative_x,
                            relative_y,
                        },
                    ) => {
                        assert_eq!(placement.clone().unwrap(), Placement::Below);
                    }
                    _ => {
                        panic!("Expected tenuto")
                    }
                }

                match &articulations.articulations[2] {
                    crate::musicxml::articulations::ArticulationType::Accent(
                        ArticulationMeta {
                            r#type,
                            color,
                            default_x,
                            default_y,
                            font_family,
                            font_size,
                            font_style,
                            font_weigth,
                            placement,
                            relative_x,
                            relative_y,
                        },
                    ) => {
                        assert_eq!(placement.clone().unwrap(), Placement::Below);
                    }
                    _ => {
                        panic!("Expected accent")
                    }
                }

                match &articulations.articulations[3] {
                    crate::musicxml::articulations::ArticulationType::StrongAccent(
                        ArticulationMeta {
                            r#type,
                            color,
                            default_x,
                            default_y,
                            font_family,
                            font_size,
                            font_style,
                            font_weigth,
                            placement,
                            relative_x,
                            relative_y,
                        },
                    ) => {
                        assert_eq!(placement.clone().unwrap(), Placement::Above);
                    }
                    _ => {
                        panic!("Expected accent")
                    }
                }
            }
            _ => {
                panic!("Expected an articulations strong-accent")
            }
        }
    }
}
