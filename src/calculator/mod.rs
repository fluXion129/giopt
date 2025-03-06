use std::{collections::HashMap, fmt::Debug, hash::Hash};

pub use rules::{Operation, Rule, Rules};
mod rules;

pub struct Calculator<'a, K>
where
    K: 'static + Clone + Eq + Hash,
{
    data: HashMap<K, f32>,
    rules: &'a Rules<K>,
}

impl<'a, K: Clone + Eq + Hash + Debug> Calculator<'a, K> {
    pub fn new(data: HashMap<K, f32>, rules: &'a Rules<K>) -> Self {
        Self { data, rules }
    }

    /// Core method of the calculator. Currently implemented through recursion.
    /// Tries to get the value of the key given, both through direct access and calculation.
    ///
    /// Methods of calculation, in order:
    /// - Retrieve the value stored in the Calculator's data
    /// - Calculate the value using the associated Rule
    ///     - This recurses to get() the values of the keys needed for the calculation
    /// - Default to 0.0
    pub fn get(&mut self, key: &K) -> f32 {
        if let Some(val) = self.data.get(key) {
            return val.clone();
        }
        let val = self
            .rules
            .get(key)
            .map(|rule| rule.eval(rule.keys().iter().map(|k| self.get(k))))
            .unwrap_or(0.0);
        self.data.insert(key.clone(), val);
        val
    }

    /// Sets the value in the calculator, and removes the values for the parents so
    /// that the effects of setting this value will be seen in upstream calculations.
    ///
    /// QUESTION - should children also be removed?
    /// Leaving them in invites a certain amount of confusion, but removing them could
    /// be annoying.
    pub fn set(&mut self, key: K, val: f32) {
        if self.data.insert(key.clone(), val).is_some() {
            self.remove_parents(key);
        }
    }

    /// Removes the value in the calculator, and removes the values for the parents
    /// to trigger a recalculation of the upstream keys.
    pub fn remove(&mut self, key: &K) -> Option<f32> {
        self.remove_parents(key.clone());
        self.data.remove(key)
    }

    /// Removes the parents of the key passed in until arriving at an unset value.
    /// Will not remove the key itself.
    ///
    /// Calling this effectively results in a recalculation now including this key.
    fn remove_parents(&mut self, mut key: K) -> Option<()> {
        loop {
            key = self.rules.get_parent(&key)?.clone();
            self.data.remove(&key)?;
        }
    }

    /// Sets the value in the calculator without removing parents. This will mean that if
    /// parents have already been calculated, their values will be used instead of recalculating
    /// from the value that you place using this method. If there was a previous value, it will be
    /// returned to you.
    pub fn place(&mut self, key: K, val: f32) -> Option<f32> {
        self.data.insert(key, val)
    }

    /// Delete the value in the calculator, without removing parents. This will mean that if parents
    /// have already been calculated, their values will be used instead of recalculating from the value
    /// that you deleted using this method. If there was a previous value, it will be returned to you.
    pub fn delete(&mut self, key: &K) -> Option<f32> {
        self.data.remove(key)
    }

    /// Debug prints the sheets current data
    pub fn print_sheet_state(&self) {
        println!("{:?}", self.data);
    }
}

#[cfg(test)]
mod tests;
