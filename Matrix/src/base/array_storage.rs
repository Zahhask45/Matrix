use std::fmt::{self, Debug, Formatter};
use std::ops::Mul;
use std::mem;

use crate::base::Storage;
use crate::base::dimension::Const;
use crate::base::storage::{IsContiguous, RawStorage};



#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ArrayStorage<T, const R: usize, const C: usize>(pub [[T; R]; C]);

impl<T, const R: usize, const C: usize> ArrayStorage<T, R, C> {
	/// Convert this array storage to a slice.
	#[inline]
	pub fn as_slice(&self) -> &[T] {
		// SAFETY: this is OK because ArrayStorage is contiguous.
		unsafe {self.as_slice_unchecked()}
	}

	/// Convert this array storage to a mutable slice.
	#[inline]
	pub fn as_mut_slice(&mut self) -> &mut [T] {
		// SAFETY: this is OK because ArrayStorage is contiguous.
		unsafe {self.as_mut_slice_unchecked()}
	}
}



impl<T: Default, const R: usize, const C: usize> Default for ArrayStorage<T, R, C>
where
	[[T; R]; C]: Default,
{
	#[inline]
	fn default() -> Self {
		Self(Default::default())
	}
}

impl<T: Debug, const R: usize, const C: usize> Debug for ArrayStorage<T, R, C>{
	#[inline]
	fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
		self.0.fmt(fmt)
	}
}

unsafe impl<T, const R: usize, const C: usize> RawStorage<T, Const<R>, Const<C>>
	for ArrayStorage<T, R, C>
{
	type RStride = Const<1>;
	type CStride = Const<R>;

	#[inline]
	fn ptr(&self) -> *const T {
		self.0.as_ptr() as *const T
	}

	#[inline]
	fn shape(&self) -> (Const<R>, Const<C>) {
		(Const, Const)
	}

	#[inline]
	fn strides(&self) -> (Self::RStride, Self::CStride) {
		(Const, Const)
	}

	#[inline]
	fn is_contiguous(&self) -> bool {
		true
	}

	#[inline]
	unsafe fn as_slice_unchecked(&self) -> &[T]{
		unsafe { std::slice::from_raw_parts(self.ptr(), R * C) }
	}
}

unsafe impl<T, const R: usize, const C: usize> IsContiguous for ArrayStorage<T, R, C> {}
