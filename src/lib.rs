#[derive(Clone, Debug, PartialEq)]
pub struct Aura {
    typ: AuraType,
    gauge: f32,
    gauge_decay_rate: f32,
}
impl Aura {
    pub fn new(typ: AuraType, gauge: f32, gauge_decay_rate: f32) -> Self {
        Self {
            typ,
            gauge,
            gauge_decay_rate,
        }
    }
    pub fn typ(&self) -> AuraType {
        self.typ
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AuraType {
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

mod damage_calculator;

mod damage;

mod element;

mod character;
