use std::{collections::HashMap, hash::Hash};

pub use rules::{Operation, Rule, Rules};
mod rules;

pub struct Calculator<'a, K>
where
    K: 'static + Clone + Eq + Hash,
{
    data: HashMap<K, f32>,
    rules: &'a Rules<K>,
}

impl<'a, K: Clone + Eq + Hash> Calculator<'a, K> {
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
        self.data.get(key).cloned().unwrap_or_else(|| {
            self.rules
                .get(key)
                .map(|rule| rule.eval(rule.keys().iter().map(|k| self.get(k))))
                .unwrap_or(0.0)
        })
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

    /// Sets the value in the calculator without removing parents. This will mean that if
    /// parents have already been calculated, their values will be used instead of recalculating
    /// from the value that you place using this method. If there was a previous value, it will be
    /// returned to you.
    pub fn place(&mut self, key: K, val: f32) -> Option<f32> {
        self.data.insert(key, val)
    }

    pub fn remove(&mut self, key: &K) -> Option<f32> {
        self.data.remove(key)
    }

    fn remove_parents(&mut self, mut key: K) -> Option<()> {
        loop {
            key = self.rules.get_parent(&key)?.clone();
            if self.data.remove(&key).is_some() {
                break Some(());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::calculator::{Rule, Rules};

    use super::{Calculator, Operation};

    #[test]
    fn basic_calculate() {
        let calcrules = Rules::new(HashMap::from([(2, Rule::new(vec![0, 1], Operation::Sum))]));
        let mut calc: Calculator<usize> = Calculator {
            data: HashMap::from([(0, 1.0), (1, 2.0)]),
            rules: &calcrules,
        };
        assert_eq!(calc.get(&0), 1.0);
        assert_eq!(calc.get(&1), 2.0);
        assert_eq!(calc.get(&2), 3.0);
    }
}
