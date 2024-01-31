use serde::{Deserialize, Serialize};

use crate::general::Currency;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Armor {
    pub name: String,
    pub armor_type: ArmorType,
    pub armor_class: ArmorClass,
    pub strength: Option<u64>,
    pub stealth_disadvantage: bool,
    pub cost: Option<Currency>,
    pub weight: Option<f64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ArmorType {
    LightArmor,
    MediumArmor,
    HeavyArmor,
    Shield,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ArmorClass {
    pub default: u64,
    pub dex_modifier: bool,
    pub max_dex_modifier: Option<u64>,
    pub ac_increment: Option<u64>,
}

pub enum ArmorList {
    // Light Armor
    Padded,
    Leather,
    StuddedLeather,
    // Medium Armor
    Hide,
    ChainShirt,
    ScaleMail,
    SpikedArmor,
    Breastplate,
    Halfplate,
    // Heavy Armor
    RingMail,
    ChainMail,
    Splint,
    Plate,
    // Shield
    Shield,
}

impl Armor {
    pub fn get_armor(armor: ArmorList) -> Armor {
        let armor = match armor {
            // Light Armor
            ArmorList::Padded => Armor {
                name: "Padded".to_string(),
                armor_type: ArmorType::LightArmor,
                armor_class: ArmorClass { default: 11, dex_modifier: true, max_dex_modifier: None, ac_increment: None },
                strength: None,
                stealth_disadvantage: true,
                cost: Some(Currency::gp_only(5)),
                weight: Some(8.0),
            },
            ArmorList::Leather => Armor {
                name: "Leather".to_string(),
                armor_type: ArmorType::LightArmor,
                armor_class: ArmorClass { default: 11, dex_modifier: true, max_dex_modifier: None, ac_increment: None },
                strength: None,
                stealth_disadvantage: false,
                cost: Some(Currency::gp_only(10)),
                weight: Some(10.0),
            },
            ArmorList::StuddedLeather => Armor {
                name: "Studded Leather".to_string(),
                armor_type: ArmorType::LightArmor,
                armor_class: ArmorClass { default: 12, dex_modifier: true, max_dex_modifier: None, ac_increment: None },
                strength: None,
                stealth_disadvantage: false,
                cost: Some(Currency::gp_only(45)),
                weight: Some(13.0),
            },
            // Medium Armor
            ArmorList::Hide => Armor {
                name: "Hide".to_string(),
                armor_type: ArmorType::MediumArmor,
                armor_class: ArmorClass { default: 12, dex_modifier: true, max_dex_modifier: Some(2), ac_increment: None },
                strength: None,
                stealth_disadvantage: false,
                cost: Some(Currency::gp_only(10)),
                weight: Some(12.0),
            },
            ArmorList::ChainShirt => Armor {
                name: "Chain Shirt".to_string(),
                armor_type: ArmorType::MediumArmor,
                armor_class: ArmorClass { default: 13, dex_modifier: true, max_dex_modifier: Some(2), ac_increment: None },
                strength: None,
                stealth_disadvantage: false,
                cost: Some(Currency::gp_only(50)),
                weight: Some(20.0),
            },
            ArmorList::ScaleMail => Armor {
                name: "Scale Mail".to_string(),
                armor_type: ArmorType::MediumArmor,
                armor_class: ArmorClass { default: 14, dex_modifier: true, max_dex_modifier: Some(2), ac_increment: None },
                strength: None,
                stealth_disadvantage: true,
                cost: Some(Currency::gp_only(50)),
                weight: Some(45.0),
            },
            ArmorList::SpikedArmor => Armor {
                name: "Spiked Armor".to_string(),
                armor_type: ArmorType::MediumArmor,
                armor_class: ArmorClass { default: 14, dex_modifier: true, max_dex_modifier: Some(2), ac_increment: None },
                strength: None,
                stealth_disadvantage: true,
                cost: Some(Currency::gp_only(75)),
                weight: Some(45.0),
            },
            ArmorList::Breastplate => Armor {
                name: "Breastplate".to_string(),
                armor_type: ArmorType::MediumArmor,
                armor_class: ArmorClass { default: 14, dex_modifier: true, max_dex_modifier: Some(2), ac_increment: None },
                strength: None,
                stealth_disadvantage: false,
                cost: Some(Currency::gp_only(400)),
                weight: Some(20.0),
            },
            ArmorList::Halfplate => Armor {
                name: "Halfplate".to_string(),
                armor_type: ArmorType::MediumArmor,
                armor_class: ArmorClass { default: 15, dex_modifier: true, max_dex_modifier: Some(2), ac_increment: None },
                strength: None,
                stealth_disadvantage: true,
                cost: Some(Currency::gp_only(750)),
                weight: Some(40.0),
            },
            // Heavy Armor
            ArmorList::RingMail => Armor {
                name: "Ring Mail".to_string(),
                armor_type: ArmorType::HeavyArmor,
                armor_class: ArmorClass { default: 14, dex_modifier: false, max_dex_modifier: None, ac_increment: None },
                strength: None,
                stealth_disadvantage: true,
                cost: Some(Currency::gp_only(30)),
                weight: Some(40.0),
            },
            ArmorList::ChainMail => Armor {
                name: "Chain Mail".to_string(),
                armor_type: ArmorType::HeavyArmor,
                armor_class: ArmorClass { default: 16, dex_modifier: false, max_dex_modifier: None, ac_increment: None },
                strength: Some(13),
                stealth_disadvantage: true,
                cost: Some(Currency::gp_only(75)),
                weight: Some(55.0),
            },
            ArmorList::Splint => Armor {
                name: "Splint".to_string(),
                armor_type: ArmorType::HeavyArmor,
                armor_class: ArmorClass { default: 17, dex_modifier: false, max_dex_modifier: None, ac_increment: None },
                strength: Some(15),
                stealth_disadvantage: true,
                cost: Some(Currency::gp_only(200)),
                weight: Some(60.0),
            },
            ArmorList::Plate => Armor {
                name: "Plate".to_string(),
                armor_type: ArmorType::HeavyArmor,
                armor_class: ArmorClass { default: 18, dex_modifier: false, max_dex_modifier: None, ac_increment: None },
                strength: Some(15),
                stealth_disadvantage: true,
                cost: Some(Currency::gp_only(1500)),
                weight: Some(65.0),
            },
            // Shield
            ArmorList::Shield => Armor {
                name: "Shield".to_string(),
                armor_type: ArmorType::Shield,
                armor_class: ArmorClass { default: 0, dex_modifier: false, max_dex_modifier: None, ac_increment: Some(2) },
                strength: Some(15),
                stealth_disadvantage: true,
                cost: Some(Currency::gp_only(1500)),
                weight: Some(65.0),
            }
        };
        armor
    }

    pub fn create_armor(
        name: String,
        armor_type: ArmorType,
        armor_class: ArmorClass,
        strength: Option<u64>,
        stealth_disadvantage: bool,
        cost: Option<Currency>,
        weight: Option<f64>
    ) -> Armor {
        Armor {
            name,
            armor_type,
            armor_class,
            strength,
            stealth_disadvantage,
            cost,
            weight
        }
    }
}
