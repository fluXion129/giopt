use std::collections::HashMap;

use super::rules::{product, sum, Rule, Rules};

use super::Calculator;

#[test]
fn calc_get() {
    let calcrules = Rules::new(HashMap::from([
        (3, Rule::new(&sum, vec![0, 1, 2])),
        (5, Rule::new(&product, vec![3, 4])),
    ]));
    let mut calc =
        Calculator::from_components(HashMap::from([(0, 1.0), (2, 5.0), (4, 2.0)]), &calcrules);
    assert_eq!(calc.get(&5), 12.0);
    assert_eq!(calc.get(&6), 0.0);
}

#[test]
fn calc_set() {
    let calcrules = Rules::new(HashMap::from([
        (3, Rule::new(&sum, vec![0, 1, 2])),
        (5, Rule::new(&product, vec![3, 4])),
    ]));
    let mut calc = Calculator::from_components(
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
        (3, Rule::new(&sum, vec![0, 1, 2])),
        (5, Rule::new(&product, vec![3, 4])),
    ]));
    let mut calc = Calculator::from_components(
        HashMap::from([(0, 1.0), (1, 4.0), (2, 5.0), (3, 100.0), (4, 2.0)]),
        &calcrules,
    );
    assert_eq!(calc.get(&5), 200.0);

    calc.remove(&3);
    assert_eq!(calc.get(&5), 20.0);

    calc.remove(&1);
    assert_eq!(calc.get(&5), 12.0);
}
