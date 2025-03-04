// TODO - Remove when actually using Characters
#![allow(dead_code)]

pub mod talent;
use talent::TalentSheet;

use crate::equipment::{artifact::Artifact, weapon::Weapon};
use crate::Ascension;

pub struct Character<'a> {
    level: f64,
    ascension: Ascension,
    equipment: &'a EquipSet<'a>,
    talents: &'static TalentSheet,
    constellation: Constellation,
}

pub struct EquipSet<'a> {
    weapon: &'a Weapon,
    artifacts: [Option<&'a Artifact>; 5],
}

enum Constellation {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}
