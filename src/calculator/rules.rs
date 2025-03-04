use std::{collections::HashMap, hash::Hash};

/// A tree of mappings between tags that describes the mathematical relations between them.
///
/// For instance:
/// - 0: Sum\[1, 2]
/// - 2: Prod\[3, 4]
/// Would mean that tag 0 is equal to the sum of tags 1 and 2,
/// and tag 2 is equal to the product of tags 3 and 4
pub struct Rules<K: Clone + Eq + Hash> {
    rules: HashMap<K, Rule<K>>,
    parents: HashMap<K, K>,
}
impl<K: Clone + Eq + Hash> Rules<K> {
    pub fn new(rules: HashMap<K, Rule<K>>) -> Self {
        let parents = rules
            .iter()
            .flat_map(|(parent, rule)| rule.keys.iter().map(|key| (key.clone(), parent.clone())))
            .collect();
        Self { rules, parents }
    }

    pub fn get(&self, key: &K) -> Option<&Rule<K>> {
        self.rules.get(key)
    }
    pub fn get_parent(&self, key: &K) -> Option<&K> {
        self.parents.get(key)
    }
}

#[derive(Clone)]
pub struct Rule<K: Clone> {
    keys: Vec<K>,
    operation: Operation,
}
impl<K: Clone> Rule<K> {
    pub fn new(keys: Vec<K>, operation: Operation) -> Self {
        Self { keys, operation }
    }

    pub fn keys(&self) -> &[K] {
        &self.keys
    }

    pub fn eval(&self, vals: impl Iterator<Item = f32>) -> f32 {
        match self.operation {
            Operation::Sum => vals.sum(),
            Operation::Prod => vals.product(),
        }
    }
}

#[derive(Clone)]
pub enum Operation {
    Sum,
    Prod,
}
