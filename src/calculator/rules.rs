use std::{collections::HashMap, hash::Hash};

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

    pub fn eval(&self, vals: &[f32]) -> f32 {
        self.operation.eval(vals)
    }
}

#[derive(Clone)]
pub enum Operation {
    Sum,
    Prod,
    MultSum,
}
impl Operation {
    pub fn eval<'a>(&self, vals: &'a [f32]) -> f32 {
        match self {
            Self::Sum => vals.iter().fold(0.0, |a, v| a + v),
            Self::Prod => vals.iter().fold(1.0, |a, v| a * v),
            Self::MultSum => vals.iter().fold(1.0, |a, v| a + v),
        }
    }
}
