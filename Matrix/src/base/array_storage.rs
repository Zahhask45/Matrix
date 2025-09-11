use std::ops::{Add, Sub, Mul};

#[derive(Copy, Clone)]
pub struct ArrayStorage<T, const R: usize, const C: usize>(pub [[T; R]; C]);

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
