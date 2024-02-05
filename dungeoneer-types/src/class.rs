use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Class {
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rouge,
    Sorcerer,
    Warlock,
    Wizard,
    Artificer,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpellSlots {
    pub level: u8,
    pub available: u8,
    pub used: u8,
}
