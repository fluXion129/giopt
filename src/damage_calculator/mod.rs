use crate::{
    character::{stats::Stats, talent::Talent},
    element::{reaction::ElementalReaction, Element},
    Aura, AuraType,
};

enum ReactionEffect {
    Additive(f64),
    Multiplicative(f64),
    None,
}

#[derive(Clone, Debug, Default)]
struct EnemyConfig {
    aura: Option<Aura>,
    level: usize,
}
impl EnemyConfig {
    fn new(aura: Aura, level: usize) -> Self {
        Self {
            aura: Some(aura),
            level,
        }
    }
    fn aura_type(&self) -> Option<AuraType> {
        self.aura.as_ref().map(|a| a.typ())
    }
}
/// A struct that stores a configuration for calculating damage.
/// Will be edited as app is used to refer to different methods of aquiring
/// each component needed for the calculation
pub struct DamageCalculator<'a> {
    stats: &'a Stats,
    talent: &'a Talent,
    enemy_config: &'a EnemyConfig,
}

impl<'a> DamageCalculator<'a> {
    fn new(stats: &'a Stats, talent: &'a Talent, enemy_config: &'a EnemyConfig) -> Self {
        Self {
            stats,
            talent,
            enemy_config,
        }
    }

    // caching requires mut, would prefer if was not mut
    // in order to multithread with caching, I will need to work around this
    fn calculate(&mut self) -> f64 {
        let mut result = self.base_dmg();
        result *= self.base_dmg_mult();
        result += self.base_dmg_flat();
        match self.rxn_effect() {
            ReactionEffect::Additive(val) => result += val,
            ReactionEffect::Multiplicative(val) => result *= val,
            ReactionEffect::None => (),
        }
        result *= self.dmg_mult();
        result *= self.def_mult();
        result *= self.res_mult();
        result *= self.crit_mult();
        result
    }

    fn base_dmg(&self) -> f64 {
        self.talent.get_scalings().iter().fold(0.0, |acc, stat| {
            self.stats.get_stat(stat.typ()) * stat.val()
        })
    }

    fn base_dmg_mult(&self) -> f64 {
        1.0
    }

    fn base_dmg_flat(&self) -> f64 {
        0.0
    }

    fn rxn_effect(&self) -> ReactionEffect {
        let Some(aura) = self.enemy_config.aura_type() else {
            return ReactionEffect::None;
        };
        let Some(application) = self.talent.application() else {
            return ReactionEffect::None;
        };
        let Some(reaction) = ElementalReaction::from_elements(aura, application.element()) else {
            return ReactionEffect::None;
        };

        match reaction {
            ElementalReaction::ForwardVaporize | ElementalReaction::ForwardMelt => {
                ReactionEffect::Multiplicative(2.0)
            }
            ElementalReaction::ReverseVaporize | ElementalReaction::ReverseMelt => {
                ReactionEffect::Multiplicative(1.5)
            }
            ElementalReaction::Quicken => ReactionEffect::Additive(todo!()),

            _ => ReactionEffect::None,
        }
    }

    fn dmg_mult(&self) -> f64 {
        // TODO
        1.0
    }

    fn def_mult(&self) -> f64 {
        1.0
    }

    fn res_mult(&self) -> f64 {
        1.0
    }

    fn crit_mult(&self) -> f64 {
        1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        character::{
            stats::{self, Stat, Stats},
            talent::{self, Talent},
        },
        element::ElementalApplication,
    };

    fn simple_stats() -> Stats {
        Stats::from([
            (stats::Type::Hp, 10000.0),
            (stats::Type::Atk, 1000.0),
            (stats::Type::Def, 500.0),
        ])
    }

    #[test]
    fn basic_calculate() {
        let stats = simple_stats();
        let talent = Talent::new(
            talent::Type::NormalAttack(0),
            None,
            vec![Stat::new(stats::Type::Atk, 1.0)],
        );
        let enemy_config = EnemyConfig::default();
        let mut calculator = DamageCalculator::new(&stats, &talent, &enemy_config);

        assert_eq!(calculator.calculate(), 1000.0);
    }

    #[test]
    fn calculate_with_amp_reaction() {
        let stats = simple_stats();
        let talent = Talent::new(
            talent::Type::NormalAttack(1),
            Some(ElementalApplication::new(
                crate::element::Element::Pyro,
                1.0,
            )),
            vec![Stat::new(stats::Type::Atk, 1.0)],
        );
        let enemy_config = EnemyConfig::new(
            Aura {
                typ: crate::AuraType::Hydro,
                gauge: 1.0,
                gauge_decay_rate: 0.0,
            },
            90,
        );
        let mut calculator = DamageCalculator::new(&stats, &talent, &enemy_config);

        assert_eq!(calculator.calculate(), 1500.0);
    }

    #[test]
    fn calculate_with_amp_reaction_and_dmg_bonus() {
        let stats = Stats::from([
            (stats::Type::Hp, 10000.0),
            (stats::Type::Atk, 1000.0),
            (stats::Type::Def, 500.0),
            (stats::Type::DMGBonus(stats::DMGBonusType::Universal), 0.50),
        ]);
        let talent = Talent::new(
            talent::Type::NormalAttack(0),
            Some(ElementalApplication::new(
                crate::element::Element::Pyro,
                1.0,
            )),
            vec![Stat::new(stats::Type::Atk, 1.0)],
        );
        let enemy_config = EnemyConfig::new(
            Aura {
                typ: crate::AuraType::Hydro,
                gauge: 1.0,
                gauge_decay_rate: 0.0,
            },
            90,
        );
        let mut calculator = DamageCalculator::new(&stats, &talent, &enemy_config);

        assert_eq!(calculator.calculate(), 2250.0);
    }
}
