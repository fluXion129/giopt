use crate::character::{stats::Stats, talent::Talent};

mod base_dmg;
use base_dmg::IntoBaseDmg;

/// A struct that stores a configuration for calculating damage.
/// Will be edited as app is used to refer to different methods of aquiring
/// each component needed for the calculation
pub struct DamageCalculator<'a> {
    stats: &'a Stats,
    talent: &'a Talent,
    
}

impl<'a> DamageCalculator<'a> {
    fn new(stats: &'a Stats, talent: &'a Talent) -> Self {
        Self {
            stats,
            talent,
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
        self.talent.get_scalings().iter().fold(0.0, |acc, stat| 
            self.stats.get_stat(stat.typ()) * stat.val()
        )
    }

    fn base_dmg_mult(&self) -> f64 {
        1.0
    }

    fn base_dmg_flat(&self) -> f64 {
        0.0
    }

    fn rxn_effect(&self) -> ReactionEffect {
        ReactionEffect::None
    }

    fn dmg_mult(&self) -> f64 {
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

enum ReactionEffect {
    Additive(f64),
    Multiplicative(f64),
    None
}

    

#[cfg(test)]
mod tests {
    use super::*;
    use crate::character::{stats::{self, Stats, Stat}, talent::{self, Talent}};

    fn test_stats() -> Stats {
        Stats::from([
            (stats::Type::Hp, 10000.0),
            (stats::Type::Atk, 1000.0),
            (stats::Type::Def, 500.0)
        ])
    }

    fn simple_talent() -> Talent {
        Talent::new(talent::Type::NormalAttack(0), vec![Stat::new(stats::Type::Atk, 1.0)])
    }

    #[test]
    fn basic_calculate() {
        let stats = test_stats();
        let talent = simple_talent();
        let mut calculator = DamageCalculator::new(&stats, &talent);

        assert_eq!(calculator.calculate(), 1000.0);
    }

    #[test]
    fn calculate_with_amp_reaction() {
        let stats = test_stats();
        let talent = simple_talent();
        let mut calculator = DamageCalculator::new(&stats, &talent);

        assert_eq!(calculator.calculate(), 300.0);
    }

    #[test]
    fn calculate_with_amp_reaction_and_dmg_bonus() {
        let stats = test_stats();
        let talent = simple_talent();
        let mut calculator = DamageCalculator::new(&stats, &talent);

        assert_eq!(calculator.calculate(), 750.0);
    }
}
