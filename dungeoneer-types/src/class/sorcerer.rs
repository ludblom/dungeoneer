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

impl SpellCaster<String> for Sorcerer {
    fn cast_spell(&mut self, spell_level: usize, cost: u8) -> Result<(), String> {
        if self.spell_slots.len() == 0 || self.spell_slots.len() < spell_level+1 {
            return Err(format!("You do not have a level {} spell-slot.", spell_level));
        }

        let spell_slot = &mut self.spell_slots[spell_level];

        if spell_slot.available < cost {
            return Err(format!("You have {} free spell-slots, and that spell costs {}.", spell_slot.available, cost));
        }

        spell_slot.update_spell_slots(cost);

        Ok(())
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
    pub available: u8,
    pub total: u8,
}

impl SpellSlots {
    pub fn update_spell_slots(&mut self, cost: u8) {
        self.available -= cost;
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SorceryPoints {
    pub available: u8,
    pub total: u8,
}

#[cfg(test)]
mod tests {
    use crate::{race::AbilityScoreOptions, class::SpellCaster};
    use super::{Sorcerer, SpellSlots, Spell, Hit};

    #[test]
    fn cast_spell_decrement_test() {
        let mut sorcer: Sorcerer = Sorcerer {
            class_level: 1,
            spell_slots: vec![SpellSlots { available: 3, total: 3 }],
            sorcery_points: None,
            spells: vec![ Spell { range: 20, hit: Hit { dc: Some((AbilityScoreOptions::Charisma, 15)), hit: None } } ]
        };

        let _ = sorcer.cast_spell(1, 1);
        assert_eq!(sorcer.spell_slots[0].available, 2);

        let _ = sorcer.cast_spell(1, 1);
        assert_eq!(sorcer.spell_slots[0].available, 1);

        let _ = sorcer.cast_spell(1, 1);
        assert_eq!(sorcer.spell_slots[0].available, 0);

        let res = sorcer.cast_spell(1, 1);
        match res {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }
}
