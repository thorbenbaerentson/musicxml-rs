use crate::musicxml::{
    credit::Credit,
    defaults::Defaults,
    identification::Identification,
    part::Part,
    score_part::ScorePart,
    work::{parse_work, Work},
};
use crate::prelude::*;

#[derive(Debug)]
pub struct ScorePartwise {
    pub version: String,
    pub parts: Vec<Part>,
    pub partlist: Vec<ScorePart>,
    pub work: Option<Work>,
    pub identification: Option<Identification>,
    pub defaults: Option<Defaults>,
    pub credits: Vec<Credit>,
}
