use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);
use lalgebra_scalar::Scalar;

impl <T: Scalar> Matrix<T> {
	pub fn new() -> Matrix<T> {
        Matrix(vec![Vec::new()])
	}

	pub fn zero(row: usize, col: usize) -> Matrix<T> {

        Matrix(vec![vec![T::zero();col];row])
	}

	pub fn identity(n: usize) -> Matrix<T> {
        let mut res : Vec<Vec<T>> = vec![Vec::new();n];
        for i in 0..n {
            for j in 0..n{
                if i == j{

                    res[i].push(T::one());
                } else{
                    res[i].push(T::zero());
                }
            }
        }
        Matrix(res)
	}
}
impl<T: Clone + Add<Output=T>> Add for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn add(self, other: Matrix<T>) -> Self::Output {
        if self.0.len() != other.0.len()
            || self.0.iter().zip(&other.0).any(|(a, b)| a.len() != b.len())
        {
            return None;
        }
        let mut res = vec![Vec::new(); self.0.len()];
        for i in 0..self.0.len() {
            for j in 0..self.0[i].len() {
                res[i].push(self.0[i][j].clone() + other.0[i][j].clone());
            }
        }
        Some(Matrix(res))
    }
}

impl<T: Clone+ Sub<Output=T>> Sub for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn sub(self, other: Matrix<T>) -> Self::Output {
        if self.0.len() != other.0.len()
            || self.0.iter().zip(&other.0).any(|(a, b)| a.len() != b.len())
        {
            return None;
        }
        let mut res = vec![Vec::new(); self.0.len()];
        for i in 0..self.0.len() {
            for j in 0..self.0[i].len() {
                res[i].push(self.0[i][j].clone() - other.0[i][j].clone());
            }
        }
        Some(Matrix(res))
    }
}
