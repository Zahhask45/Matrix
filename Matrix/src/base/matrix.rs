use crate::base::{Scalar};

use std::marker::PhantomData;
use std::ops::{Add, Sub, Mul, AddAssign, SubAssign};

pub struct Matrix<T, R, C, S>{
	pub data: S,
	_phatomns: PhantomData<(T, R, C)>,
}

impl<T, R: Dim, C: Dim, S: fmt::Debug> fmt::Debug for Matrix<T, R, C, S>{
	
}

impl<T, R, C, S> Add for Matrix<T, R, C, S>
where
	S: Add<Output = S>,
{
	type Output = Self;

	fn add(self, other: Self) -> Self::Output{
		Self{
			data: self.data + other.data,
			_phatomns: PhantomData,
		}
	}
}

impl<T, R, C, S> AddAssign for Matrix<T, R, C, S>
where
	S: AddAssign,
{
	fn add_assign(&mut self, rhs: Self) {
		self.data += rhs.data;
	}
}

impl<T, R, C, S> Sub for Matrix<T, R, C, S>
where
	S: Sub<Output = S>,
{
	type Output = Self;

	fn sub(self, other: Self) -> Self::Output{
		Self{
			data: self.data - other.data,
			_phatomns: PhantomData,
		}
	}
}

impl<T, R, C, S> SubAssign for Matrix<T, R, C, S>
where
	S: SubAssign,
{
	fn sub_assign(&mut self, rhs: Self) {
		self.data -= rhs.data;
	}
}

impl<T, R, C, S> Mul<T> for Matrix<T, R, C, S>
where
	S: Mul<T, Output = S>,
	T: Copy,
{
	type Output = Self;

	fn mul(self, rhs: T) -> Self::Output {
		Self {
			data: self.data * rhs,
			_phatomns: PhantomData,
		}
	}
}
