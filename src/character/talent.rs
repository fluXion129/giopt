use std::collections::HashMap;
use std::ops::{Mul, MulAssign};
use std::time::Duration;

use crate::damage::{Attribute, Category};

use crate::stats::{Condition, Stat};

/// Determines the pattern of elemental applications
pub struct ICD {
    time: Duration,
    hits: usize,
}
impl ICD {
    pub const STANDARD: Self = Self {
        time: Duration::new(2, 500000000),
        hits: 3,
    };
}

#[derive(PartialEq, Eq, Hash)]
pub struct ID {
    typ: Option<Category>,
    num: u8,
}

/// A struct that store the talent data of a character.
///
/// TODO
///
/// After implemented -> Add character-specific talent sheets
pub struct TalentSheet {
    data: HashMap<ID, Talent>,
}
impl TalentSheet {
    /// Gets the talent you're looking for. This does not include the level multiplier
    pub fn get(&self, id: &ID) -> Option<&Talent> {
        self.data.get(id)
    }
}

#[derive(Clone)]
pub struct Talent {
    category: Option<Category>,
    attribute: Option<Attribute>,
    _icd: &'static ICD,
    scalings: Vec<Stat>,
}
impl Talent {
    pub fn new(
        category: Option<Category>,
        attribute: Option<Attribute>,
        _icd: &'static ICD,
        scalings: Vec<Stat>,
    ) -> Self {
        Self {
            category,
            attribute,
            _icd,
            scalings,
        }
    }

    pub fn category(&self) -> Option<Category> {
        self.category
    }
    pub fn get_scalings(&self) -> &[Stat] {
        &self.scalings
    }

    /// Creates a vector of all conditions that this talent meets
    pub fn conditions_met(&self) -> Vec<Option<Condition>> {
        [
            self.attribute.map(Condition::Attribute),
            self.category.map(Condition::Category),
        ]
        .into_iter()
        .filter(|x| x.is_some())
        .chain(None)
        .collect()
    }
}
impl Mul<f32> for &Talent {
    type Output = Talent;
    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            category: self.category,
            attribute: self.attribute,
            _icd: self._icd,
            scalings: self.scalings.iter().map(|x| x * rhs).collect(),
        }
    }
}
impl MulAssign<f32> for Talent {
    fn mul_assign(&mut self, rhs: f32) {
        for scaling in self.scalings.iter_mut() {
            *scaling *= rhs;
        }
    }
}
