use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, PartialOrd)]
pub enum Dynamics {
    #[serde(rename = "p")]
    #[default]
    P,

    #[serde(rename = "pp")]
    PP,

    #[serde(rename = "ppp")]
    PPP,

    #[serde(rename = "pppp")]
    PPPP,

    #[serde(rename = "ppppp")]
    PPPPP,

    #[serde(rename = "pppppp")]
    PPPPPP,

    #[serde(rename = "f")]
    F,

    #[serde(rename = "ff")]
    FF,

    #[serde(rename = "fff")]
    FFF,

    #[serde(rename = "ffff")]
    FFFF,

    #[serde(rename = "fffff")]
    FFFFF,

    #[serde(rename = "ffffff")]
    FFFFFF,

    #[serde(rename = "mp")]
    MP,

    #[serde(rename = "mf")]
    MF,

    #[serde(rename = "sf")]
    SF,

    #[serde(rename = "sfp")]
    SFP,

    #[serde(rename = "sfpp")]
    SFPP,

    #[serde(rename = "fp")]
    FP,

    #[serde(rename = "rf")]
    RF,

    #[serde(rename = "rfz")]
    RFZ,

    #[serde(rename = "sfz")]
    SFZ,

    #[serde(rename = "sffz")]
    SFFZ,

    #[serde(rename = "fz")]
    FZ,

    #[serde(rename = "n")]
    N,

    #[serde(rename = "pf")]
    PF,

    #[serde(rename = "sfzp")]
    SFZP,

    #[serde(rename = "other-dynamics")]
    OtherDynamics
}