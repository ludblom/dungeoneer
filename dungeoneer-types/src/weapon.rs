use serde::{Deserialize, Serialize};

use crate::general::{Currency, Dices};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Weapon {
    pub name: String,
    pub attack_type: Option<AttackType>,
    pub damage_type: Vec<PhysicalDamage>,
    pub range: u64,
    pub damage: Dices,
    pub cost: Option<Currency>,
    pub weight: Option<f64>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AttackType {
    Melee,
    Ranged,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PhysicalDamage {
    Acid,
    Bludgeoning,
    Cold,
    Fire,
    ForceLightning,
    Necrotic,
    Piercing,
    Poison,
    Psychic,
    Radiant,
    Slashing,
    Thunder,
}

pub enum WeaponList {
    Club,
    Dagger,
    Greatclub,
    Handaxe,
    Javelin,
    LightHammer,
    Mace,
    Quarterstaff,
    Sickle,
    Spear,
    // CrossbowLight,
    // Dart,
    // Shortbow,
    // Sling,
    // Battleaxe,
    // Flail,
    // Glaive,
    // Greataxe,
    // Greatsword,
    // Halberd,
    // Lance,
    // Longsword,
    // Maul,
    // Morningstar,
    // Pike,
    // Rapier,
    // Scimitar,
    // Shortsword,
    // Trident,
    // WarPick,
    // Warhammer,
    // Whip,
    // Blowgun,
    // CrossbowHand,
    // CrossbowHeavy,
    // Longbow,
    // Net,
}

impl Weapon {
    pub fn get_weapon(weapon: Option<WeaponList>) -> Weapon {
        // TODO: http://dnd5e.wikidot.com/weapons
        let weapon = match weapon {
            Some(WeaponList::Club) => Weapon {
                name: "Club".to_string(),
                attack_type: Some(AttackType::Melee),
                damage_type: vec![PhysicalDamage::Bludgeoning],
                range: 5,
                damage: Dices::only_fours(1),
                cost: Some(Currency::sp_only(1)),
                weight: Some(2.0),
                notes: None,
            },
            Some(WeaponList::Dagger) => Weapon {
                name: "Dagger".to_string(),
                attack_type: Some(AttackType::Melee),
                damage_type: vec![PhysicalDamage::Piercing],
                range: 5,
                damage: Dices::only_fours(1),
                cost: Some(Currency::gp_only(2)),
                weight: Some(1.0),
                notes: None,
            },
            Some(WeaponList::Greatclub) => Weapon {
                name: "Greatclub".to_string(),
                attack_type: Some(AttackType::Melee),
                damage_type: vec![PhysicalDamage::Bludgeoning],
                range: 5,
                damage: Dices::only_eights(1),
                cost: Some(Currency::sp_only(2)),
                weight: Some(10.0),
                notes: None,
            },
            Some(WeaponList::Handaxe) => Weapon {
                name: "Handaxe".to_string(),
                attack_type: Some(AttackType::Melee),
                damage_type: vec![PhysicalDamage::Slashing],
                range: 5,
                damage: Dices::only_sizes(1),
                cost: Some(Currency::gp_only(5)),
                weight: Some(2.0),
                notes: None,
            },
            Some(WeaponList::Javelin) => Weapon {
                name: "Javelin".to_string(),
                attack_type: Some(AttackType::Melee),
                damage_type: vec![PhysicalDamage::Piercing],
                range: 5,
                damage: Dices::only_sizes(1),
                cost: Some(Currency::sp_only(5)),
                weight: Some(2.0),
                notes: None,
            },
            Some(WeaponList::LightHammer) => Weapon {
                name: "Light hammer".to_string(),
                attack_type: Some(AttackType::Melee),
                damage_type: vec![PhysicalDamage::Bludgeoning],
                range: 5,
                damage: Dices::only_fours(1),
                cost: Some(Currency::gp_only(2)),
                weight: Some(2.0),
                notes: None,
            },
            Some(WeaponList::Mace) => Weapon {
                name: "Mace".to_string(),
                attack_type: Some(AttackType::Melee),
                damage_type: vec![PhysicalDamage::Bludgeoning],
                range: 5,
                damage: Dices::only_sizes(1),
                cost: Some(Currency::gp_only(5)),
                weight: Some(4.0),
                notes: None,
            },
            Some(WeaponList::Quarterstaff) => Weapon {
                name: "Quarterstaff".to_string(),
                attack_type: Some(AttackType::Melee),
                damage_type: vec![PhysicalDamage::Bludgeoning],
                range: 5,
                damage: Dices::only_sizes(1),
                cost: Some(Currency::sp_only(2)),
                weight: Some(4.0),
                notes: None,
            },
            Some(WeaponList::Sickle) => Weapon {
                name: "Sickle".to_string(),
                attack_type: Some(AttackType::Melee),
                damage_type: vec![PhysicalDamage::Slashing],
                range: 5,
                damage: Dices::only_fours(1),
                cost: Some(Currency::gp_only(1)),
                weight: Some(2.0),
                notes: None,
            },
            Some(WeaponList::Spear) => Weapon {
                name: "Spear".to_string(),
                attack_type: Some(AttackType::Melee),
                damage_type: vec![PhysicalDamage::Piercing],
                range: 5,
                damage: Dices::only_sizes(1),
                cost: Some(Currency::gp_only(1)),
                weight: Some(3.0),
                notes: None,
            },
            _ => todo!("Continue"), // TODO: Continue to add all weapons
        };
        weapon
    }
}
