use crate::{
    character::talent::Talent,
    element::{
        reaction::ElementalReaction::{self, *},
        Aura, ElementalApplication, GaugedAura,
    },
    stats::{StatSheet, Type::*},
};

#[derive(Debug)]
pub enum ReactionEffect {
    Additive(f64),
    Multiplicative(f64),
}
impl ReactionEffect {
    pub fn map(&self, f: impl Fn(f64) -> f64) -> Self {
        match *self {
            Self::Additive(x) => Self::Additive(f(x)),
            Self::Multiplicative(x) => Self::Multiplicative(f(x)),
        }
    }
}

pub enum CritMode {
    NonCrit,
    AvgCrit,
    OnCrit,
}

pub fn evaluate_damage_instance(
    stats: &StatSheet,
    talent: &Talent,
    target_stats: &StatSheet,
    target_aura: Option<&GaugedAura>,
    crit_mode: CritMode,
) -> f64 {
    let conds = talent.conditions_met();
    let mut result = base_dmg(stats, talent)
        * stats.sum_mults(conds.iter().map(|&x| BaseDMGMult(x)))
        + stats.sum_mults(conds.iter().map(|&x| BaseDMGFlat(x)));
    match rxn_effect(target_aura.map(|x| x.typ()), talent.elem_app(), stats) {
        Some(ReactionEffect::Additive(val)) => result += val,
        Some(ReactionEffect::Multiplicative(val)) => result *= val,
        None => (),
    }
    result
        * stats.sum_mults(conds.iter().map(|&x| DMGMult(x)))
        * def_mult(stats.get(Level), target_stats.get(Level))
        * res_mult(target_stats.get(ResMult(talent.attribute())))
        * crit_mult(stats.get(CritRate), stats.get(CritDmg), crit_mode)
}

/// Calculates the base damage given the stats and talent
pub fn base_dmg(stats: &StatSheet, talent: &Talent) -> f64 {
    let result = talent
        .get_scalings()
        .iter()
        .map(|s| stats.get(s.typ()) * s.val())
        .sum();
    println!("base dmg: {result}");
    result
}

/// Calculates the defense multiplier of the enemy
pub fn def_mult(char_lvl: f64, enemy_lvl: f64) -> f64 {
    let result = (char_lvl + 100.0) / (char_lvl + enemy_lvl + 200.0);
    println!("def mult: {result}");
    result
}

/// Calculates the resistance multiplier of the target
pub fn res_mult(target_res: f64) -> f64 {
    let result = if target_res < 0.0 {
        1.0 - target_res / 2.0
    } else if target_res < 0.75 {
        1.0 - target_res
    } else {
        1.0 / (4.0 * target_res + 1.0)
    };
    println!("res mult: {result}");
    result
}

/// Calculates the crit multiplier
pub fn crit_mult(cr: f64, cd: f64, mode: CritMode) -> f64 {
    let result = match mode {
        CritMode::NonCrit => 1.0,
        CritMode::AvgCrit => 1.0 + cr * cd,
        CritMode::OnCrit => 1.0 + cd,
    };
    println!("crit mult: {result}");
    result
}

/// Returns the reaction effect for the damage instance
pub fn rxn_effect(
    target_aura: Option<Aura>,
    elem_app: Option<&ElementalApplication>,
    stats: &StatSheet,
) -> Option<ReactionEffect> {
    let rxn = ElementalReaction::from_elements(target_aura?, elem_app?.element())?;
    let result = match rxn {
        ForwardVaporize | ForwardMelt => Some(ReactionEffect::Multiplicative(2.0)),
        ReverseVaporize | ReverseMelt => Some(ReactionEffect::Multiplicative(1.5)),
        Aggravate => Some(ReactionEffect::Additive(
            1.15 * level_multiplier(stats.get(Level)),
        )),
        Spread => Some(ReactionEffect::Additive(
            1.25 * level_multiplier(stats.get(Level)),
        )),

        _ => None,
    }
    .map(|re| {
        re.map(|x| {
            x * (1.0 + rxn_em_mult(rxn, stats.get(ElementalMastery)) + stats.get(RxnDMGMult(rxn)))
        })
    });
    println!("reaction effect: {result:?}");
    result
}

pub fn rxn_em_mult(rxn: ElementalReaction, em: f64) -> f64 {
    match rxn {
        ForwardMelt | ForwardVaporize | ReverseMelt | ReverseVaporize => amp_em_mult(em),
        Aggravate | Spread => add_em_mult(em),
        Crystallize => cry_em_dmg_absorb(em),
        _ => trans_em_mult(em),
    }
}

pub fn amp_em_mult(em: f64) -> f64 {
    let result = 2.778 * em / (em + 1400.0);
    println!("amplifying em mult: {result}");
    result
}

pub fn trans_em_mult(em: f64) -> f64 {
    let result = 16.0 * em / (em + 2000.0);
    println!("transformative em mult: {result}");
    result
}

pub fn add_em_mult(em: f64) -> f64 {
    let result = 5.0 * em / (em + 1200.0);
    println!("additive em mult: {result}");
    result
}

pub fn cry_em_dmg_absorb(em: f64) -> f64 {
    let result = 4.44 * em / (em + 1400.0);
    println!("crystallize em mult: {result}");
    result
}

pub fn level_multiplier(_level: f64) -> f64 {
    todo!("level multipliers")
}
