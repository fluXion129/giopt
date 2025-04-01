use std::{collections::HashMap, sync::LazyLock};

use crate::{
    calculator::{
        rules::{mux, mux0, mux1, neg, product, sum, sum_plus_one, Rule, Rules},
        Calculator,
    },
    damage::{Attribute, Category},
    element::{reaction::ElementalReaction, Element},
    stats::{Condition, Type as StatType},
};

use super::{B, GCK, L, S};

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
        GCK::B(B::DamageInstanceOutput) => product[
            GCK::B(B::BaseDMGFinal),
            GCK::B(B::DMGBonusMult),
            GCK::B(B::TargetDEFMult),
            GCK::B(B::TargetRESMult),
            GCK::B(B::AmpRxnMult),
            GCK::B(B::CritMult)
        ];

        // Evaluating BaseDMGFinal
        GCK::B(B::BaseDMGFinal) => sum[
            GCK::B(B::BaseDMGPostMult),
            GCK::L(L::BaseDMGAdd)
        ];
        GCK::B(B::BaseDMGPostMult) => product[
            GCK::B(B::BaseDMG),
            GCK::B(B::BaseDMGMult)
        ];
        // TODO - ADD Conditional BaseDMG modifiers
        GCK::B(B::BaseDMGMult) => sum_plus_one[
            GCK::L(L::Stat(StatType::BaseDMGMult(None)))
        ];
        GCK::B(B::BaseDMG) => sum[
            GCK::B(B::EvalScaling(S::Atk)),
            GCK::B(B::EvalScaling(S::MaxHP)),
            GCK::B(B::EvalScaling(S::Def)),
            GCK::B(B::EvalScaling(S::EM))
        ];
        GCK::B(B::EvalScaling(S::Atk)) => product[
            GCK::L(L::Scaling(S::Atk)),
            GCK::L(L::Stat(StatType::Atk))
        ];
        GCK::B(B::EvalScaling(S::MaxHP)) => product[
            GCK::L(L::Scaling(S::MaxHP)),
            GCK::L(L::Stat(StatType::MaxHP))
        ];
        GCK::B(B::EvalScaling(S::Def)) => product[
            GCK::L(L::Scaling(S::Def)),
            GCK::L(L::Stat(StatType::Def))
        ];
        GCK::B(B::EvalScaling(S::EM)) => product[
            GCK::L(L::Scaling(S::EM)),
            GCK::L(L::Stat(StatType::ElementalMastery))
        ];

        // Evaluating DMGBonusMult
        GCK::B(B::DMGBonusMult) => sum_plus_one[
            GCK::L(L::Stat(StatType::DMGMult(None))),
            GCK::B(B::AttributeDMGBonusMult),
            GCK::B(B::CategoryDMGBonusMult),
            GCK::L(L::TargetDMGBonusMult)
        ];
        GCK::B(B::AttributeDMGBonusMult) => mux[
            GCK::L(L::Attribute),
            GCK::L(L::Stat(StatType::DMGMult(Some(Condition::Attribute(Attribute::Elemental(Element::Anemo)))))),
            GCK::L(L::Stat(StatType::DMGMult(Some(Condition::Attribute(Attribute::Elemental(Element::Anemo)))))),
            GCK::L(L::Stat(StatType::DMGMult(Some(Condition::Attribute(Attribute::Elemental(Element::Geo)))))),
            GCK::L(L::Stat(StatType::DMGMult(Some(Condition::Attribute(Attribute::Elemental(Element::Electro)))))),
            GCK::L(L::Stat(StatType::DMGMult(Some(Condition::Attribute(Attribute::Elemental(Element::Dendro)))))),
            GCK::L(L::Stat(StatType::DMGMult(Some(Condition::Attribute(Attribute::Elemental(Element::Hydro)))))),
            GCK::L(L::Stat(StatType::DMGMult(Some(Condition::Attribute(Attribute::Elemental(Element::Pyro)))))),
            GCK::L(L::Stat(StatType::DMGMult(Some(Condition::Attribute(Attribute::Elemental(Element::Cryo))))))
        ];
        GCK::B(B::CategoryDMGBonusMult) => mux[
            GCK::L(L::Category),
            GCK::L(L::Stat(StatType::DMGMult(Some(Condition::Category(Category::NormalAttack))))),
            GCK::L(L::Stat(StatType::DMGMult(Some(Condition::Category(Category::ChargedAttack))))),
            GCK::L(L::Stat(StatType::DMGMult(Some(Condition::Category(Category::PlungeAttack))))),
            GCK::L(L::Stat(StatType::DMGMult(Some(Condition::Category(Category::ElementalSkill))))),
            GCK::L(L::Stat(StatType::DMGMult(Some(Condition::Category(Category::ElementalBurst)))))
        ];

        // Evaluating TargetDEFMult
        GCK::B(B::TargetDEFMult) => def_mult[
            GCK::L(L::Stat(StatType::Level)),
            GCK::L(L::TargetLevel),
            GCK::L(L::TargetDEFReduct),
            GCK::B(B::TotalDEFIgnore)
        ];
        // todo - TotalDEFIgnore

        // Evaluating TargetRESMult
        GCK::B(B::TargetRESMult) => res_mult[
            GCK::B(B::TargetRESFinal)
        ];
        GCK::B(B::TargetRESFinal) => sum[
            GCK::B(B::TargetAttributeRES),
            GCK::B(B::TargetAttributeRESReductNeg)
        ];
        GCK::B(B::TargetAttributeRES) => mux[
            GCK::L(L::Attribute),
            GCK::L(L::TargetAttributeRES(Attribute::Physical)),
            GCK::L(L::TargetAttributeRES(Attribute::Elemental(Element::Anemo))),
            GCK::L(L::TargetAttributeRES(Attribute::Elemental(Element::Geo))),
            GCK::L(L::TargetAttributeRES(Attribute::Elemental(Element::Electro))),
            GCK::L(L::TargetAttributeRES(Attribute::Elemental(Element::Dendro))),
            GCK::L(L::TargetAttributeRES(Attribute::Elemental(Element::Hydro))),
            GCK::L(L::TargetAttributeRES(Attribute::Elemental(Element::Pyro))),
            GCK::L(L::TargetAttributeRES(Attribute::Elemental(Element::Cryo)))
        ];
        GCK::B(B::TargetAttributeRESReductNeg) => neg[
            GCK::B(B::TargetAttributeRESReduct)
        ];
        GCK::B(B::TargetAttributeRESReduct) => mux[
            GCK::L(L::Attribute),
            GCK::L(L::TargetAttributeRESReduct(Attribute::Physical)),
            GCK::L(L::TargetAttributeRESReduct(Attribute::Elemental(Element::Anemo))),
            GCK::L(L::TargetAttributeRESReduct(Attribute::Elemental(Element::Geo))),
            GCK::L(L::TargetAttributeRESReduct(Attribute::Elemental(Element::Electro))),
            GCK::L(L::TargetAttributeRESReduct(Attribute::Elemental(Element::Dendro))),
            GCK::L(L::TargetAttributeRESReduct(Attribute::Elemental(Element::Hydro))),
            GCK::L(L::TargetAttributeRESReduct(Attribute::Elemental(Element::Pyro))),
            GCK::L(L::TargetAttributeRESReduct(Attribute::Elemental(Element::Cryo)))
        ];

        // Evaluating AmpRxnMult
        GCK::B(B::AmpRxnMult) => mux1[
            GCK::L(L::AmpRxnType),
            GCK::B(B::PotentialAmpRxnMult),
            GCK::B(B::PotentialAmpRxnMult)
        ];

        GCK::B(B::PotentialAmpRxnMult) => product[
            GCK::L(L::BaseAmpRxnMult),
            GCK::B(B::AmpRxnTotalBonusMult)
        ];
        GCK::B(B::AmpRxnTotalBonusMult) => sum_plus_one[
            GCK::B(B::AmpRxnEMMult),
            GCK::B(B::AmpRxnBonusMult)
        ];
        GCK::B(B::AmpRxnEMMult) => amp_rxn_em_mult[
            GCK::L(L::Stat(StatType::ElementalMastery))
        ];
        GCK::B(B::AmpRxnBonusMult) => mux0[
            GCK::L(L::AmpRxnType),
            GCK::L(L::Stat(StatType::RxnDMGMult(ElementalReaction::ForwardVaporize))),
            GCK::L(L::Stat(StatType::RxnDMGMult(ElementalReaction::ForwardMelt)))
        ];

        // Evaluate CritMult
        GCK::B(B::CritMult) => crit_mult[
            GCK::B(B::TotalCritRate),
            GCK::B(B::TotalCritDMG)
        ];
        GCK::B(B::TotalCritRate) => sum[
            GCK::L(L::Stat(StatType::CritRate))
            // TODO - ADD Conditional Crit Stats
        ];
        GCK::B(B::TotalCritDMG) => sum [
            GCK::L(L::Stat(StatType::CritDmg))
            // TODO - ADD Conditional Crit Stats
        ]
    )
});
