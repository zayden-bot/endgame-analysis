use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};

pub const TIERS: [Tier; 7] = [
    Tier::S,
    Tier::A,
    Tier::B,
    Tier::C,
    Tier::D,
    Tier::E,
    Tier::F,
];

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Deserialize, Serialize)]
pub enum Tier {
    S,
    A,
    B,
    C,
    D,
    E,
    F,
}

impl FromStr for Tier {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "S" => Ok(Tier::S),
            "A" => Ok(Tier::A),
            "B" => Ok(Tier::B),
            "C" => Ok(Tier::C),
            "D" => Ok(Tier::D),
            "E" => Ok(Tier::E),
            "F" => Ok(Tier::F),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Tier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tier::S => write!(f, "S"),
            Tier::A => write!(f, "A"),
            Tier::B => write!(f, "B"),
            Tier::C => write!(f, "C"),
            Tier::D => write!(f, "D"),
            Tier::E => write!(f, "E"),
            Tier::F => write!(f, "F"),
        }
    }
}
