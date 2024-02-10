use serde::{Deserialize, Serialize};

use crate::{general::Dices, race::AbilityScoreOptions};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ClassEnum {
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

// TODO Break this out to a sorcerer.rs file
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Sorcerer {
    pub class_level: u32,
    pub spell_slots: Vec<SpellSlots>,
    pub sorcery_points: Option<SorceryPoints>,
    pub spells: Vec<Spell>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Spell {
    pub range: u64,
    pub hit: Hit,
}

type Threshold = i64;
type Modifier = i64;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Hit {
    pub dc: (AbilityScoreOptions, Threshold ),
    pub hit: (Modifier, Dices),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpellSlots {
    pub level: u8,
    pub available: u8,
    pub used: u8,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SorceryPoints {
    pub available: u8,
    pub used: u8,
}
