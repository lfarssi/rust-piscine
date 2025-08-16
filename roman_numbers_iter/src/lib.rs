use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RomanDigit {
    I = 1,
    V = 5,
    X = 10,
    L = 50,
    C = 100,
    D = 500,
    M = 1000,
}

#[derive(Clone, PartialEq)]
pub struct RomanNumber(pub Vec<RomanDigit>, u32);

impl fmt::Debug for RomanNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f,"RomanNumber({:?})",self.0)
    }
}

impl From<u32> for RomanNumber {
    fn from(n: u32) -> Self {
        let digits = to_roman_digits(n);
        RomanNumber(digits, n)
    }
}

impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        self.1 += 1;  
        let digits = to_roman_digits(self.1);
        self.0 = digits.clone(); 
        Some(RomanNumber(digits, self.1))
    }
}

fn to_roman_digits(mut n: u32) -> Vec<RomanDigit> {
    let mut result = Vec::new();
    
    let values_and_representations = [
        (1000, vec![RomanDigit::M]),
        (900, vec![RomanDigit::C, RomanDigit::M]),
        (500, vec![RomanDigit::D]),
        (400, vec![RomanDigit::C, RomanDigit::D]),
        (100, vec![RomanDigit::C]),
        (90, vec![RomanDigit::X, RomanDigit::C]),
        (50, vec![RomanDigit::L]),
        (40, vec![RomanDigit::X, RomanDigit::L]),
        (10, vec![RomanDigit::X]),
        (9, vec![RomanDigit::I, RomanDigit::X]),
        (5, vec![RomanDigit::V]),
        (4, vec![RomanDigit::I, RomanDigit::V]),
        (1, vec![RomanDigit::I]),
    ];

    for &(value, ref digits) in values_and_representations.iter() {
        while n >= value {
            n -= value;
            result.extend_from_slice(digits);
        }
    }

    result
}