use std::collections::HashMap;

use giopt::{
    calculator::Calculator,
    damage::{Attribute, Category},
    element::Element,
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

    // Pre-GI Rules New Calc Testing
    // #[derive(Clone, PartialEq, Eq, Hash, Debug)]
    // enum Keys {
    //     One,
    //     Attribute,
    //     DamageInstanceOutput,
    //     TotalAtk,
    //     DmgBMult,
    //     DmgB,
    //     AttributeDmgB,
    //     FireDmgB,
    //     IceDmgB,
    //     CritMult,
    //     CritAddMult,
    //     CritRate,
    //     TotalCritDmg,
    //     CritDmg,
    //     AttributeCritDmg,
    //     FireCritDmg,
    //     IceCritDmg,
    // }

    // // Calculator testing
    // let calcrules = Rules::new(HashMap::from([
    //     (
    //         Keys::DamageInstanceOutput,
    //         Rule::new(
    //             &product,
    //             vec![Keys::TotalAtk, Keys::DmgBMult, Keys::CritMult],
    //         ),
    //     ),
    //     (
    //         Keys::DmgBMult,
    //         Rule::new(&sum, vec![Keys::One, Keys::DmgB, Keys::AttributeDmgB]),
    //     ),
    //     (
    //         Keys::AttributeDmgB,
    //         Rule::new(&mux, vec![Keys::Attribute, Keys::FireDmgB, Keys::IceDmgB]),
    //     ),
    //     (
    //         Keys::CritMult,
    //         Rule::new(&sum, vec![Keys::One, Keys::CritAddMult]),
    //     ),
    //     (
    //         Keys::CritAddMult,
    //         Rule::new(&product, vec![Keys::CritRate, Keys::TotalCritDmg]),
    //     ),
    //     (
    //         Keys::TotalCritDmg,
    //         Rule::new(&sum, vec![Keys::CritDmg, Keys::AttributeCritDmg]),
    //     ),
    //     (
    //         Keys::AttributeCritDmg,
    //         Rule::new(
    //             &mux,
    //             vec![Keys::Attribute, Keys::FireCritDmg, Keys::IceCritDmg],
    //         ),
    //     ),
    // ]));
    // let mut calc = Calculator::from_components(
    //     HashMap::from([
    //         (Keys::One, 1.0),
    //         (Keys::Attribute, 2.0),
    //         (Keys::TotalAtk, 100.0),
    //         (Keys::DmgB, 0.5),
    //         (Keys::FireDmgB, 0.5),
    //         (Keys::CritRate, 0.5),
    //         (Keys::CritDmg, 1.0),
    //         (Keys::IceCritDmg, 1.0),
    //     ]),
    //     &calcrules,
    // );
    // calc.get(&Keys::DamageInstanceOutput);
    // calc.print_sheet_state();

    let rules = GI_RULES;
    // GI_RULES testing
    let mut calc = Calculator::from_components(
        HashMap::from([
            (GCK::One, 1.0),
            (GCK::L(L::Stat(Level)), 90.0),
            (GCK::L(L::TargetLevel), 90.0),
            (GCK::L(L::Scaling(S::Atk)), 1.0),
            (
                GCK::L(L::Attribute),
                Attribute::Elemental(Element::Pyro).calcindex(),
            ),
            (GCK::L(L::Category), Category::NormalAttack.calcindex()),
            (GCK::L(L::BaseAmpRxnMult), 1.0),
            (GCK::L(L::AmpRxnType), 0.0),

            (GCK::L(L::Stat(Atk)), 100.0),
            (GCK::L(L::Stat(MaxHP)), 1000.0),
            (GCK::L(L::Stat(DMGMult(None))), 0.5)
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

        GCK::B(B::TargetRESMult),

        GCK::B(B::AmpRxnMult)
    );

    // println!("3: {}", calc.get(&3));
    // println!("5: {}", calc.get(&5));
    // println!("6: {}", calc.get(&6));
}
