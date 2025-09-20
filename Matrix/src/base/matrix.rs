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

impl<T, R, C, S> Matrix<T, R, C, S> {
	/// # Safety
	#[inline(always)]
	pub const unsafe fn from_data_statically_unchecked(data: S) -> Matrix<T, R, C, S> {
		Matrix {
			data,
			_phantoms: PhantomData,
		}
	}
}
