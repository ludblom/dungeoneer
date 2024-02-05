use dungeoneer_types::{character::{Character, Gender, AbilityScore, Language}, class::Class, race::Race};

pub fn new_character(
    name: String,
    gender: Gender,
    languages: Vec<Language>,
    class: Class,
    race: Race,
    ability_score: AbilityScore
) -> Character {
    let character = Character::new(name, gender, languages, class, race, ability_score);
    character
}
