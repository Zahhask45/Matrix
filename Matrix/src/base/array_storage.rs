use std::fmt::{self, Debug, Formatter};
use std::ops::{Add, Sub, Mul};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ArrayStorage<T, const R: usize, const C: usize>(pub [[T; R]; C]);

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

	fn add(self, rhs: Self) -> Self::Output {
		let mut result = self;

		for i in 0..C{
			for j in 0..R{
				result.0[i][j] = self.0[i][j] + rhs.0[i][j];
			}
		}
		ArrayStorage(result.0)
	}
}
