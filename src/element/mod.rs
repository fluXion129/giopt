/// Note for calculator "mux": indexing starts at 1 and is in order
/// that is written down here (order of nation's release)
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
impl Element {
    /// Return the index used by the calculator for each element.
    pub fn calcindex(&self) -> f32 {
        match *self {
            Self::Anemo => 0.0,
            Self::Geo => 1.0,
            Self::Electro => 2.0,
            Self::Dendro => 3.0,
            Self::Hydro => 4.0,
            Self::Pyro => 5.0,
            Self::Cryo => 6.0,
        }
    }
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
    pub fn aura(&self) -> Aura {
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
