// TODO - Remove once completed
#![allow(dead_code)]

pub mod calculator;
pub mod gi_calculator;
pub use crate::gi_calculator::gi_rules::{B, GCK, GI_RULES, L, S};
pub mod character;
pub mod damage;
pub mod damage_calculator;
pub mod element;
pub mod equipment;
pub mod stats;

// Uncategorized genshin data types
enum Ascension {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}
