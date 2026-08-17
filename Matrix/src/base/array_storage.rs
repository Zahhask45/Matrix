use std::fmt::{self, Debug, Formatter};
use std::ops::Add;
use std::slice;



#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ArrayStorage<T, const R: usize, const C: usize>(pub [[T; R]; C]);

impl<T, const R: usize, const C: usize> ArrayStorage<T, R, C> {
	/// # Safety
	/// The backing storage is contiguous in memory.
	#[inline]
	pub unsafe fn as_slice_unchecked(&self) -> &[T] {
		unsafe { slice::from_raw_parts(self.0.as_ptr() as *const T, R * C) }
	}

	/// # Safety
	/// The backing storage is contiguous in memory.
	#[inline]
	pub unsafe fn as_mut_slice_unchecked(&mut self) -> &mut [T] {
		unsafe { slice::from_raw_parts_mut(self.0.as_mut_ptr() as *mut T, R * C) }
	}

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

impl<T, const R: usize, const C: usize> Add for ArrayStorage<T, R, C>
where
	T: Add<Output = T> + Copy,
{
	type Output = Self;

	fn add (self, other: Self) -> Self::Output {
		let mut result = self;

		for i in 0..C {
			for j in 0..R {
				result.0[i][j] = self.0[i][j] + other.0[i][j];
			}
		}

		result
	}
}
