use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Character {
    pub id: Uuid,
    pub name: String,
    pub gender: Gender,
    pub level: u64,
    pub class: Class,
    pub race: Race,
    pub updated: String,
    pub created: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RacialTraits {

}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Gender {
    Male,
    Female,
    Unknown,
}

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
pub enum Race {
    Dragonborn,
    Dwarf,
    Elf,
    Gnome,
    HalfElf,
    Halfling,
    HalfOrc,
    Human,
    Tiefling,
}
