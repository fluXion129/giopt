use crate::element::Element;

/// Attribute of an attack.
/// For the calculator:
/// - Physical = 1
/// - Elemental = 2-8
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Attribute {
    Physical,
    Elemental(Element),
}
impl From<Option<Element>> for Attribute {
    fn from(value: Option<Element>) -> Self {
        match value {
            Some(e) => Self::Elemental(e),
            None => Self::Physical,
        }
    }
}
impl From<Element> for Attribute {
    fn from(value: Element) -> Self {
        Self::Elemental(value)
    }
}
impl Attribute {
    pub fn calcindex(&self) -> f32 {
        match *self {
            Self::Physical => 0.0,
            Self::Elemental(e) => 1.0 + e.calcindex(),
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
impl Category {
    pub fn calcindex(&self) -> f32 {
        match *self {
            Self::NormalAttack => 0.0,
            Self::ChargedAttack => 1.0,
            Self::PlungeAttack => 2.0,
            Self::ElementalBurst => 3.0,
            Self::ElementalSkill => 4.0,
        }
    }
}
