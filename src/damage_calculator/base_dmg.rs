
/// Types that implement this trait can be used in a damage_calculator for the base_damage value
/// Because these calculations often are quite repetative, I recommend implementing caching for
/// anything significantly complicated.
pub trait IntoBaseDmg {
    fn to(&self) -> f64;
}

impl IntoBaseDmg for f64 {
    fn to(&self) -> f64 {
        *self
    }
}

use crate::character::{talent::Talent, stats::Stats};
pub struct FromTalentAndStats<'a> {
   talent: &'a Talent,
   stats: &'a Stats,
   cache: Option<f64>
}
impl<'a> FromTalentAndStats<'a> {
    pub fn new(talent: &'a Talent, stats: &'a Stats) -> Self {
        Self { talent, stats, cache: None }
    }

    pub fn set_talent(&mut self, talent: &'a Talent) {
        if self.talent != talent {
            self.talent = talent;
            self.cache = None;
        }
    }
    pub fn set_stats(&mut self, stats: &'a Stats) {
        if self.stats != stats {
            self.stats = stats;
            self.cache = None;
        }
    }
}
impl<'a> IntoBaseDmg for FromTalentAndStats<'a> {
    fn to(&self) -> f64 {
        if let Some(val) = self.cache { return val; }
        self.talent.get_scalings().iter()
            .map(|scaling| self.stats.get_stat(scaling.typ()) * scaling.val())
            .fold(0.0, |a, x| a + x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::character::{stats::{self, Stats, Stat}, talent::{self, Talent}};

    #[test]
    fn from_atk_scaling_talent_and_stats() {
        let test_talent = Talent::new(
            talent::Type::NormalAttack(1),
            vec![
                Stat::new(stats::Type::Atk, 0.50)
            ]
        );
        let test_stats = Stats::from([
            (stats::Type::Atk, 1000.0)
        ]);

        let base_dmg = FromTalentAndStats::new(&test_talent, &test_stats);

        assert_eq!(base_dmg.to(), 500.0);
    }

    #[test]
    fn from_atk_and_em_scaling_talent_and_stats() {
        let test_talent = Talent::new(
            talent::Type::NormalAttack(1),
            vec![
                Stat::new(stats::Type::Atk, 0.50),
                Stat::new(stats::Type::ElementalMastery, 3.00)
            ]
        );
        let test_stats = Stats::from([
            (stats::Type::Atk, 1000.0),
            (stats::Type::ElementalMastery, 400.0)
        ]);

        let base_dmg = FromTalentAndStats::new(&test_talent, &test_stats);

        assert_eq!(base_dmg.to(), 1700.0);
    }
}
