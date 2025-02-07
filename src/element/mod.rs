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
    units: f64,
}
impl ElementalApplication {
    pub fn new(element: Element, units: f64) -> Self {
        Self { element, units }
    }
    pub fn element(&self) -> Element {
        self.element
    }
    pub fn units(&self) -> &f64 {
        &self.units
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GaugedAura {
    aura: Aura,
    gauge: f32,
    gauge_decay_rate: f32,
}
impl GaugedAura {
    pub fn new(aura: Aura, gauge: f32, gauge_decay_rate: f32) -> Self {
        Self {
            aura,
            gauge,
            gauge_decay_rate,
        }
    }
    pub fn typ(&self) -> Aura {
        self.aura
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Aura {
    Electro,
    Dendro,
    Hydro,
    Pyro,
    Cryo,
    Catalyzed,
    BloomCore,
    Frozen,
    Burning,
    ElectroCharged,
}

pub mod reaction;
