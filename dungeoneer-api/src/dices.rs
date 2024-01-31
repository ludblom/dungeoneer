use dungeoneer_types::general::Dices;
use rand::Rng;

pub fn roll_dices(dices: Dices) -> u64 {
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

pub fn roll_d4() -> u64 {
    rand::thread_rng().gen_range(1..5)
}

pub fn roll_d6() -> u64 {
    rand::thread_rng().gen_range(1..7)
}

pub fn roll_d8() -> u64 {
    rand::thread_rng().gen_range(1..9)
}

pub fn roll_d10() -> u64 {
    rand::thread_rng().gen_range(1..11)
}

pub fn roll_d12() -> u64 {
    rand::thread_rng().gen_range(1..13)
}

pub fn roll_d20() -> u64 {
    rand::thread_rng().gen_range(1..21)
}

pub fn roll_d100() -> u64 {
    rand::thread_rng().gen_range(1..101)
}
