use std::{collections::HashMap, sync::LazyLock};

use crate::{
    calculator::{
        rules::{mux, mux_or_0, mux_or_1, neg, product, sum, sum_plus_one, Rule, Rules},
        Calculator,
    },
    damage::{Attribute, Category},
    element::{reaction::ElementalReaction, Element},
    stats::{Condition, Type as StatType},
};

use super::{Scaling, GCK};

// Specialized calculator node evaluators

pub fn def_mult(calc: &mut Calculator<GCK>, keys: &[GCK]) -> f32 {
    let c_level = calc.get(
        keys.first()
            .expect("def_mult nodes must have character level first"),
    );
    let e_level = calc.get(
        keys.get(1)
            .expect("def_mult nodes must have enemy level second"),
    );
    let def_reduct = calc.get(
        keys.get(2)
            .expect("def_mult nodes must have DEFReduct third"),
    );
    let def_ignore = calc.get(
        keys.get(3)
            .expect("def_mult nodes must have DEFIgnore fourth"),
    );
    (c_level + 100.0)
        / ((1.0 / (1.0 + def_reduct)) * (1.0 / (1.0 + def_ignore)) * (e_level + 100.0)
            + (c_level + 100.0))
}

pub fn res_mult(calc: &mut Calculator<GCK>, keys: &[GCK]) -> f32 {
    let res = calc.get(
        keys.first()
            .expect("res_mult nodes must have RESFinal as first"),
    );
    if res < 0.0 {
        1.0 - (res / 2.0)
    } else if res < 0.75 {
        1.0 - res
    } else {
        1.0 / (4.0 * res + 1.0)
    }
}

pub fn amp_rxn_em_mult(calc: &mut Calculator<GCK>, keys: &[GCK]) -> f32 {
    let em = calc.get(
        keys.first()
            .expect("amp_rxn_em_mult nodes must have EM first"),
    );
    2.78 * em / (em + 1400.0)
}

pub fn crit_mult(calc: &mut Calculator<GCK>, keys: &[GCK]) -> f32 {
    let cr = calc.get(
        keys.first()
            .expect("crit_mult nodes must have TotalCritRate first"),
    );
    let cdmg = calc.get(
        keys.get(1)
            .expect("crit_mult nodes must have TotalCritDMG first"),
    );
    1.0 + cr * cdmg
}

macro_rules! rule_gen {
    ($($t:expr => $o:ident [$($k:expr),+]);*) => {
        Rules::new(HashMap::from([
            $(
                ($t, Rule::new(&$o, vec![$($k),*])),
            )*
        ]))
    };
}

// NOTE Clippy says that const items should not be interior mutable. Not quite sure what that means
// to be honest, and I think that I should probably do some more research on what LazyLock is actually
// doing here.

pub const GI_RULES: LazyLock<Rules<GCK>> = LazyLock::new(|| {
    rule_gen!(
        // Top level Damage formula
        GCK::DamageInstanceOutput => product[
            GCK::BaseDMGFinal,
            GCK::DMGBonusMult,
            GCK::TargetDEFMult,
            GCK::TargetRESMult,
            GCK::AmpRxnMult,
            GCK::CritMult
        ];

        // Evaluating BaseDMGFinal
        GCK::BaseDMGFinal => sum[
            GCK::BaseDMGPostMult,
            GCK::BaseDMGAdd
        ];
        GCK::BaseDMGPostMult => product[
            GCK::BaseDMG,
            GCK::BaseDMGMult
        ];
        // TODO - ADD Conditional BaseDMG modifiers
        GCK::BaseDMGMult => sum_plus_one[
            GCK::Stat(StatType::BaseDMGMult(Condition::None))
        ];
        GCK::BaseDMG => sum[
            GCK::EvalScaling(Scaling::Atk),
            GCK::EvalScaling(Scaling::MaxHP),
            GCK::EvalScaling(Scaling::Def),
            GCK::EvalScaling(Scaling::EM)
        ];
        GCK::EvalScaling(Scaling::Atk) => product[
            GCK::Scaling(Scaling::Atk),
            GCK::Stat(StatType::Atk)
        ];
        GCK::EvalScaling(Scaling::MaxHP) => product[
            GCK::Scaling(Scaling::MaxHP),
            GCK::Stat(StatType::MaxHP)
        ];
        GCK::EvalScaling(Scaling::Def) => product[
            GCK::Scaling(Scaling::Def),
            GCK::Stat(StatType::Def)
        ];
        GCK::EvalScaling(Scaling::EM) => product[
            GCK::Scaling(Scaling::EM),
            GCK::Stat(StatType::ElementalMastery)
        ];

        // Evaluating DMGBonusMult
        GCK::DMGBonusMult => sum_plus_one[
            GCK::Stat(StatType::DMGMult(Condition::None)),
            GCK::AttributeDMGBonusMult,
            GCK::CategoryDMGBonusMult,
            GCK::TargetDMGBonusMult
        ];
        GCK::AttributeDMGBonusMult => mux[
            GCK::Attribute,
            GCK::Stat(StatType::DMGMult(Condition::Attribute(Attribute::Elemental(Element::Anemo)))),
            GCK::Stat(StatType::DMGMult(Condition::Attribute(Attribute::Elemental(Element::Anemo)))),
            GCK::Stat(StatType::DMGMult(Condition::Attribute(Attribute::Elemental(Element::Geo)))),
            GCK::Stat(StatType::DMGMult(Condition::Attribute(Attribute::Elemental(Element::Electro)))),
            GCK::Stat(StatType::DMGMult(Condition::Attribute(Attribute::Elemental(Element::Dendro)))),
            GCK::Stat(StatType::DMGMult(Condition::Attribute(Attribute::Elemental(Element::Hydro)))),
            GCK::Stat(StatType::DMGMult(Condition::Attribute(Attribute::Elemental(Element::Pyro)))),
            GCK::Stat(StatType::DMGMult(Condition::Attribute(Attribute::Elemental(Element::Cryo))))
        ];
        GCK::CategoryDMGBonusMult => mux[
            GCK::Category,
            GCK::Stat(StatType::DMGMult(Condition::Category(Category::NormalAttack))),
            GCK::Stat(StatType::DMGMult(Condition::Category(Category::ChargedAttack))),
            GCK::Stat(StatType::DMGMult(Condition::Category(Category::PlungeAttack))),
            GCK::Stat(StatType::DMGMult(Condition::Category(Category::ElementalSkill))),
            GCK::Stat(StatType::DMGMult(Condition::Category(Category::ElementalBurst)))
        ];

        // Evaluating TargetDEFMult
        GCK::TargetDEFMult => def_mult[
            GCK::Stat(StatType::Level),
            GCK::TargetLevel,
            GCK::TargetDEFReduct,
            GCK::TotalDEFIgnore
        ];
        // todo - TotalDEFIgnore

        // Evaluating TargetRESMult
        GCK::TargetRESMult => res_mult[
            GCK::TargetRESFinal
        ];
        GCK::TargetRESFinal => sum[
            GCK::SelTargetAttributeRES,
            GCK::SelTargetAttributeRESReductNeg
        ];
        GCK::SelTargetAttributeRES => mux[
            GCK::Attribute,
            GCK::TargetAttributeRES(Attribute::Physical),
            GCK::TargetAttributeRES(Attribute::Elemental(Element::Anemo)),
            GCK::TargetAttributeRES(Attribute::Elemental(Element::Geo)),
            GCK::TargetAttributeRES(Attribute::Elemental(Element::Electro)),
            GCK::TargetAttributeRES(Attribute::Elemental(Element::Dendro)),
            GCK::TargetAttributeRES(Attribute::Elemental(Element::Hydro)),
            GCK::TargetAttributeRES(Attribute::Elemental(Element::Pyro)),
            GCK::TargetAttributeRES(Attribute::Elemental(Element::Cryo))
        ];
        GCK::SelTargetAttributeRESReductNeg => neg[
            GCK::SelTargetAttributeRESReduct
        ];
        GCK::SelTargetAttributeRESReduct => mux[
            GCK::Attribute,
            GCK::TargetAttributeRESReduct(Attribute::Physical),
            GCK::TargetAttributeRESReduct(Attribute::Elemental(Element::Anemo)),
            GCK::TargetAttributeRESReduct(Attribute::Elemental(Element::Geo)),
            GCK::TargetAttributeRESReduct(Attribute::Elemental(Element::Electro)),
            GCK::TargetAttributeRESReduct(Attribute::Elemental(Element::Dendro)),
            GCK::TargetAttributeRESReduct(Attribute::Elemental(Element::Hydro)),
            GCK::TargetAttributeRESReduct(Attribute::Elemental(Element::Pyro)),
            GCK::TargetAttributeRESReduct(Attribute::Elemental(Element::Cryo))
        ];

        // Evaluating AmpRxnMult
        GCK::AmpRxnMult => mux_or_1[
            GCK::AmpRxnType,
            GCK::PotentialAmpRxnMult,
            GCK::PotentialAmpRxnMult
        ];
        GCK::PotentialAmpRxnMult => product[
            GCK::BaseAmpRxnMult,
            GCK::AmpRxnTotalBonusMult
        ];
        GCK::AmpRxnTotalBonusMult => sum_plus_one[
            GCK::AmpRxnEMMult,
            GCK::AmpRxnBonusMult
        ];
        GCK::AmpRxnEMMult => amp_rxn_em_mult[
            GCK::Stat(StatType::ElementalMastery)
        ];
        GCK::AmpRxnBonusMult => mux_or_0[
            GCK::AmpRxnType,
            GCK::Stat(StatType::RxnDMGMult(ElementalReaction::ForwardVaporize)),
            GCK::Stat(StatType::RxnDMGMult(ElementalReaction::ForwardMelt))
        ];

        // Evaluatee CritMult
        GCK::CritMult => crit_mult[
            GCK::TotalCritRate,
            GCK::TotalCritDMG
        ];
        GCK::TotalCritRate => sum[
            GCK::Stat(StatType::CritRate)
            // TODO - ADD Conditional Crit Stats
        ];
        GCK::TotalCritDMG => sum [
            GCK::Stat(StatType::CritDmg)
            // TODO - ADD Conditional Crit Stats
        ]
    )
});
