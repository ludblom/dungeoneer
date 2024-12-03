use dungeoneer_types::general::{Dices, AbilityRoll};
use rand::Rng;

pub fn roll_dices(dices: Dices) -> i64 {
    let d_four = match dices.d_four {
        Some(n) => (0..n).map(|_| roll_d4()).sum(),
        None => 0,
    };

    let d_six = match dices.d_six {
        Some(n) => (0..n).map(|_| roll_d6()).sum(),
        None => 0,
    };

    let d_eight = match dices.d_eight {
        Some(n) => (0..n).map(|_| roll_d8()).sum(),
        None => 0,
    };

    let d_ten = match dices.d_ten {
        Some(n) => (0..n).map(|_| roll_d10()).sum(),
        None => 0,
    };

    let d_twelve = match dices.d_twelve {
        Some(n) => (0..n).map(|_| roll_d12()).sum(),
        None => 0,
    };

    let d_twenty = match dices.d_twenty {
        Some(n) => (0..n).map(|_| roll_d20()).sum(),
        None => 0,
    };

    let d_hundred = match dices.d_hundred {
        Some(n) => (0..n).map(|_| roll_d100()).sum(),
        None => 0,
    };

    d_four + d_six + d_eight + d_ten + d_twelve + d_twenty + d_hundred
}

pub fn roll_d4() -> i64 {
    rand::thread_rng().gen_range(1..5)
}

pub fn roll_d6() -> i64 {
    rand::thread_rng().gen_range(1..7)
}

pub fn roll_d8() -> i64 {
    rand::thread_rng().gen_range(1..9)
}

pub fn roll_d10() -> i64 {
    rand::thread_rng().gen_range(1..11)
}

pub fn roll_d12() -> i64 {
    rand::thread_rng().gen_range(1..13)
}

pub fn roll_d20() -> i64 {
    rand::thread_rng().gen_range(1..21)
}

pub fn roll_d100() -> i64 {
    rand::thread_rng().gen_range(1..101)
}

pub fn ability_roll() -> AbilityRoll {
    let mut rolls: Vec<i64> = (0..4).map(|_| roll_d6()).collect();
    let lowest: i64 = *rolls.iter().min().unwrap();
    let mut i = 0;
    while i < rolls.len() {
        if rolls[i] == lowest {
            rolls.remove(i);
            break;
        }
        i += 1;
    }
    AbilityRoll {
        chosen_roll: rolls.clone(),
        sum: rolls.iter().sum(),
        removed_roll: lowest,
    }
}

#[cfg(test)]
mod tests {
    use dungeoneer_types::general::AbilityRoll;

    use super::ability_roll;

    #[test]
    fn test_ability_roll() {
        for _ in 1..400 {
            let roll: AbilityRoll = ability_roll();
            assert!(roll.sum <= 18);
            assert!(roll.chosen_roll.len() == 3);
        }
    }
}
