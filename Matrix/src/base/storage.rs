use std::ptr;

use crate::base::Scalar;
use crate::base::dimension::{Dim, U1};

pub unsafe trait RawStorage<T, R: Dim, C: Dim = U1>: Sized {
	type RStride: Dim;

	type CStride: Dim;

	fn ptr(&self) -> *const T;

	fn shape(&self) -> (R, C);

	fn strides(&self) -> (Self::RStride, Self::CStride);
}
