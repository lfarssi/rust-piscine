
use std::ops::{Add, Sub, Mul, Div};

pub trait Scalar: Add<Output=Self::Item> + Sub<Output=Self::Item> + Mul<Output=Self::Item> +Div<Output=Self::Item> + Copy + Clone {
	type Item;
	fn zero() -> Self::Item;
	fn one() -> Self::Item;
}

impl Scalar for u32{
	type Item = u32;
 fn one() -> Self::Item {
	 1
 }
 fn zero() -> Self::Item {
	 0
 }
}

impl Scalar for i32{
	type Item = i32;
 fn one() -> Self::Item {
	 1
 }
 fn zero() -> Self::Item {
	 0
 }
}
impl Scalar for u64{
	type Item = u64;
 fn one() -> Self::Item {
	 1
 }
 fn zero() -> Self::Item {
	 0
 }
}
impl Scalar for i64{
	type Item = i64;
 fn one() -> Self::Item {
	 1
 }
 fn zero() -> Self::Item {
	 0
 }
}


impl Scalar for f64{
	type Item = f64;
 fn one() -> Self::Item {
	 1.0
 }
 fn zero() -> Self::Item {
	 0.0
 }
}

impl Scalar for f32{
	type Item = f32;
 fn one() -> Self::Item {
	 1.0
 }
 fn zero() -> Self::Item {
	 0.0
 }
}