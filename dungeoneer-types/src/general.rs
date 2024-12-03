use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Currency {
    pub pp: i64,
    pub gp: i64,
    pub ep: i64,
    pub sp: i64,
    pub cp: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Dices {
    pub d_four:    Option<i64>,
    pub d_six:     Option<i64>,
    pub d_eight:   Option<i64>,
    pub d_ten:     Option<i64>,
    pub d_twelve:  Option<i64>,
    pub d_twenty:  Option<i64>,
    pub d_hundred: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AbilityRoll {
    pub chosen_roll: Vec<i64>,
    pub sum: i64,
    pub removed_roll: i64,
}

impl Currency {
    pub fn pp_only(pp: i64) -> Currency {
        Currency { pp, gp: 0, ep: 0, sp: 0, cp: 0 }
    }

    pub fn gp_only(gp: i64) -> Currency {
        Currency { pp: 0, gp, ep: 0, sp: 0, cp: 0 }
    }

    pub fn ep_only(ep: i64) -> Currency {
        Currency { pp: 0, gp: 0, ep, sp: 0, cp: 0 }
    }

    pub fn sp_only(sp: i64) -> Currency {
        Currency { pp: 0, gp: 0, ep: 0, sp, cp: 0 }
    }

    pub fn cp_only(cp: i64) -> Currency {
        Currency { pp: 0, gp: 0, ep: 0, sp: 0, cp }
    }
}

impl Dices {
    pub fn only_fours(dices: i64) -> Dices {
        Dices {
            d_four: Some(dices),
            d_six: None,
            d_eight: None,
            d_hundred: None,
            d_ten: None,
            d_twelve: None,
            d_twenty: None,
        }
    }

    pub fn only_sizes(dices: i64) -> Dices {
        Dices {
            d_four: None,
            d_six: Some(dices),
            d_eight: None,
            d_hundred: None,
            d_ten: None,
            d_twelve: None,
            d_twenty: None,
        }
    }

    pub fn only_eights(dices: i64) -> Dices {
        Dices {
            d_four: None,
            d_six: None,
            d_eight: Some(dices),
            d_hundred: None,
            d_ten: None,
            d_twelve: None,
            d_twenty: None,
        }
    }

    pub fn only_ten(dices: i64) -> Dices {
        Dices {
            d_four: None,
            d_six: None,
            d_eight: None,
            d_hundred: None,
            d_ten: Some(dices),
            d_twelve: None,
            d_twenty: None,
        }
    }

    pub fn only_twelve(dices: i64) -> Dices {
        Dices {
            d_four: None,
            d_six: None,
            d_eight: None,
            d_hundred: None,
            d_ten: None,
            d_twelve: Some(dices),
            d_twenty: None,
        }
    }

    pub fn only_twenty(dices: i64) -> Dices {
        Dices {
            d_four: None,
            d_six: None,
            d_eight: None,
            d_hundred: None,
            d_ten: None,
            d_twelve: None,
            d_twenty: Some(dices),
        }
    }

    pub fn only_hundred(dices: i64) -> Dices {
        Dices {
            d_four: None,
            d_six: None,
            d_eight: None,
            d_hundred: Some(dices),
            d_ten: None,
            d_twelve: None,
            d_twenty: None,
        }
    }
}
