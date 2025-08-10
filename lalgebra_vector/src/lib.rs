use lalgebra_scalar::Scalar;
use std::ops::{Add, Mul};
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Vector<T: Scalar>(pub Vec<T>);


impl<T: Scalar + Add<Output=T> + Copy> Add for Vector<T> {
    type Output=Option<Self>;
    fn add(self, other:Self) ->Self::Output {
        if self.0.len() != other.0.len() {
               return None;
       }
        let res = self.0.iter().zip(other.0.iter()).map(|(a , b)| a.clone() + b.clone()).collect();
        Some(Vector(res))
    }
}

impl<T: Scalar + Add<Output=T> + Mul<Output = T>  > Vector<T> {
	pub fn new() -> Self {  
        Self(
            Vec::new()
        )
	}

	pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len(){
            return None;
        }
        let mut dot_product=T::zero();
        for i in 0..self.0.len(){
            dot_product = dot_product + self.0[i] * other.0[i];
        }
        Some(dot_product)
	}
}