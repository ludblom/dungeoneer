use chrono::Local;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::race::Race;
use crate::class::Class;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Character {
    pub id: Uuid,
    pub name: String,
    pub gender: Gender,
    pub level: u64,
    pub languages: Vec<Language>,
    pub class: Vec<Class>,
    pub race: Race,
    pub ability_score: AbilityScore,
    pub health: i64,
    pub total_health: i64,
    pub temporary_health: i64,
    pub created: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Gender {
    Male,
    Female,
    Unknown,
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

impl Character {
    pub fn new(
        name: String,
        gender: Gender,
        languages: Vec<Language>,
        class: Class,
        race: Race,
        ability_score: AbilityScore,
    ) -> Character {
        let health = Character::calculate_total_health(&class, &ability_score);
        Character {
            id: Uuid::new_v4(),
            name,
            gender,
            level: 1,
            languages,
            class: vec![class],
            race,
            ability_score,
            health,
            total_health: health,
            temporary_health: 0,
            created: Local::now().to_string(),
        }
    }

    pub fn long_rest(&mut self) {
        self.health = self.total_health;
        // self.spell_slots.as_mut().map(|spell_slots| {
        //     spell_slots.iter_mut().for_each(|s| s.used = 0);
        // });
    }

    fn calculate_total_health(class: &Class, ability_score: &AbilityScore) -> i64 {
        let class_health: i64 = match class {
            Class::Barbarian => 12,
            Class::Sorcerer => 6,
            _ => todo!("Not defined"),
        };
        class_health + ability_score.constitution.modifier
    }
}
