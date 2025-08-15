#[derive(Debug)]
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