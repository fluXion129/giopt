use super::*;
use crate::{
    character::{
        stats::{Stat, Stats, Type as StatType},
        talent::Talent,
    },
    damage::Category as DamageCategory,
    element::{Aura, Element, ElementalApplication, GaugedAura},
};

fn simple_stats() -> Stats {
    Stats::from([
        (StatType::Level, 90.0),
        (StatType::Hp, 10000.0),
        (StatType::Atk, 1000.0),
        (StatType::Def, 500.0),
    ])
}

#[test]
fn basic_calculate() {
    let stats = simple_stats();
    let talent = Talent::new(
        DamageCategory::NormalAttack,
        None,
        vec![Stat::new(StatType::Atk, 1.0)],
    );
    let enemy_stats = Stats::from([(StatType::Level, 90.0)]);
    let calculator = DamageInstance::new(&stats, &talent, &enemy_stats, None);

    assert_eq!(calculator.calculate(), 1000.0);
}

#[test]
fn calculate_with_amp_reaction() {
    let stats = simple_stats();
    let talent = Talent::new(
        DamageCategory::NormalAttack,
        Some(ElementalApplication::new(Element::Pyro, 1.0)),
        vec![Stat::new(StatType::Atk, 1.0)],
    );
    let enemy_stats = Stats::from([(StatType::Level, 90.0)]);
    let calculator = DamageInstance::new(
        &stats,
        &talent,
        &enemy_stats,
        Some(GaugedAura::new(Aura::Hydro, 1.0, 0.0)),
    );

    assert_eq!(calculator.calculate(), 1500.0);
}

#[test]
fn calculate_with_amp_reaction_and_dmg_bonus() {
    let stats = Stats::from([
        (StatType::Hp, 10000.0),
        (StatType::Atk, 1000.0),
        (StatType::Def, 500.0),
        (StatType::DMGMult(None), 0.50),
    ]);
    let talent = Talent::new(
        DamageCategory::NormalAttack,
        Some(ElementalApplication::new(Element::Pyro, 1.0)),
        vec![Stat::new(StatType::Atk, 1.0)],
    );
    let enemy_stats = Stats::from([(StatType::Level, 90.0)]);
    let calculator = DamageInstance::new(
        &stats,
        &talent,
        &enemy_stats,
        Some(GaugedAura::new(Aura::Hydro, 1.0, 0.0)),
    );

    assert_eq!(calculator.calculate(), 2250.0);
}

#[test]
fn calculate_with_crit_averaged() {
    let stats = Stats::from([
        (StatType::Hp, 10000.0),
        (StatType::Atk, 1000.0),
        (StatType::Def, 500.0),
        (StatType::CritRate, 0.5),
        (StatType::CritDmg, 1.0),
    ]);
    let talent = Talent::new(
        DamageCategory::NormalAttack,
        None,
        vec![Stat::new(StatType::Atk, 1.0)],
    );
    let enemy_stats = Stats::from([(StatType::Level, 90.0)]);
    let calculator = DamageInstance::new(&stats, &talent, &enemy_stats, None);

    assert_eq!(calculator.calculate(), 1500.0);
}

#[test]
fn calculate_with_applicable_category_dmg_mult() {
    let stats = Stats::from([
        (StatType::Hp, 10000.0),
        (StatType::Atk, 1000.0),
        (StatType::Def, 500.0),
        (
            StatType::DMGMult(Some(Condition::Category(DamageCategory::NormalAttack))),
            0.5,
        ),
    ]);
    let talent = Talent::new(
        DamageCategory::NormalAttack,
        None,
        vec![Stat::new(StatType::Atk, 1.0)],
    );
    let enemy_stats = Stats::from([(StatType::Level, 90.0)]);
    let calculator = DamageInstance::new(&stats, &talent, &enemy_stats, None);

    assert_eq!(calculator.calculate(), 1500.0);
}

#[test]
fn calculate_with_irrelevant_category_dmg_mult() {
    let stats = Stats::from([
        (StatType::Hp, 10000.0),
        (StatType::Atk, 1000.0),
        (StatType::Def, 500.0),
        (
            StatType::DMGMult(Some(Condition::Category(DamageCategory::NormalAttack))),
            0.5,
        ),
    ]);
    let talent = Talent::new(
        DamageCategory::ChargedAttack,
        None,
        vec![Stat::new(StatType::Atk, 1.0)],
    );
    let enemy_stats = Stats::from([(StatType::Level, 90.0)]);
    let calculator = DamageInstance::new(&stats, &talent, &enemy_stats, None);

    assert_eq!(calculator.calculate(), 1000.0);
}

#[test]
fn calculate_with_reaction_dmg_mult_and_generic_dmg_bonus() {
    let stats = Stats::from([
        (StatType::Hp, 10000.0),
        (StatType::Atk, 1000.0),
        (StatType::Def, 500.0),
        (StatType::DMGMult(None), 0.5),
        (StatType::RxnDMGMult(ElementalReaction::ForwardMelt), 0.15),
    ]);
    let talent = Talent::new(
        DamageCategory::ChargedAttack,
        Some(ElementalApplication::new(Element::Pyro, 1.0)),
        vec![Stat::new(StatType::Atk, 1.0)],
    );
    let enemy_stats = Stats::from([(StatType::Level, 90.0)]);
    let calculator = DamageInstance::new(
        &stats,
        &talent,
        &enemy_stats,
        Some(GaugedAura::new(Aura::Cryo, 1.0, 0.0)),
    );

    assert_eq!(calculator.calculate(), 3450.0);
}
