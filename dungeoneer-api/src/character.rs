use std::collections::HashMap;

use dungeoneer_types::{character::{Character, Gender, AbilityScore, Language}, class::Class, race::{Race, RaceSelection, Proficiency, AbilityScoreOptions, Resilience, Unique}};

pub fn new_character(
    name: String,
    gender: Gender,
    languages: Vec<Language>,
    class: Class,
    race_selection: RaceSelection,
    proficiency: Proficiency,
    ability_score: AbilityScore
) -> Character {
    let race: Race = {
        let proficiency = proficiency;
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
    };
    let character = Character::new(name, gender, languages, class, race, ability_score);
    character
}
