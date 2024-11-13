use crate::AuraType;
use super::Element;

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

    Bloom, Hyperbloom, Burgeon,

    Burning,

    ForwardVaporize,
    ReverseVaporize,

    Freeze,

    ForwardMelt,
    ReverseMelt,
}
impl ElementalReaction {
    fn is_amp_reaction(&self) -> bool {
        match *self {
            Self::ForwardVaporize
            | Self::ReverseVaporize
            | Self::ForwardMelt
            | Self::ReverseMelt => true,
            _ => false,
        }
    }

    fn from_elements(aura: AuraType, trigger: Element) -> Option<Self> {
        match (aura, trigger) {
            (AuraType::Electro, Element::Anemo)
            | (AuraType::Hydro, Element::Anemo)
            | (AuraType::Pyro, Element::Anemo)
            | (AuraType::Cryo, Element::Anemo) => Some(Self::Swirl),

            (AuraType::Electro, Element::Geo)
            | (AuraType::Hydro, Element::Geo)
            | (AuraType::Pyro, Element::Geo)
            | (AuraType::Cryo, Element::Geo) => Some(Self::Crystallize),

            (AuraType::Electro, Element::Dendro) => Some(Self::Quicken),
            (AuraType::Catalyzed, Element::Electro) => Some(Self::Aggravate),
            (AuraType::Catalyzed, Element::Dendro) => Some(Self::Spread),

            (AuraType::Electro, Element::Hydro) | (AuraType::Hydro, Element::Electro) => {
                Some(Self::ElectroCharged)
            }

            (AuraType::Electro, Element::Pyro) | (AuraType::Pyro, Element::Electro) => {
                Some(Self::Overloaded)
            }

            (AuraType::Electro, Element::Cryo) | (AuraType::Cryo, Element::Electro) => {
                Some(Self::Superconduct)
            }

            (AuraType::Dendro, Element::Hydro) | (AuraType::Hydro, Element::Dendro) => {
                Some(Self::Bloom)
            }
            (AuraType::BloomCore, Element::Electro) => Some(Self::Hyperbloom),
            (AuraType::BloomCore, Element::Pyro) => Some(Self::Burgeon),

            (AuraType::Dendro, Element::Pyro) | (AuraType::Pyro, Element::Dendro) => {
                Some(Self::Burning)
            }

            (AuraType::Hydro, Element::Pyro) => Some(Self::ReverseVaporize),
            (AuraType::Pyro, Element::Hydro) => Some(Self::ForwardVaporize),

            (AuraType::Hydro, Element::Cryo) | (AuraType::Cryo, Element::Hydro) => {
                Some(Self::Freeze)
            }

            (AuraType::Pyro, Element::Cryo) => Some(Self::ReverseMelt),
            (AuraType::Cryo, Element::Pyro) => Some(Self::ForwardMelt),

            _ => None,
        }
    }

    fn amp_reaction_em_mult(em: f32) -> f32 {
        2.78 * em / (em + 1400.0)
    }

    fn tra_reaction_em_mult(em: f32) -> f32 {
        16.0 * em / (em + 2000.0)
    }

    fn add_reaction_em_mult(em: f32) -> f32 {
        5.0 * em / (em + 1200.0)
    }

    fn cry_em_dmg_absorb(em: f32) -> f32 {
        4.44 * em / (em + 1400.0)
    }

    const LV_MULTS: [f32; 100] = [1.0; 100];
    fn lv_mult(level: f32) -> f32 {
        ElementalReaction::LV_MULTS[level as usize]
    }

    fn apply(&self, level: f32, em: f32) {
        match *self {
            _ => todo!(),
        }
    }
}
