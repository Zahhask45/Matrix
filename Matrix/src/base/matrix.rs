use std::fmt;
use std::marker::PhantomData;
use std::ops::{Add, Sub, Mul, AddAssign, SubAssign};

use crate::base::dimension::{Dim, U1};
use crate::base::{Scalar};
use crate::{ArrayStorage};

pub type Vector<T, D, S> = Matrix<T, D, U1, S>;

#[derive(Clone, Copy)]
pub struct Matrix<T, R, C, S>{
	pub data: S,
	_phantoms: PhantomData<(T, R, C)>,
}

impl<T, R: Dim, C: Dim, S: fmt::Debug> fmt::Debug for Matrix<T, R, C, S>{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
		self.data.fmt(f)
	}
}

impl<T, R, C, S> Default for Matrix<T, R, C, S>
where
	T: Scalar,
	R: Dim,
	C: Dim,
	S: Default,
{
	fn default() -> Self {
		Matrix {
			data: Default::default(),
			_phantoms: PhantomData, 
		}
	}
}

impl<T, R, C, S> Add for Matrix<T, R, C, S>
where
	S: Add<Output = S>,
{
	type Output = Self;

	fn add(self, other: Self) -> Self::Output{
		Self{
			data: self.data + other.data,
			_phantoms: PhantomData,
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
			_phantoms: PhantomData,
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
			_phantoms: PhantomData,
		}
	}
}
