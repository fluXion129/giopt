use std::hash::Hash;

use crate::{
    calculator::{CalcKey, Calculator},
    damage::Attribute,
    element::reaction::ElementalReaction,
    stats::{Stat, StatSheet, Type as StatType},
};

/// Genshin Calc Keys
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum GCK {
    B(B),
    L(L),
}

impl CalcKey for GCK {}

// This is for convenience of inputting character stats.
impl From<StatType> for GCK {
    fn from(value: StatType) -> Self {
        Self::L(L::Stat(value))
    }
}

/// Branch Genshin Calc Keys - These are typically calculated from the Leaf Keys,
/// but if you want to override the functionality of the calculator, it can be
/// useful to manipulate these.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum B {
    DamageInstanceOutput,

    BaseDMGFinal,
    BaseDMGPostMult,
    BaseDMGAdd,
    BaseDMGMult,
    BaseDMG,
    EvalScaling(S),

    DMGBonusMult,
    AttributeDMGBonusMult,
    CategoryDMGBonusMult,

    TargetDEFMult,
    TotalDEFIgnore,

    TargetRESMult,
    TargetRESFinal,
    TargetBaseRES,
    TargetAttributeRES,
    TargetAttributeRESReductNeg,
    TargetAttributeRESReduct,

    AmpRxnMult,
    PotentialAmpRxnMult,
    AmpRxnTotalBonusMult,
    AmpRxnEMMult,
    AmpRxnBonusMult,

    CritMult,
    TotalCritRate,
    TotalCritDMG,
    AttributeCritDMG,
}

/// Leaf Genshin Calc Keys
/// Have no rule associated with them - the keys you will need to put values for.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum L {
    Attribute,
    Category,

    Scaling(S),
    BaseDMGAdd,

    Stat(StatType),

    TargetDMGBonusMult,

    TargetLevel,
    // This needs to be positive!
    TargetDEFReduct,
    TargetAttributeRES(Attribute),
    // This needs to be negative!
    TargetAttributeRESReduct(Attribute),

    BaseAmpRxnMult,
    AmpRxnType,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum S {
    Atk,
    Def,
    MaxHP,
    EM,
}

/// Contains the actual definition of the relations between GCKs.
pub mod gi_rules_def;
pub use gi_rules_def::GI_RULES;

// Helpful additional methods for calculators using GCK, in other words, genshin damage calculators.
impl Calculator<'_, GCK> {
    pub fn add_character_stat(&mut self, stat: Stat) {
        self.set(stat.typ().into(), stat.val())
    }

    /// To import an existing stat sheet into a calculator. Note that this currently
    /// clones all the values from that statsheet into the calculator. I wonder if
    /// it would be possible to have an external statsheet to look up. It may be
    /// overly difficult though, likely requiring a significant rewrite.
    pub fn import_stat_sheet(&mut self, statsheet: &StatSheet) {
        for (&st, &sv) in statsheet.data() {
            self.set(st.into(), sv);
        }
    }

    // TODO - Integrating various stat sources into the calculator.
    // One of the things that I need to make in order for this system to really take advantage of
    // its capabilities is more detailed descriptions of stat breakdowns. Being able to recompute
    // the total stats from pieces. I suppose it could be a separate calculator as well.

    /// Sets up the calculator for a specific reaction or lack of reaction.
    ///
    /// Sets AmpRxnType, BaseAmpRxnMult
    pub fn set_rxn(&mut self, rxn: Option<ElementalReaction>) {
        self.set(
            GCK::L(L::AmpRxnType),
            ElementalReaction::amp_rxn_type_calcindex(rxn),
        );
        self.set(
            GCK::L(L::BaseAmpRxnMult),
            ElementalReaction::amp_rxn_mult(rxn),
        );
    }
}
