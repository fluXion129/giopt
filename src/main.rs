use giopt::{
    character::{
        stats::{Condition, Stat, Stats, Type::*},
        talent::Talent,
    },
    damage::{
        Attribute::{self, *},
        Category as DamageCategory,
    },
    damage_calculator::DamageInstance,
    element::{Aura, Element::*, ElementalApplication, GaugedAura},
};

fn main() {
    let stats = Stats::from([
        (Hp, 10000.0),
        (Atk, 3000.0),
        (Def, 500.0),
        (ElementalMastery, 395.0),
        (CritRate, 0.77),
        (CritDmg, 2.03),
        (DMGMult(Some(Condition::Attribute(Elemental(Pyro)))), 1.266),
    ]);
    let talent = Talent::new(
        DamageCategory::NormalAttack,
        Some(ElementalApplication::new(Pyro, 1.0)),
        vec![Stat::new(Atk, 5.40)],
    );

    let enemy_stats = Stats::from([(Level, 90.0), (ResMult(Attribute::Elemental(Pyro)), 0.1)]);
    let calculator = DamageInstance::new(
        &stats,
        &talent,
        &enemy_stats,
        Some(GaugedAura::new(Aura::Cryo, 1.0, 0.0)),
    );

    // 1 million calculatons takes around 20 seconds
    // for _ in 0..1000000 {
    //     calculator.calculate();
    // }

    println!("calculation: {}", calculator.calculate());
}
