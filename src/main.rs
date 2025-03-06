use std::collections::HashMap;

use giopt::{
    calculator::{Calculator, Operation, Rule, Rules},
    character::talent::{Talent, ICD},
    damage::Attribute::{self, *},
    damage_calculator::{evaluate_damage_instance, CritMode},
    element::{Aura, Element::*, ElementalApplication, GaugedAura},
    stats::{Condition, Stat, StatSheet, Type::*},
};

fn main() {
    // Arlecchino Melt testing
    let stats = StatSheet::from([
        (Level, 90.0),
        (Hp, 20626.0),
        (Atk, 4514.2),
        (Def, 765.0),
        (ElementalMastery, 380.0),
        (CritRate, 0.772),
        (CritDmg, 1.918),
        (DMGMult(Some(Condition::Attribute(Elemental(Pyro)))), 1.416),
        (DMGMult(None), 0.18),
    ]);
    let c2burst = Talent::new(
        None,
        Some(Attribute::Elemental(Pyro)),
        &ICD::STANDARD,
        vec![Stat::new(Atk, 9.00)],
    );

    let enemy_stats = StatSheet::from([(Level, 103.0), (ResMult(Elemental(Pyro)), -0.5)]);
    let enemy_aura = Some(GaugedAura::new(Aura::Dendro, 1.0, 0.0));
    let dmg_on_crit = evaluate_damage_instance(
        &stats,
        &c2burst,
        Some(ElementalApplication::new(Pyro, 1.0)),
        &enemy_stats,
        enemy_aura.as_ref(),
        CritMode::OnCrit,
    );

    // 1 million calculatons takes around 20 seconds
    // for _ in 0..1000000 {
    //     calculator.calculate();
    // }

    // println!("damage on crit: {dmg_on_crit}");

    // Calculator testing
    let calcrules = Rules::new(HashMap::from([
        (3, Rule::new(vec![0, 1, 2], Operation::Sum)),
        (5, Rule::new(vec![3, 4], Operation::Prod)),
    ]));
    let mut calc = Calculator::new(
        HashMap::from([(0, 1.0), (1, 4.0), (2, 5.0), (4, 2.0)]),
        &calcrules,
    );
    calc.print_sheet_state();
    println!("{:?}\n", calc.get(&5));

    calc.set(4, 9.0);
    calc.print_sheet_state();
    println!("{:?}\n", calc.get(&5));

    calc.set(3, 2.0);
    calc.print_sheet_state();
    println!("{:?}\n", calc.get(&5));

    calc.remove(&3);
    calc.print_sheet_state();
    println!("{:?}\n", calc.get(&5));

    calc.print_sheet_state();
    calc.remove(&1);
    calc.print_sheet_state();
    println!("{:?}\n", calc.get(&5));

    println!("{:?}", calc.get(&6));

    calc.print_sheet_state();
}
