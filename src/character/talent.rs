use super::stats::Stat;

#[derive(Clone, PartialEq)]
pub struct Talent {
    typ: Type,
    scalings: Vec<Stat>
}
impl Talent {
    pub fn new(typ: Type, scalings: Vec<Stat>) -> Self {
        Self { typ, scalings }
    }

    pub fn get_scalings(&self) -> &[Stat] {
        &self.scalings
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Type {
    NormalAttack(u8), // the problem with talents is that every character has different numbers
    ChargedAttack(u8), // of distinct damage instances for each category.
    PlungeCollision,
    LowPlunge,
    HighPlunge,
    ElementalSkill(u8),
    ElementalBurst(u8),
}
