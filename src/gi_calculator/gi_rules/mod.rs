use std::hash::Hash;

use crate::{
    calculator::{CalcKey, Calculator},
    damage::Attribute,
    element::reaction::ElementalReaction,
    stats::{Stat, StatSheet, Type as StatType},
};

/// Genshin Calc Keys
///
/// "Reduct" keys should be given positive values
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum GCK {
    DamageInstanceOutput,

    BaseDMGFinal,
    BaseDMGPostMult,
    BaseDMGMult,
    BaseDMG,
    EvalScaling(Scaling),

    DMGBonusMult,
    AttributeDMGBonusMult,
    CategoryDMGBonusMult,

    TargetDEFMult,
    TotalDEFIgnore,

    TargetRESMult,
    TargetRESFinal,
    TargetBaseRES,
    // These keys have to be labelled Sel to avoid clashing with the Individual Attribute-specific RES key name
    SelTargetAttributeRES,
    SelTargetAttributeRESReductNeg,
    SelTargetAttributeRESReduct,

    AmpRxnMult,
    PotentialAmpRxnMult,
    AmpRxnTotalBonusMult,
    AmpRxnEMMult,
    AmpRxnBonusMult,

    CritMult,
    TotalCritRate,
    TotalCritDMG,
    AttributeCritDMG,

    // Past this point are the "Leaf Keys" - Intended for most frequent use
    Attribute,
    Category,

    Scaling(Scaling),
    BaseDMGAdd,

    Stat(StatType),

    TargetDMGBonusMult,

    TargetLevel,
    TargetDEFReduct,
    TargetAttributeRES(Attribute),
    TargetAttributeRESReduct(Attribute),

    BaseAmpRxnMult,
    AmpRxnType,
}

impl CalcKey for GCK {}

// This is for convenience of inputting character stats.
impl From<StatType> for GCK {
    fn from(value: StatType) -> Self {
        Self::Stat(value)
    }
}

/// A type containing all current valid scaling stats. It is likely that other scalings will be implemented.
///
/// I may want scalings to be arbitrary functions, but that doesn't mesh well with the calculator. If I think of a way
/// to do that, this may become obsolete
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Scaling {
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
            GCK::AmpRxnType,
            ElementalReaction::amp_rxn_type_calcindex(rxn),
        );
        self.set(GCK::BaseAmpRxnMult, ElementalReaction::amp_rxn_mult(rxn));
    }
}
