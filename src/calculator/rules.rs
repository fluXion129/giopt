use super::Calculator;
use std::{collections::HashMap, hash::Hash};

/// A tree of mappings between tags that describes the mathematical relations between them.
///
/// For instance:
/// - 0: Sum\[1, 2]
/// - 2: Prod\[3, 4]
/// Would mean that tag 0 is equal to the sum of tags 1 and 2,
/// and tag 2 is equal to the product of tags 3 and 4
pub struct Rules<K: Clone + Eq + Hash + 'static> {
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
pub struct Rule<K: Clone + Eq + Hash + 'static> {
    keys: Vec<K>,
    operation: &'static dyn Fn(&mut Calculator<K>, &[K]) -> f32,
}
impl<K: Clone + Eq + Hash + 'static> Rule<K> {
    pub fn new(operation: &'static dyn Fn(&mut Calculator<K>, &[K]) -> f32, keys: Vec<K>) -> Self {
        Self { keys, operation }
    }

    pub fn keys(&self) -> &[K] {
        &self.keys
    }
    pub fn op(&self) -> &'static dyn Fn(&mut Calculator<K>, &[K]) -> f32 {
        self.operation
    }
}

/// Sum node evaluator. All keys' values will be added together.
pub fn sum<K: Clone + Eq + Hash + 'static>(calc: &mut Calculator<K>, keys: &[K]) -> f32 {
    keys.iter().map(|k| calc.get(k)).sum()
}

/// Product node evaluator. All keys' values will be multiplied together.
pub fn product<K: Clone + Eq + Hash + 'static>(calc: &mut Calculator<K>, keys: &[K]) -> f32 {
    keys.iter().map(|k| calc.get(k)).product()
}

/// Mux selector node evaluator. The first node determines the index of the node to pick within the keys
/// excluding itself. If it contains an index that is not a valid option, it will panic.
pub fn mux<K: Clone + Eq + Hash + 'static>(calc: &mut Calculator<K>, keys: &[K]) -> f32 {
    let index = calc.get(keys.get(0).expect("Mux Node will have index node"));
    let key = keys
        .get(index as usize + 1)
        .expect("Mux Node Index should correspond to a valid Node.");
    calc.get(key)
}

pub fn sum_mult_bonus<K: Clone + Eq + Hash + 'static>(calc: &mut Calculator<K>, keys: &[K]) -> f32 {
    keys.iter().map(|k| calc.get(k)).fold(1.0, |a, x| a + x)
}
