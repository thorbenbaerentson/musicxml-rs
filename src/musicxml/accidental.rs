use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, PartialOrd, Clone)]
pub enum Accidental {
    #[default]
    None,

    #[serde(rename = "sharp")]
    Sharp,

    #[serde(rename = "natural")]
    Natural,

    #[serde(rename = "flat")]
    Flat,

    #[serde(rename = "double-sharp")]
    DoubleSharp,

    #[serde(rename = "sharp-sharp")]
    SharpSharp,

    #[serde(rename = "flat-flat")]
    FlatFlat,

    #[serde(rename = "natural-sharp")]
    NaturalSharp,

    #[serde(rename = "natural-flat")]
    NaturalFlat,

    #[serde(rename = "quarter-flat")]
    QuarterFlat,

    #[serde(rename = "quarter-sharp")]
    QuarterSharp,

    #[serde(rename = "three-quarters-flat")]
    ThreeQuartersFlat,

    #[serde(rename = "three-quarters-sharp")]
    ThreeQuartersSharp,

    #[serde(rename = "sharp-down")]
    SharpDown,

    #[serde(rename = "sharp-up")]
    SharpUp,

    #[serde(rename = "natural-down")]
    NaturalDown,

    #[serde(rename = "natural-up")]
    NaturalUp,

    #[serde(rename = "flat-down")]
    FlatDown,

    #[serde(rename = "flat-up")]
    FlatUp,

    #[serde(rename = "double-sharp-down")]
    DoubleSharpDown,

    #[serde(rename = "double-sharp-up")]
    DoubleSharpUp,

    #[serde(rename = "flat-flat-down")]
    FlatFlatDown,

    #[serde(rename = "flat-flat-up")]
    FlatFlatUp,

    #[serde(rename = "arrow-down")]
    ArrowDown,

    #[serde(rename = "arrow-up")]
    ArrowUp,

    #[serde(rename = "triple-sharp")]
    TripleSharp,

    #[serde(rename = "triple-flat")]
    TripleFlat,

    #[serde(rename = "slash-quarter-sharp")]
    SlashQuarterSharp,

    #[serde(rename = "slash-sharp")]
    SlashSharp,

    #[serde(rename = "slash-flat")]
    SlashFlat,

    #[serde(rename = "double-slash-flat")]
    DoubleSlashFlat,

    #[serde(rename = "sharp-1")]
    Sharp1,

    #[serde(rename = "sharp-2")]
    Sharp2,

    #[serde(rename = "sharp-3")]
    Sharp3,

    #[serde(rename = "sharp-4")]
    Sharp4,

    #[serde(rename = "sharp-5")]
    Sharp5,

    #[serde(rename = "flat-1")]
    Flat1,

    #[serde(rename = "flat-2")]
    Flat2,

    #[serde(rename = "flat-3")]
    Flat3,

    #[serde(rename = "flat-4")]
    Flat4,

    #[serde(rename = "flat-5")]
    Flat5,

    #[serde(rename = "sori")]
    Sori,

    #[serde(rename = "koron")]
    Koron,

    #[serde(rename = "other")]
    Other,
}
