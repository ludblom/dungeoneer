use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::race::Race;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Character {
    pub id: Uuid,
    pub name: String,
    pub gender: Gender,
    pub level: u64,
    pub class: Vec<Class>,
    pub race: Race,
    pub updated: String,
    pub created: String,
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
pub enum Language {
    Common,
    Dwarvish,
    Elvish,
    Giant,
    Gnomish,
    Goblin,
    Halfling,
    Orc,
    Abyssal,
    Celestial,
    Draconic,
    DeepSpeech,
    Infernal,
    Primordial,
    Sylvan,
    Undercommon,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AbilityScore {
    pub strength: AbilityScoreCalculation,
    pub dexterity: AbilityScoreCalculation,
    pub constitution: AbilityScoreCalculation,
    pub intelligence: AbilityScoreCalculation,
    pub wisdom: AbilityScoreCalculation,
    pub charisma: AbilityScoreCalculation,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AbilityScoreCalculation {
    pub modifier: i64,
    pub base_score: i64,
    pub racial_bonus: i64,
    pub ability_improvements: i64,
    pub misc_bonus: i64,
    pub set_score: i64,
    pub other_modifier: i64,
    pub override_score: i64,
}
