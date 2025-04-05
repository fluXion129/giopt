use super::{CalcKey, Calculator};
use std::collections::{HashMap, HashSet};

/// A tree of mappings between tags that describes the mathematical relations between them.
///
/// For instance:
/// - 0: Sum\[1, 2]
/// - 2: Prod\[3, 4]
///
/// Would mean that tag 0 is equal to the sum of tags 1 and 2,
/// and tag 2 is equal to the product of tags 3 and 4
pub struct Rules<K: CalcKey> {
    rules: HashMap<K, Rule<K>>,
    parents: HashMap<K, K>,
}
impl<K: CalcKey> Rules<K> {
    /// returns none if rules are a strict tree, and returns the first key it finds that
    /// has has multiple parents if it is not a strict tree
    pub fn strict_tree(&self) -> Option<K> {
        let mut parents = HashSet::new();
        self.rules
            .values()
            .flat_map(|r| r.keys().iter())
            .find(|&p| !parents.insert(p))
            .cloned()
    }

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
pub struct Rule<K: CalcKey> {
    keys: Vec<K>,
    operation: &'static dyn Fn(&mut Calculator<K>, &[K]) -> f32,
}
impl<K: CalcKey> Rule<K> {
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
pub fn sum<K: CalcKey>(calc: &mut Calculator<K>, keys: &[K]) -> f32 {
    keys.iter().map(|k| calc.get(k)).sum()
}

/// Product node evaluator. All keys' values will be multiplied together.
pub fn product<K: CalcKey>(calc: &mut Calculator<K>, keys: &[K]) -> f32 {
    keys.iter().map(|k| calc.get(k)).product()
}

/// Mux selector node evaluator. The first node determines the index of the node to pick within the keys
/// excluding itself. If it contains an index that is not a valid option, it will panic.
pub fn mux<K: CalcKey>(calc: &mut Calculator<K>, keys: &[K]) -> f32 {
    let idxk = keys.first().expect("Mux Node will have index node");
    let index = calc.get(idxk) + 1.0;
    let key = keys
        .get(index as usize)
        .expect("Mux Node Index should correspond to a valid Node.");
    calc.get(key)
}

/// Mux selector, except defaults to 1 instead of panic.
pub fn mux_or_1<K: CalcKey>(calc: &mut Calculator<K>, keys: &[K]) -> f32 {
    let idxk = keys.first().expect("Mux Node will have index node");
    let index = calc.get(idxk) + 1.0;
    let key = keys.get(index as usize);
    key.map(|x| calc.get(x)).unwrap_or(1.0)
}

/// Mux selector, except defaults to 0 instead of panic.
pub fn mux_or_0<K: CalcKey>(calc: &mut Calculator<K>, keys: &[K]) -> f32 {
    let idxk = keys.first().expect("Mux Node will have index node");
    let index = calc.get(idxk) + 1.0;
    let key = keys.get(index as usize);
    key.map(|x| calc.get(x)).unwrap_or(0.0)
}

/// Same as sum node, but adds one to it.
pub fn sum_plus_one<K: CalcKey>(calc: &mut Calculator<K>, keys: &[K]) -> f32 {
    keys.iter().map(|k| calc.get(k)).sum::<f32>() + 1.0
}

/// Negation node evaluator. The first node passed in will be negated and returned.
pub fn neg<K: CalcKey>(calc: &mut Calculator<K>, keys: &[K]) -> f32 {
    -calc.get(
        keys.first()
            .expect("neg nodes should be passed exactly one key"),
    )
}
