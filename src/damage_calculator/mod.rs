use crate::{
    character::{
        stats::{Condition, Stats, Type::*},
        talent::Talent,
    },
    element::{
        reaction::ElementalReaction::{self, *},
        Aura, GaugedAura,
    },
};

enum ReactionEffect {
    Additive(f64),
    Multiplicative(f64),
}
impl ReactionEffect {
    /// Multiplies the effect of the reaction, preserving its type
    fn mult_internal(&self, mult: f64) -> Self {
        match *self {
            Self::Additive(x) => Self::Additive(x * mult),
            Self::Multiplicative(x) => Self::Multiplicative(x * mult),
        }
    }
}

/// Status of the enemy that the damage is being calculated for.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct EnemyConfig {
    aura: Option<GaugedAura>,
    level: usize,
}
impl EnemyConfig {
    pub fn new(aura: GaugedAura, level: usize) -> Self {
        Self {
            aura: Some(aura),
            level,
        }
    }
    pub fn aura_type(&self) -> Option<Aura> {
        self.aura.as_ref().map(|a| a.typ())
    }
}
/// A struct that stores all information needed to calculate a damage instance.
///
/// The calculate() method is used to evaluate the damage instance. The helper functions
/// for calculate() are also available to inspect components of the calculation.
pub struct DamageInstance<'a> {
    stats: &'a Stats,
    talent: &'a Talent,
    target_stats: &'a Stats,
    target_aura: Option<GaugedAura>,
}

impl<'a> DamageInstance<'a> {
    pub fn new(
        stats: &'a Stats,
        talent: &'a Talent,
        target_stats: &'a Stats,
        target_aura: Option<GaugedAura>,
    ) -> Self {
        Self {
            stats,
            talent,
            target_stats,
            target_aura,
        }
    }

    pub fn calculate(&self) -> f64 {
        let mut result = self.base_dmg() * self.base_dmg_mult() + self.base_dmg_flat();
        match self.rxn_effect() {
            Some(ReactionEffect::Additive(val)) => result += val,
            Some(ReactionEffect::Multiplicative(val)) => result *= val,
            None => (),
        }
        result * self.dmg_mult() * self.def_mult() * self.res_mult() * self.crit_mult()
    }

    fn conditions_met(&self) -> [Option<Condition>; 3] {
        [
            None,
            Some(Condition::Category(self.talent.category())),
            Some(Condition::Attribute(
                self.talent.elem_app().map(|app| app.element()).into(),
            )),
        ]
    }

    fn base_dmg(&self) -> f64 {
        self.talent
            .get_scalings()
            .iter()
            .map(|s| self.stats.get_stat(s.typ()) * s.val())
            .sum()
    }

    fn base_dmg_mult(&self) -> f64 {
        self.conditions_met()
            .into_iter()
            .fold(1.0, |a, cond| a + self.stats.get_stat(BaseDMGMult(cond)))
    }

    fn base_dmg_flat(&self) -> f64 {
        self.conditions_met()
            .into_iter()
            .fold(1.0, |a, cond| a + self.stats.get_stat(BaseDMGFlat(cond)))
    }

    /// Returns the reaction effect for the damage instance
    fn rxn_effect(&self) -> Option<ReactionEffect> {
        let rxn = ElementalReaction::from_elements(
            self.target_aura.as_ref()?.typ(),
            self.talent.elem_app()?.element(),
        )?;
        match rxn {
            ForwardVaporize | ForwardMelt => Some(ReactionEffect::Multiplicative(2.0)),
            ReverseVaporize | ReverseMelt => Some(ReactionEffect::Multiplicative(1.5)),
            Aggravate => Some(ReactionEffect::Additive(
                1.15 * level_multiplier(self.stats.get_stat(Level)),
            )),
            Spread => Some(ReactionEffect::Additive(
                1.25 * level_multiplier(self.stats.get_stat(Level)),
            )),

            _ => None,
        }
        .map(|re| {
            re.mult_internal(
                1.0 + rxn_em_mult(rxn, self.stats.get_stat(ElementalMastery))
                    + self.stats.get_stat(RxnDMGMult(rxn)),
            )
        })
    }

    fn dmg_mult(&self) -> f64 {
        self.conditions_met().into_iter().fold(1.0, |a, condition| {
            a + self.stats.get_stat(DMGMult(condition))
        })
    }

    fn def_mult(&self) -> f64 {
        (self.stats.get_stat(Level) + 100.0)
            / (self.stats.get_stat(Level) + self.target_stats.get_stat(Level) + 200.0)
    }

    fn res_mult(&self) -> f64 {
        // This requires the enemy to have a proper stat list to grab resistances from
        1.0 - self
            .target_stats
            .get_stat(ResMult(self.talent.elem_app().map(|x| x.element()).into()))
    }

    fn crit_mult(&self) -> f64 {
        // TODO - various "crit modes"
        1.0 + self.stats.get_stat(CritRate) * self.stats.get_stat(CritDmg)
    }
}

/// Calculate the em multiplier for a type of reaction with a certain amount of EM
pub fn rxn_em_mult(rxn: ElementalReaction, em: f64) -> f64 {
    match rxn {
        ForwardMelt | ForwardVaporize | ReverseMelt | ReverseVaporize => amp_em_mult(em),
        Aggravate | Spread => add_em_mult(em),
        Crystallize => cry_em_dmg_absorb(em),
        _ => trans_em_mult(em),
    }
}

pub fn amp_em_mult(em: f64) -> f64 {
    2.78 * em / (em + 1400.0)
}

pub fn trans_em_mult(em: f64) -> f64 {
    16.0 * em / (em + 2000.0)
}

pub fn add_em_mult(em: f64) -> f64 {
    5.0 * em / (em + 1200.0)
}

pub fn cry_em_dmg_absorb(em: f64) -> f64 {
    4.44 * em / (em + 1400.0)
}

// todo
pub fn level_multiplier(_level: f64) -> f64 {
    1.0
}

#[cfg(test)]
mod tests;
