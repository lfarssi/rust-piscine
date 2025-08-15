use std::ops::Mul;
#[derive(Debug,Clone, PartialEq, Eq)]
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
    pub fn number_of_cols(&self) -> usize {
        self.0[0].len()
	}

	pub fn number_of_rows(&self) -> usize {
        self.0.len()
	}

	pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
	}
    
	pub fn col(&self, n: usize) -> Vec<T> {    
        let mut res= Vec::new();
        for i in 0..self.0.len(){
                res.push(self.0[i][n]);
        }
        res
	}
}
impl<T: Scalar +Clone + Mul<Output=T>> Mul for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn mul(self, other: Matrix<T>) -> Self::Output {
        if self.number_of_rows() != other.number_of_rows()
            || self.number_of_cols() != self.number_of_cols()
        {
            return None;
        }
        let mut res = vec![vec![T::zero();self.number_of_cols()]; self.number_of_rows()];
        for i in 0..self.number_of_rows() {
            for j in 0..self.number_of_cols() {
                let mut sum =T::zero();
                for k in 0..self.number_of_cols(){
                    sum = sum + self.0[i][k] * other.0[k][j];
                }
                res[i][j]= sum;
            }
        }
        Some(Matrix(res))
    }
}