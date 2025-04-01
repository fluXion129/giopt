use super::{Aura, Element};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ElementalReaction {
    Swirl,

    Crystallize,

    Quicken,
    Aggravate,
    Spread,

    ElectroCharged,

    Overloaded,

    Superconduct,

    Bloom,
    Hyperbloom,
    Burgeon,

    Burning,

    ForwardVaporize,
    ReverseVaporize,

    Freeze,

    ForwardMelt,
    ReverseMelt,
}
impl ElementalReaction {
    pub fn amp_rxn_type_calcindex(&self) -> f32 {
        match *self {
            Self::ForwardVaporize | Self::ReverseVaporize => 0.0,
            Self::ForwardMelt | Self::ReverseMelt => 1.0,
            _ => 2.0,
        }
    }
    // pub fn is_amp_reaction(&self) -> bool {
    //     match *self {
    //         Self::ForwardVaporize
    //         | Self::ReverseVaporize
    //         | Self::ForwardMelt
    //         | Self::ReverseMelt => true,
    //         _ => false,
    //     }
    // }

    pub fn from_elements(aura: Aura, trigger: Element) -> Option<Self> {
        match (aura, trigger) {
            (Aura::Electro, Element::Anemo)
            | (Aura::Hydro, Element::Anemo)
            | (Aura::Pyro, Element::Anemo)
            | (Aura::Cryo, Element::Anemo) => Some(Self::Swirl),

            (Aura::Electro, Element::Geo)
            | (Aura::Hydro, Element::Geo)
            | (Aura::Pyro, Element::Geo)
            | (Aura::Cryo, Element::Geo) => Some(Self::Crystallize),

            (Aura::Electro, Element::Dendro) => Some(Self::Quicken),
            (Aura::Catalyzed, Element::Electro) => Some(Self::Aggravate),
            (Aura::Catalyzed, Element::Dendro) => Some(Self::Spread),

            (Aura::Electro, Element::Hydro) | (Aura::Hydro, Element::Electro) => {
                Some(Self::ElectroCharged)
            }

            (Aura::Electro, Element::Pyro) | (Aura::Pyro, Element::Electro) => {
                Some(Self::Overloaded)
            }

            (Aura::Electro, Element::Cryo) | (Aura::Cryo, Element::Electro) => {
                Some(Self::Superconduct)
            }

            (Aura::Dendro, Element::Hydro) | (Aura::Hydro, Element::Dendro) => Some(Self::Bloom),
            (Aura::BloomCore, Element::Electro) => Some(Self::Hyperbloom),
            (Aura::BloomCore, Element::Pyro) => Some(Self::Burgeon),

            (Aura::Dendro, Element::Pyro) | (Aura::Pyro, Element::Dendro) => Some(Self::Burning),

            (Aura::Hydro, Element::Pyro) => Some(Self::ReverseVaporize),
            (Aura::Pyro, Element::Hydro) => Some(Self::ForwardVaporize),

            (Aura::Hydro, Element::Cryo) | (Aura::Cryo, Element::Hydro) => Some(Self::Freeze),

            (Aura::Pyro, Element::Cryo) => Some(Self::ReverseMelt),
            (Aura::Cryo, Element::Pyro) => Some(Self::ForwardMelt),

            _ => None,
        }
    }
}
