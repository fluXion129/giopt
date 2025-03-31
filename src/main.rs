use std::collections::HashMap;

use giopt::{
    calculator::Calculator,
    damage::{Attribute, Category},
    element::{reaction::ElementalReaction::*, Element::*},
    stats::Type::*,
    B, GCK, GI_RULES, L, S,
};

fn main() {
    // Old Calculator Testing
    // // Arlecchino Melt testing
    // let stats = StatSheet::from([
    //     (Level, 90.0),
    //     (MaxHP, 20626.0),
    //     (Atk, 4514.2),
    //     (Def, 765.0),
    //     (ElementalMastery, 380.0),
    //     (CritRate, 0.772),
    //     (CritDmg, 1.918),
    //     (DMGMult(Some(Condition::Attribute(Elemental(Pyro)))), 1.416),
    //     (DMGMult(None), 0.18),
    // ]);
    // let c2burst = Talent::new(
    //     None,
    //     Some(Attribute::Elemental(Pyro)),
    //     &ICD::STANDARD,
    //     vec![Stat::new(Atk, 9.00)],
    // );

    // let enemy_stats = StatSheet::from([(Level, 103.0), (AttributeRES(Elemental(Pyro)), -0.5)]);
    // let enemy_aura = Some(GaugedAura::new(Aura::Dendro, 1.0, 0.0));
    // let dmg_on_crit = evaluate_damage_instance(
    //     &stats,
    //     &c2burst,
    //     Some(ElementalApplication::new(Pyro, 1.0)),
    //     &enemy_stats,
    //     enemy_aura.as_ref(),
    //     CritMode::OnCrit,
    // );

    // 1 million calculatons takes around 20 seconds
    // for _ in 0..1000000 {
    //     calculator.calculate();
    // }

    // println!("damage on crit: {dmg_on_crit}");

    // You have to create an instance of the GI_RULES so that the LazyLock gets evaluated.
    // Actually I'm not quite sure about this, the type is still LazyLock<Rules<GCK>>
    let rules = GI_RULES;

    // GI_RULES testing
    let mut calc = Calculator::from_components(
        HashMap::from([
            (GCK::One, 1.0),
            (GCK::L(L::TargetLevel), 103.0),
            (GCK::L(L::TargetAttributeRES(Pyro.into())), 0.1),
            (GCK::L(L::TargetAttributeRESReduct(Pyro.into())), -0.6),
            (GCK::L(L::Scaling(S::Atk)), 9.0),
            (GCK::L(L::Attribute), Attribute::from(Pyro).calcindex()),
            (GCK::L(L::Category), Category::NormalAttack.calcindex()),
            (GCK::L(L::BaseAmpRxnMult), 2.0),
            (GCK::L(L::AmpRxnType), ForwardMelt.amp_rxn_type_calcindex()),
            (Level.into(), 90.0),
            (MaxHP.into(), 20626.0),
            (Atk.into(), 4514.2),
            (Def.into(), 765.0),
            (ElementalMastery.into(), 380.0),
            (CritRate.into(), 0.772),
            (CritDmg.into(), 1.918),
            (DMGMult(None).into(), 0.18),
            (DMGMult(Some(Pyro.into())).into(), 1.416),
            (DMGMult(Some(Cryo.into())).into(), 0.40),
        ]),
        &rules,
    );

    macro_rules! calc_print {
        ($($n:expr),*) => {
            $(println!("{:?}: {}", $n, calc.get(&$n));)*
        };
    }

    calc_print!(
        GCK::B(B::DamageInstanceOutput),
        GCK::B(B::BaseDMGFinal),
        // GCK::B(B::BaseDMGPostMult),
        // GCK::B(B::BaseDMG),
        // GCK::B(B::BaseDMGMult),
        GCK::B(B::DMGBonusMult),
        GCK::B(B::TargetDEFMult),
        // GCK::L(L::Stat(Level)),
        // GCK::L(L::TargetLevel),
        // GCK::L(L::TargetDEFReduct),
        // GCK::B(B::TotalDEFIgnore),
        GCK::B(B::TargetRESMult),
        GCK::B(B::AmpRxnMult),
        // GCK::B(B::AmpRxnTotalBonusMult),
        GCK::B(B::CritMult)
    );
}
