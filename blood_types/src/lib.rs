#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

use std::cmp::Ord;

use std::str::FromStr;

impl FromStr for BloodType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (antigen_str, rh_char) = if s.starts_with("AB") {
            if s.len() != 3 {
                return Err("Invalid AB blood type format".into());
            }
            ("AB", s.chars().nth(2).unwrap())
        } else {
            (&s[0..1], s.chars().nth(1).unwrap())
        };
        let antigen = match antigen_str {
            "A" => Antigen::A,
            "B" => Antigen::B,
            "O" => Antigen::O,
            "AB" => Antigen::AB,
            _ => return Err(format!("Invalid antigen: {}", antigen_str)),
        };
        let rh_factor = match rh_char {
            '+' => RhFactor::Positive,
            '-' => RhFactor::Negative,
            _ => return Err(format!("Invalid Rh factor: {}", rh_char)),
        };
        Ok(BloodType { antigen, rh_factor })
    }
}

use std::fmt::{self, Debug, Formatter};

impl Debug for BloodType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let antigen_str = match self.antigen {
            Antigen::A => "A",
            Antigen::B => "B",
            Antigen::AB => "AB",
            Antigen::O => "O",
        };

        let rh_str = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };

        write!(f, "{}{}", antigen_str, rh_str)
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        let antigen_compatible = match self.antigen {
            Antigen::O => other.antigen == Antigen::O,
            Antigen::A => other.antigen == Antigen::A || other.antigen == Antigen::O,
            Antigen::B => other.antigen == Antigen::B || other.antigen == Antigen::O,
            Antigen::AB => true,
        };

        let rh_compatible = match self.rh_factor {
            RhFactor::Positive => true,
            RhFactor::Negative => other.rh_factor == RhFactor::Negative,
        };

        antigen_compatible && rh_compatible
    }

    pub fn donors(&self) -> Vec<Self> {
        let mut res: Vec<Self> = Vec::new();
        let blood = ["A+", "A-", "O+", "O-", "B+", "B-", "AB+", "AB-"];
        for i in blood {
            let peepee: BloodType = i.parse::<BloodType>().unwrap();
            if self.can_receive_from(&peepee) {
                res.push(peepee);
            }
        }
        res
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        let mut res: Vec<Self> = Vec::new();
        let blood = ["A+", "A-", "O+", "O-", "B+", "B-", "AB+", "AB-"];
        for i in blood {
            let peepee: BloodType = i.parse::<BloodType>().unwrap();
            if peepee.can_receive_from(&self) {
                res.push(peepee);
            }
        }
        res
    }
}
