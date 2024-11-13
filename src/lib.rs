pub struct Aura {
    aura_type: AuraType,
    gauge: f32,
    gauge_decay_rate: f32,
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
