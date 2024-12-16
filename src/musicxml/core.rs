use super::articulations::ArticulationType;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum_macros::EnumString;
pub type Duration = usize;

#[derive(Debug, EnumString, PartialEq, Serialize, Deserialize, PartialOrd, Default)]
pub enum DurationType {
    #[strum(serialize = "64th")]
    #[serde(rename = "64th")]
    #[default]
    Sixtyfourth,

    #[strum(serialize = "32nd")]
    #[serde(rename = "32nd")]
    Thirtysecond,

    #[strum(serialize = "16th")]
    #[serde(rename = "16th")]
    Sixteenth,

    #[strum(serialize = "eighth")]
    #[serde(rename = "eighth")]
    Eighth,

    #[strum(serialize = "quarter")]
    #[serde(rename = "quarter")]
    Quarter,

    #[strum(serialize = "half")]
    #[serde(rename = "half")]
    Half,

    #[strum(serialize = "whole")]
    #[serde(rename = "whole")]
    Whole,

    #[strum(serialize = "breve")]
    #[serde(rename = "breve")]
    Breve,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Default)]
pub enum DirectionUD {
    #[serde(rename = "up")]
    Up,

    #[serde(rename = "down")]
    Down,

    #[serde(rename = "double")]
    Double,

    #[serde(rename = "none")]
    #[default]
    None,
}

#[derive(Debug, EnumString, PartialEq, Serialize, Deserialize, PartialOrd, Default)]
pub enum SyllabicType {
    #[strum(serialize = "begin")]
    #[serde(rename = "begin")]
    #[default]
    Begin,

    #[strum(serialize = "end")]
    #[serde(rename = "end")]
    End,

    #[strum(serialize = "single")]
    #[serde(rename = "single")]
    Single,
}

#[derive(Debug, EnumString, PartialEq, Clone, Serialize, Deserialize, PartialOrd, Default)]
pub enum Placement {
    #[strum(serialize = "above")]
    #[serde(rename = "above")]
    #[default]
    Above,

    #[strum(serialize = "below")]
    #[serde(rename = "below")]
    Below,
}

#[derive(Debug, EnumString, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum RepeatDirection {
    #[strum(serialize = "forward")]
    Forward,

    #[strum(serialize = "backward")]
    Backward,
}

// https://www.w3.org/2021/06/musicxml40/musicxml-reference/examples/tutorial-percussion/
// https://www.w3.org/2021/06/musicxml40/musicxml-reference/examples/tutorial-tablature/
// https://www.w3.org/2021/06/musicxml40/musicxml-reference/examples/tutorial-chord-symbols/
// https://www.w3.org/2021/06/musicxml40/musicxml-reference/examples/tutorial-chopin-prelude/
#[cfg(test)]
mod test_core {
    use crate::musicxml::articulations::{ArticulationMeta, ArticulationType, Articulations};
    use crate::musicxml::core::Placement;
    use crate::musicxml::note::NotationType;
    use crate::prelude::*;
    use std::str::FromStr;
    use strum_macros::EnumString;

    #[test]
    fn example() {
        assert_eq!(
            ArticulationType::Tenuto(ArticulationMeta::default(),),
            ArticulationType::from_str("tenuto").unwrap()
        );
        assert_eq!(Placement::Above, Placement::from_str("above").unwrap());
        assert_eq!(
            NotationType::Articulations(Articulations {
                articulations: vec![ArticulationType::Tenuto(ArticulationMeta::default(),)]
            }),
            NotationType::Articulations(Articulations {
                articulations: vec![ArticulationType::Tenuto(ArticulationMeta::default(),)]
            })
        );
    }
}
