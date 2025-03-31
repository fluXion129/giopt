use crate::{
    damage,
    element::{reaction::ElementalReaction, Element},
};
use std::{
    collections::HashMap,
    ops::{Mul, MulAssign},
};

#[derive(Clone, Debug, PartialEq)]
pub struct StatSheet {
    data: HashMap<Type, f64>,
}

impl StatSheet {
    /// Gets the stat associated with the key. Stats that are not in the map are considered to be 0.0.
    pub fn get(&self, key: Type) -> f64 {
        self.data.get(&key).unwrap_or(&0.0).to_owned()
    }

    /// Sums all multipliers for the keys in the given iterator
    ///
    /// ```
    /// use giopt::{stats::{StatSheet, Type::DMGMult, Condition}, damage::Category::*};
    /// let stats = StatSheet::from([
    ///     (DMGMult(None), 0.25),
    ///     (DMGMult(Some(Condition::Category(NormalAttack))), 0.25),
    ///     (DMGMult(Some(Condition::Category(ChargedAttack))), 0.50)
    /// ]);
    /// let keys = vec![DMGMult(None), DMGMult(Some(Condition::Category(NormalAttack)))];
    ///
    /// assert_eq!(stats.sum_mults(keys.into_iter()), 1.5);
    /// ```
    pub fn sum_mults(&self, keys: impl Iterator<Item = Type>) -> f64 {
        keys.fold(1.0, |a, x| a + self.get(x))
    }

    /// Adds a stat to the stats. If the stat is already present, the values will be added.
    pub fn add_stat(&mut self, stat: &Stat) {
        *self.data.entry(stat.typ()).or_insert(0.0) += stat.val();
    }
}

impl<const N: usize> From<[(Type, f64); N]> for StatSheet {
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
impl Mul<f64> for &Stat {
    type Output = Stat;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            typ: self.typ,
            val: self.val * rhs,
        }
    }
}
impl MulAssign<f64> for Stat {
    fn mul_assign(&mut self, rhs: f64) {
        self.val *= rhs;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Type {
    Level,

    MaxHP,
    Atk,
    Def,

    ElementalMastery,

    // Conditional Crit Stats exist. It's actually possible
    // that all stats can be conditional... Not sure what to
    // do with that.
    CritRate,
    CritDmg,

    EnergyRecharge,

    CooldownReduction,

    ShieldStrength,

    HealingBonus,
    IncomingHealingBonus,

    DMGMult(Option<Condition>),

    // You can only have resistance to attributes for some reason, not any general condition
    AttributeRES(damage::Attribute),

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
impl From<damage::Attribute> for Condition {
    fn from(value: damage::Attribute) -> Self {
        Self::Attribute(value)
    }
}
impl From<Element> for Condition {
    fn from(value: Element) -> Self {
        Self::Attribute(damage::Attribute::Elemental(value))
    }
}
impl From<damage::Category> for Condition {
    fn from(value: damage::Category) -> Self {
        Self::Category(value)
    }
}
