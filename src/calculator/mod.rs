use std::{collections::HashMap, fmt::Debug, hash::Hash};

pub mod rules;
use rules::Rules;

/// This trait is really just a shorthand for some other traits. Namely,
/// CalcKeys should be types that are 'static, Clone, Eq, Hash, and Debug.
///
/// 'static can be understood to mean that the type does not contain references,
/// except perhaps 'static references, I'm not quite sure.
pub trait CalcKey: 'static + Clone + Eq + Hash + Debug {}

impl CalcKey for u32 {}
impl CalcKey for usize {}

pub struct Calculator<'a, K>
where
    K: CalcKey,
{
    values: HashMap<K, f32>,
    rules: &'a Rules<K>,
}

impl<'a, K: CalcKey> Calculator<'a, K> {
    pub fn from_components(values: HashMap<K, f32>, rules: &'a Rules<K>) -> Self {
        Self { values, rules }
    }

    pub fn new(rules: &'a Rules<K>) -> Self {
        Self {
            values: HashMap::new(),
            rules,
        }
    }

    /// Core method of the calculator. Currently implemented through recursion.
    /// Tries to get the value of the key given, both through direct access and calculation.
    ///
    /// Methods of calculation, in order:
    /// - Retrieve the value cached in the Calculator
    /// - Branch off and retrieve that value
    ///     - This recurses to get() the value of the branch
    /// - Calculate the value using the associated Rule
    ///     - This recurses to get() the values of the keys needed for the calculation
    /// - Default to 0.0
    ///
    /// After calling this function, the value computed will be cached.
    pub fn get(&mut self, key: &K) -> f32 {
        if let Some(val) = self.values.get(key) {
            return *val;
        }
        let val = self
            .rules
            .get(key)
            .map(|rule| (rule.op())(self, rule.keys()))
            .unwrap_or(0.0);
        self.values.insert(key.clone(), val);
        val
    }

    /// Sets the value in the calculator.
    ///
    /// If the value has changed, removes the values for the parents so
    /// that the effects of setting this value will be seen in upstream calculations.
    ///
    /// If there was no previous value, it will be assumed to be 0.0, and if the inserted
    /// value is 0.0, the parents will not be removed.
    ///
    ///
    /// QUESTION - should children also be removed?
    /// Leaving them in invites a certain amount of confusion, but removing them could
    /// be annoying.
    pub fn set(&mut self, key: K, val: f32) {
        if self.values.insert(key.clone(), val).unwrap_or(0.0) != val {
            self.remove_parents(key);
        }
    }

    /// Adds to a key. Will compute the value of that key before adding to it if it currently
    /// does not exist yet.
    pub fn add(&mut self, key: K, val: f32) {
        let base = self.get(&key);
        self.values.insert(key.clone(), base + val);
        self.remove_parents(key);
    }

    /// Removes the value in the calculator, and removes the values for the parents
    /// to trigger a recalculation of the upstream keys.
    pub fn remove(&mut self, key: K) {
        if self.values.remove(&key).unwrap_or(0.0) != 0.0 {
            self.remove_parents(key);
        }
    }

    /// Removes the parents of the key passed in until arriving at an unset value.
    /// Will not remove the key itself.
    ///
    /// Calling this effectively results in a recalculation now including this key.
    fn remove_parents(&mut self, mut key: K) -> Option<()> {
        println!("Removing parents of {key:?}:");
        loop {
            key = self.rules.get_parent(&key)?.clone();
            println!("   {key:?}");
            self.values.remove(&key)?;
        }
    }

    /// Sets the value in the calculator without removing parents. This will mean that if
    /// parents have already been calculated, their values will be used instead of recalculating
    /// from the value that you place using this method. If there was a previous value, it will be
    /// returned to you.
    pub fn place(&mut self, key: K, val: f32) -> Option<f32> {
        self.values.insert(key, val)
    }

    /// Delete the value in the calculator, without removing parents. This will mean that if parents
    /// have already been calculated, their values will be used instead of recalculating from the value
    /// that you deleted using this method. If there was a previous value, it will be returned to you.
    pub fn delete(&mut self, key: &K) -> Option<f32> {
        self.values.remove(key)
    }

    /// Debug prints the sheets current data
    pub fn print_sheet_state(&self) {
        println!("{:?}", self.values);
    }
}

#[cfg(test)]
mod tests;
