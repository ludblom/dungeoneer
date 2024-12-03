use serde::{Deserialize, Serialize};

pub mod sorcerer;

// TODO Continue...
pub trait SpellCaster<T> {
    fn cast_spell(&mut self, spell_level: usize, number_of_slots: u8) -> Result<(), T>;
}

#[derive(Debug, Deserialize, Serialize)]
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
