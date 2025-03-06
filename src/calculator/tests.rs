use std::collections::HashMap;

use crate::calculator::{Rule, Rules};

use super::{Calculator, Operation};

#[test]
fn calc_get() {
    let calcrules = Rules::new(HashMap::from([
        (3, Rule::new(vec![0, 1, 2], Operation::Sum)),
        (5, Rule::new(vec![3, 4], Operation::Prod)),
    ]));
    let mut calc = Calculator::new(HashMap::from([(0, 1.0), (2, 5.0), (4, 2.0)]), &calcrules);
    assert_eq!(calc.get(&5), 12.0);
    assert_eq!(calc.get(&6), 0.0);
}

#[test]
fn calc_set() {
    let calcrules = Rules::new(HashMap::from([
        (3, Rule::new(vec![0, 1, 2], Operation::Sum)),
        (5, Rule::new(vec![3, 4], Operation::Prod)),
    ]));
    let mut calc = Calculator::new(
        HashMap::from([(0, 1.0), (1, 4.0), (2, 5.0), (4, 2.0)]),
        &calcrules,
    );
    calc.set(4, 9.0);
    assert_eq!(calc.get(&5), 90.0);

    calc.set(3, 2.0);
    assert_eq!(calc.get(&5), 18.0);
}

#[test]
fn calc_remove() {
    let calcrules = Rules::new(HashMap::from([
        (3, Rule::new(vec![0, 1, 2], Operation::Sum)),
        (5, Rule::new(vec![3, 4], Operation::Prod)),
    ]));
    let mut calc = Calculator::new(
        HashMap::from([(0, 1.0), (1, 4.0), (2, 5.0), (3, 100.0), (4, 2.0)]),
        &calcrules,
    );
    assert_eq!(calc.get(&5), 200.0);

    calc.remove(&3);
    assert_eq!(calc.get(&5), 20.0);

    calc.remove(&1);
    assert_eq!(calc.get(&5), 12.0);
}
