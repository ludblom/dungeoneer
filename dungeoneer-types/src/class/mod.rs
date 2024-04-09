use serde::{Deserialize, Serialize};

pub mod sorcerer;

// TODO Continue...
pub trait SpellCaster<T> {
    fn cast_spell(&self) -> Result<(), T>;
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
