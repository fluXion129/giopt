use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Element { Anemo, Geo, Electro, Dendro, Hydro, Pyro, Cryo }

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DamageAttribute { Elemental(Element), Physical }

pub enum DamageType { NormalAttack, ChargedAttack, PlungeAttack, ElementalSkill, ElementalBurst }

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StatType {
    Hp,
    Atk,
    Def,

    EleMast,

    CritRate,
    CritDmg,

    EnergyRecharge,

    CooldownReduction,
    
    ShieldStrength,

    HealingBonus,
    IncomingHealBonus,

    // ------------------- DMG Bonuses ------------------------
    AllDMG, 
    
    AttributeDMG(DamageAttribute),
    TypeDMG(DamageType),
    TalentDMG(Talent), // Talent DMG Bonuses are rare bonuses that apply to a specific talent on a character. They should really only come from a character's talents, never from equipment.
    // --------------------------------------------------------
}

pub struct CharacterStats(pub HashMap<StatType, f32>);

impl CharacterStats {
    pub fn get_stat(&self, key: &StatType) -> f32 {
        self.0.get(key).unwrap_or(0.0)
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Talent {
    NormalAttack(usize),
    ChargedAttack(usize),
    PlungeCollision,
    LowPlunge,
    HighPlunge,
    ElementalSkill(usize),
    ElementalBurst(usize),
}

pub struct Stat {
    key: StatType,
    value: f32
}


pub fn calc_base_dmg(stats: &CharacterStats, scalings: &[Stat]) -> f32 {
    scalings.iter().fold(0.0, |acc, scaling| acc + stats.get_stat(&scaling.key) * scaling.value)
}

// often poorly understood or unclearly referenced, this calculates the pre-multiplier damage (pm damage),
// which is after the base multipliers and flat additions are added, before the damage has all of
// the multipliers applied to it.
pub fn calc_pm_dmg(base_damage: f32, base_damage_mult: f32, base_damage_flat: f32) -> f32 {
    base_damage * base_damage_mult + base_damage_flat
}

// calculate non-crit damage, a good option to have as endpoint, but also a midpoint to later calculate avg or on-crit dmg.
// calculation steps
pub fn calc_nc_dmg(pm_damage: f32, total_dmg_bonus: f32, target_dmg_reduction: f32, target_def_mult: f32, target_res_mult: f32, amp_reaction_mult: f32) -> f32 {
    pm_damage * (1f32 + total_dmg_bonus - target_dmg_reduction) * target_def_mult * target_res_mult * amp_reaction_mult
}

// Talent, Damage Type, and Damage Attribute are frequently linked, but ultimately can be
// unassociated. Raiden, for instance, converts her Normal Attacks from Physical Normal Attack DMG
// to Electro Burst DMG.
pub fn calc_dmg_bonus(stats: &CharacterStats, talent: Talent, damage_type: DamageType, damage_attribute: DamageAttribute) -> f64 {
    let attribute_dmg_bonus = stats.get_stat(StatType::AttributeDMG(damage_attribute));
    let type_dmg_bonus = stats.get_stat(StatType::TypeDMG(damage_type));
    let talent_dmg_bonus = stats.get_stat(StatType::TalentDMG(talent));
    let all_dmg_bonus = stats.get_stat(StatType::AllDMG);

    all_dmg_bonus + attribute_dmg_bonus + type_dmg_bonus + talent_dmg_bonus
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn base_dmg_calculated_correctly() {
        let char1 = CharacterStats(HashMap::from([
            (StatType::Hp, 1000.0),
            (StatType::Atk, 500.0),
            (StatType::Def, 200.0),
            (StatType::EM, 81.0),
            (StatType::CR, 0.224),
            (StatType::CDmg, 0.887),
            (StatType::ER, 1.231)
        ]));

        let scalings = [Stat{key: StatType::Atk, value: 1.0}, Stat{key: StatType::EM, value: 5.0}];

        let base_damage = calc_base_dmg(&char1, &scalings);
        
        assert_eq!(905.0, base_damage);
    }

    #[test]
    fn dmg_bonuses_calculated_correctly() {
        let char1 = CharacterStats(HashMap::from([
            (StatType::Hp, 1000.0),
            (StatType::Atk, 500.0),
            (StatType::Def, 200.0),
            (StatType::EM, 81.0),
            (StatType::CR, 0.224),
            (StatType::CDmg, 0.887),
            (StatType::ER, 1.231)
        ]));
        
        let 
    }
    
}
