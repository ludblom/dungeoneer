use serde::{Deserialize, Serialize};

use crate::{general::Dices, race::AbilityScoreOptions};
use crate::class::SpellCaster;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Sorcerer {
    pub class_level: u32,
    pub spell_slots: Vec<SpellSlots>,
    pub sorcery_points: Option<SorceryPoints>,
    pub spells: Vec<Spell>
}

impl SpellCaster<&'static str> for Sorcerer {
    fn cast_spell(&self) -> Result<(), &'static str> {
        Err("Test")
    }
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
    pub dc: Option<(AbilityScoreOptions, Threshold)>,
    pub hit: Option<(Modifier, Dices)>,
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

// pub fn get_base_spell_slots(class: &Class) -> Option<Vec<SpellSlots>> {
//     match class {
//         Class::Sorcerer => Some(vec![
//             SpellSlots { level: 0, available: 4, used: 0 },
//             SpellSlots { level: 1, available: 2, used: 0 },
//         ]),
//         Class::Barbarian => None,
//         _ => todo!("Not implemented."),
//     }
// }

#[cfg(test)]
mod tests {
    use crate::race::AbilityScoreOptions;
    use crate::class::SpellCaster;

    use super::{Sorcerer, SpellSlots, Spell, Hit};

    #[test]
    fn cast_spell_test() {
        let sorcer: Sorcerer = Sorcerer {
            class_level: 1,
            spell_slots: vec![SpellSlots { level: 1, available: 3, used: 0 }],
            sorcery_points: None,
            spells: vec![ Spell { range: 20, hit: Hit { dc: Some((AbilityScoreOptions::Charisma, 15)), hit: None } } ]
        };
        println!("{}", sorcer.cast_spell());
    }
}
