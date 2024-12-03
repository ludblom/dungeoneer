use serde::{Deserialize, Serialize};

use crate::{general::Currency, weapon::Weapon};

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Gear {
    pub name: String,
    pub ac_modifier: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Object {
    pub name: String,
    pub cost: Option<Currency>,
    pub weight: Option<f64>,
    pub notes: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Bag {
    pub weapons: Vec<Weapon>,
    pub gear: Vec<Gear>,
    pub objects: Vec<Object>,
    pub total_value: Vec<Currency>,
    pub total_weight: Option<f64>,
}
