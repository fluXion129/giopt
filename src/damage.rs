use crate::element::Element;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Attribute {
    Elemental(Element),
    Physical,
}
impl From<Option<Element>> for Attribute {
    fn from(value: Option<Element>) -> Self {
        match value {
            Some(e) => Self::Elemental(e),
            None => Self::Physical,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Category {
    NormalAttack,
    ChargedAttack,
    PlungeAttack,
    ElementalSkill,
    ElementalBurst,
}
