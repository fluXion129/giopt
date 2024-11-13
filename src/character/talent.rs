use crate::element::ElementalApplication;

use super::stats::Stat;

#[derive(Clone)]
pub struct Talent {
    typ: Type,
    elem_app: Option<ElementalApplication>,
    scalings: Vec<Stat>,
}
impl Talent {
    pub fn new(typ: Type, elem_app: Option<ElementalApplication>, scalings: Vec<Stat>) -> Self {
        Self {
            typ,
            elem_app,
            scalings,
        }
    }

    pub fn get_scalings(&self) -> &[Stat] {
        &self.scalings
    }
    pub fn application(&self) -> Option<&ElementalApplication> {
        self.elem_app.as_ref()
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
