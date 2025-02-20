use crate::{
    damage::{self, Attribute},
    element::reaction::ElementalReaction,
};
use std::collections::HashMap;

use super::talent::Talent;

#[derive(Clone, Debug, PartialEq)]
pub struct Stats {
    data: HashMap<Type, f64>,
}

impl Stats {
    /// Gets the stat associated with the key. Stats that are not in the map are considered to be 0.0.
    pub fn get_stat(&self, key: Type) -> f64 {
        self.data.get(&key).unwrap_or(&0.0).to_owned()
    }
    /// Adds a stat to the stats. If the stat is already present, the values will be added.
    pub fn add_stat(&mut self, stat: &Stat) {
        *self.data.entry(stat.typ()).or_insert(0.0) += stat.val();
    }

    /// Aggregates the relevant dmg multipliers for a talent
    pub fn tal_dmg_mult(&self, talent: &Talent) -> f64 {
        talent
            .conditions_met()
            .into_iter()
            .fold(1.0, |a, condition| {
                a + self.get_stat(Type::DMGMult(condition))
            })
    }

    /// Aggregates the relevant base dmg multipliers for a talent
    pub fn tal_base_dmg_mult(&self, talent: &Talent) -> f64 {
        talent
            .conditions_met()
            .into_iter()
            .fold(1.0, |a, cond| a + self.get_stat(Type::BaseDMGMult(cond)))
    }

    /// Aggregates the relevant base dmg multipliers for a talent
    pub fn tal_base_dmg_flat(&self, talent: &Talent) -> f64 {
        talent
            .conditions_met()
            .into_iter()
            .fold(1.0, |a, cond| a + self.get_stat(Type::BaseDMGFlat(cond)))
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

    DMGMult(Option<Condition>),

    // You can only have resistance to attributes for some reason, not any general condition
    ResMult(Attribute),

    BaseDMGMult(Option<Condition>),
    BaseDMGFlat(Option<Condition>),

    // Reaction DMG bonuses are dealt with in the Reaction effect, and thus
    // I think they warrant their own type of stat rather than being handled with any condition.
    RxnDMGMult(ElementalReaction),

    // TODO - Def Ignore? Def Shred? Enemy Debuffs? Character Temporary Buffs?
    // Def ignore should be a character stat because it's character-specific.
    // Def shred and Def Ignore stack multiplicatively
    DefIgnore(Option<Condition>),
}

// For Stats that only apply to some types of damage
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Condition {
    Attribute(damage::Attribute),
    Category(damage::Category),
}
