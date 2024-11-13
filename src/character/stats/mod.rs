use super::talent;
use crate::{damage, element::reaction::ElementalReaction};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Stats {
    data: HashMap<Type, f64>,
}

impl Stats {
    pub fn get_stat(&self, key: Type) -> f64 {
        match self.data.get(&key) {
            Some(value) => value.to_owned(),
            None => 0.0,
        }
    }
    pub fn add_stat(&mut self, stat: &Stat) {
        self.data.insert(stat.typ(), stat.val());
    }
}

impl<const N: usize> From<[(Type, f64); N]> for Stats {
    fn from(arr: [(Type, f64); N]) -> Self {
        Self {
            data: HashMap::from(arr),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stat {
    typ: Type,
    val: f64,
}
impl Stat {
    pub fn new(typ: Type, val: f64) -> Self {
        Self { typ, val }
    }

    pub fn typ(&self) -> Type {
        self.typ
    }
    pub fn val(&self) -> f64 {
        self.val
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Type {
    Level,

    Hp,
    Atk,
    Def,

    ElementalMastery,

    CritRate,
    CritDmg,

    EnergyRecharge,

    CooldownReduction,

    ShieldStrength,

    HealingBonus,
    IncomingHealingBonus,

    DMGBonus(DMGBonusType),

    AttributeRES(damage::Attribute),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DMGBonusType {
    Universal,
    Attribute(damage::Attribute),
    Type(damage::Type),
    Talent(talent::Type),
    Reaction(ElementalReaction),
}
