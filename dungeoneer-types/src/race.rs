use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Race {
    pub race_selection: RaceSelection,
    pub ability_score_increase: HashMap<String, i64>,
    pub resilience: Vec<Resilience>,
    pub proficiency: Vec<Proficiency>,
    pub unique: Vec<Unique>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum RaceSelection {
    HillDwarf,
    MountainDwarf,
    HalfElf, // TODO: Not implemented
    Elf, // TODO: Not implemented
    Gnome, // TODO: Not implemented
    Halfling, // TODO: Not implemented
    HalfOrc, // TODO: Not implemented
    Human, // TODO: Not implemented
    Tiefling, // TODO: Not implemented
    Dragonborn, // TODO: Not implemented
}

#[derive(Debug, Clone, Deserialize, Serialize, strum_macros::Display)]
pub enum AbilityScoreOptions {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Resilience {
    Poison,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Proficiency {
    SmithsTools,
    BrewersSupplies,
    MansonsTools,
    LightArmor,
    MediumArmor,
    HeavyArmor,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Unique {
    DwarvenToughness,
}

impl Race {
    pub fn new(race_selection: RaceSelection, proficiency: Proficiency) -> Race {
        match race_selection {
            RaceSelection::HillDwarf => {
                let race = Race {
                    race_selection,
                    ability_score_increase: HashMap::from([
                        (AbilityScoreOptions::Constitution.to_string(), 2),
                        (AbilityScoreOptions::Wisdom.to_string(), 1),
                    ]),
                    resilience: vec![
                        Resilience::Poison,
                    ],
                    proficiency: vec![
                        proficiency
                    ],
                    unique: vec![
                        Unique::DwarvenToughness,
                    ]
                };
                race
            },
            RaceSelection::MountainDwarf => {
                let race = Race {
                    race_selection,
                    ability_score_increase: HashMap::from([
                        (AbilityScoreOptions::Constitution.to_string(), 2),
                        (AbilityScoreOptions::Strength.to_string(), 2),
                    ]),
                    resilience: vec![
                        Resilience::Poison,
                    ],
                    proficiency: vec![
                        proficiency,
                        Proficiency::LightArmor,
                        Proficiency::MediumArmor,
                    ],
                    unique: Vec::new(),
                };
                race
            },
            _ => todo!("Not implemented."),
        }
    }
}
