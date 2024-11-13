use crate::element::Element;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Attribute {
    Elemental(Element),
    Physical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Type {
    NormalAttack,
    ChargedAttack,
    PlungeAttack,
    ElementalSkill,
    ElementalBurst
}
