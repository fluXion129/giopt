use giopt::{
    character::{
        stats::{Condition, Stat, StatSheet, Type::*},
        talent::Talent,
    },
    damage::Attribute::*,
    damage_calculator::{evaluate_damage_instance, CritMode},
    element::{Aura, Element::*, ElementalApplication, GaugedAura},
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
        Some(ElementalApplication::new(Pyro, 1.0)),
        vec![Stat::new(Atk, 9.00)],
    );

    let enemy_stats = StatSheet::from([(Level, 103.0), (ResMult(Elemental(Pyro)), -0.5)]);
    let enemy_aura = Some(GaugedAura::new(Aura::Dendro, 1.0, 0.0));

    // 1 million calculatons takes around 20 seconds
    // for _ in 0..1000000 {
    //     calculator.calculate();
    // }

    println!(
        "damage on crit: {}",
        evaluate_damage_instance(
            &stats,
            &c2burst,
            &enemy_stats,
            enemy_aura.as_ref(),
            CritMode::OnCrit
        )
    );
}
