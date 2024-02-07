use dungeoneer_types::{character::{Character, Gender, AbilityScore, Language}, class::Class, race::{Race, RaceSelection, Proficiency}};

pub fn new_character(
    name: String,
    gender: Gender,
    languages: Vec<Language>,
    class: Class,
    race_selection: RaceSelection,
    proficiency: Proficiency,
    ability_score: AbilityScore
) -> Character {
    let race: Race = Race::new(race_selection, proficiency);
    let character = Character::new(name, gender, languages, class, race, ability_score);
    character
}
