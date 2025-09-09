use std::marker::PhantomData;
use std::ops::{Add, Sub, AddAssign, SubAssign};

pub struct Vector<T, R, C, S>{
	pub data: S,
	_phatomns: PhantomData<(T, R, C)>,
}

impl<T, R, C, S> Add for Vector<T, R, C, S>
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

impl<T, R, C, S> AddAssign for Vector<T, R, C, S>
where
	S: AddAssign,
{
	fn add_assign(&mut self, rhs: Self) {
		self.data += rhs.data;
	}
}

impl<T, R, C, S> Sub for Vector<T, R, C, S>
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

impl<T, R, C, S> SubAssign for Vector<T, R, C, S>
where
	S: SubAssign,
{
	fn sub_assign(&mut self, rhs: Self) {
		self.data -= rhs.data;
	}
}
