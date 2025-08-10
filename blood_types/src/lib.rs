use std::fmt::{Formatter, Result as Res};
#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
	A,
	AB,
	B,
	O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum RhFactor {
	Positive,
	Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
	antigen: Antigen,
	rh_factor: RhFactor,
}

use std::cmp::{Ord};

use std::str::FromStr;



impl FromStr for BloodType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars:Vec<char>=s.chars().collect();
        if chars.len()==3 {
                    match chars[2] {
                        '+'=> Ok(Self { antigen: Antigen::AB, rh_factor: RhFactor::Positive }),
                        '-'=> Ok(Self { antigen: Antigen::AB, rh_factor: RhFactor::Negative }),
                        _=> Err("slm cv".to_string()),
                    }
        } else {
            let antigens= chars[0];
            let rh_factors= chars[1];
            match antigens {
                'A' => {
                    match rh_factors {
                        '+'=> Ok(Self { antigen: Antigen::A, rh_factor: RhFactor::Positive }),
                        '-'=> Ok(Self { antigen: Antigen::A, rh_factor: RhFactor::Negative }),
                        _=> Err("slm cv".to_string()),
                    }
                },
                'B' => {
                    match rh_factors {
                        '+'=> Ok(Self { antigen: Antigen::B, rh_factor: RhFactor::Positive }),
                        '-'=> Ok(Self { antigen: Antigen::B, rh_factor: RhFactor::Negative }),
                        _=> Err("slm cv".to_string()),
                    }
                },
                'O' => {
                    match rh_factors {
                        '+'=> Ok(Self { antigen: Antigen::O, rh_factor: RhFactor::Positive }),
                        '-'=> Ok(Self { antigen: Antigen::O, rh_factor: RhFactor::Negative }),
                        _=> Err("slm cv".to_string()),
                    }
                },
                _=> Err("slm cv".to_string())
            }
        }
    }
}

use std::fmt::{Debug};

impl Debug for BloodType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Res {
        let antigen = match self.antigen{
            Antigen::AB => "AB",
            Antigen::A => "A",
            Antigen::B => "B",
            Antigen::O => "O",
        };
        let rh_factor = match self.rh_factor{
            RhFactor::Negative =>"-",
            RhFactor::Positive =>"+",
        };
        write!(f,"{}{}", antigen, rh_factor)
    }
}

impl BloodType {
	pub fn can_receive_from(&self, other: &Self) -> bool {
        let anti = match self.antigen {
               Antigen::O=> other.antigen == Antigen::O,  
               Antigen::A=> other.antigen == Antigen::A || other.antigen == Antigen::O,  
               Antigen::B=> other.antigen == Antigen::B || other.antigen == Antigen::O,  
               Antigen::AB=> true,  
        };
        let rh_fac = match self.rh_factor {
            RhFactor::Negative =>other.rh_factor==RhFactor::Negative,
            RhFactor::Positive =>true,
        };
        anti && rh_fac
	}

	pub fn donors(&self) -> Vec<Self> {
        let mut res :Vec<Self>=Vec::new();
        let bloods=["A+","A-","O+","O-","B+","B-","AB+","AB-"];
        for blood in bloods{
            let type_blood: BloodType=blood.parse().unwrap();
            if self.can_receive_from(&type_blood){
                res.push(type_blood);
            }
        }
        res
	}

	pub fn recipients(&self) -> Vec<BloodType> {
        let mut res :Vec<Self>=Vec::new();
        let bloods=["A+","A-","O+","O-","B+","B-","AB+","AB-"];
        for blood in bloods{
            let type_blood: BloodType=blood.parse().unwrap();
            if type_blood.can_receive_from(&self){
                res.push(type_blood);
            }
        }
        res
}
}