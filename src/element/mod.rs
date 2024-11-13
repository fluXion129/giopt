#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Element {
    Anemo,
    Geo,
    Electro,
    Dendro,
    Hydro,
    Pyro,
    Cryo,
}

#[derive(Clone, Debug)]
pub struct ElementalApplication {
    element: Element,
    units: f32,
}
impl ElementalApplication {
    pub fn new(element: Element, units: f32) -> Self {
        Self { element, units }
    }
    pub fn element(&self) -> Element {
        self.element
    }
}

pub mod reaction;
