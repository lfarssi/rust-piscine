use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(n: u32) -> Self {
        match n {
            0 => Nulla,
            1 => I,
            5 => V,
            10 => X,
            50 => L,
            100 => C,
            500 => D,
            1000 => M,
            _ => panic!("invalid roman digit {}", n),
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(mut n: u32) -> Self {
        if n == 0 {
            return RomanNumber(vec![Nulla]);
        }
        let mut res = Vec::new();
        let table = [
            (1000, vec![M]),
            (900, vec![C, M]),
            (500, vec![D]),
            (400, vec![C, D]),
            (100, vec![C]),
            (90, vec![L, C]),
            (50, vec![L]),
            (40, vec![X, L]),
            (10, vec![X]),
            (9, vec![V, X]),
            (5, vec![V]),
            (4, vec![I, V]),
            (1, vec![I]),
        ];
        for (v, digits) in table.iter() {
                res.extend_from_slice(&digits.repeat((n/ *v ) as usize));
                n %= *v;
            
        }
        RomanNumber(res)
    }
}
