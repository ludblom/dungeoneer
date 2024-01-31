use serde::{Deserialize, Serialize};

use crate::weapon::Weapon;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Equipment {
    pub main_hand: Option<Weapon>,
    pub off_hand: Option<Weapon>,
    pub weapons_bag: Vec<Weapon>,
    pub head: Option<Gear>,
    pub chest: Option<Gear>,
    pub pants: Option<Gear>,
    pub feet: Option<Gear>,
    pub gear_bag: Vec<Gear>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Gear {
    pub name: String,
    pub ac_modifier: Option<i64>,
}
