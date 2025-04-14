use std::collections::HashMap;

use giopt::{
    calculator::Calculator,
    damage::{Attribute, Category},
    element::{reaction::ElementalReaction::*, Element::*},
    stats::{Condition, StatSheet, Type::*},
    Scaling, GCK, GI_RULES,
};

fn main() {
    // Old Calculator Testing
    // // Arlecchino Melt testing
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
            (GCK::TargetLevel, 103.0),
            (GCK::TargetAttributeRES(Pyro.into()), 0.1),
            (GCK::TargetAttributeRESReduct(Pyro.into()), 0.6),
            (GCK::Scaling(Scaling::Atk), 9.0),
            (GCK::Attribute, Attribute::from(Pyro).calcindex()),
            (GCK::Category, Category::NormalAttack.calcindex()),
        ]),
        &rules,
    );

    let stats = StatSheet::from([
        (Level, 90.0),
        (MaxHP, 21390.0),
        (Atk, 4500.0),
        (Def, 764.71),
        (ElementalMastery, 333.16),
        (CritRate, 0.819),
        (CritDmg, 2.104),
        (DMGMult(Pyro.into()), 1.416),
        (DMGMult(Condition::None), 0.18),
    ]);
    // testing import_stat_sheet. I probably want to not use this method?
    calc.import_stat_sheet(&stats);
    calc.set_rxn(Some(ForwardMelt));

    calc.get(&GCK::DamageInstanceOutput);

    // calc.set(GCK::L(L::Scaling(S::EM)), 1.0);

    // calc.add(GCK::L(L::Stat(ElementalMastery)), 250.0);

    macro_rules! calc_print {
        ($($n:expr),*) => {
            $(println!("{:?}: {}", $n, calc.get(&$n));)*
        };
    }

    calc_print!(
        GCK::DamageInstanceOutput,
        GCK::BaseDMGFinal,
        // GCK::BaseDMGPostMult,
        // GCK::BaseDMG,
        // GCK::BaseDMGMult,
        GCK::DMGBonusMult,
        GCK::TargetDEFMult,
        // GCK::Stat(Level),
        // GCK::TargetLevel,
        // GCK::TargetDEFReduct,
        // GCK::TotalDEFIgnore,
        GCK::TargetRESMult,
        GCK::AmpRxnMult,
        // GCK::AmpRxnTotalBonusMult,
        GCK::CritMult
    );
}
