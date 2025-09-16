use std::ptr;

use crate::base::Scalar;
use crate::base::dimension::{Dim, U1};

/// # Safety
pub unsafe trait RawStorage<T, R: Dim, C: Dim = U1>: Sized {
	type RStride: Dim;

	type CStride: Dim;

	fn ptr(&self) -> *const T;

	fn shape(&self) -> (R, C);

	fn strides(&self) -> (Self::RStride, Self::CStride);
}


// pub unsafe trait Storage<T: Scalar, R: Dim, C: Dim = U1>: RawStorage<T, R, C> {
// 	fn into_owned(self) -> Owned<T, R, C>
// 	where
// 		DefaultAllocator: Allocator<R, C>
// }
