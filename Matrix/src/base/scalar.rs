use std::fmt::Debug;
use std::ops::{Add, Mul, Sub};

pub trait Scalar: 'static + Clone + PartialEq + Debug {}

impl<T: 'static + Clone + PartialEq + Debug> Scalar for T {}

pub trait LinearScalar: Scalar + Default + Copy + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self>{
	fn fma(self, y: Self, z: Self) -> Self;
}

impl LinearScalar for f32 {
	fn fma(self, y: Self, z: Self) -> Self{
		self.mul_add(y, z)
	}
}

