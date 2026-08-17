use std::ops::{Add, Sub, Mul, Neg, Div};

use crate::base::{LinearScalar, Scalar};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Complex<T> {
	pub re: T,
	pub im: T,
}

impl<T> Complex<T> {
	#[inline]
	pub const fn new(re: T, im: T) -> Self {
		Complex {re, im}
	}
}

impl<T> Scalar for Complex<T>
where T: Scalar,
{
	fn zero() -> Self{ Self::new(T::zero(),T::zero()) }
	fn one() -> Self{ Self::new(T::one(), T::zero()) }
}


impl<T> Default for Complex<T>
where T: Default
{
	fn default() -> Self {
		Self {
			re: T::default(),
			im: T::default(),
		}
	}
}



impl<T> Add for Complex<T>
where T: Copy + Add<Output = T>,
{
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output{
		Self::new(self.re + rhs.re, self.im + rhs.im)
	}
}

impl<T> Sub for Complex<T>
where T: Copy + Sub<Output = T>,
{
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output{
		Self::new(self.re - rhs.re, self.im - rhs.im)
	}
}

impl<T> Mul for Complex<T>
where T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
	type Output = Self;
	fn mul(self, rhs: Self) -> Self::Output{
		Self::new(
			self.re * rhs.re - self.im * rhs.im, 
			self.re * rhs.im + self.im * rhs.re
		)
	}
}

impl<T: Mul<f32, Output = T>> Mul<f32> for Complex<T> {
	type Output = Self;
	fn mul(self, rhs: f32) -> Self::Output{
		Self::new(
			self.re * rhs, 
			self.im * rhs
		)
	}
}

impl<T> LinearScalar for Complex<T> 
where T: Scalar + Default + Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Neg<Output = T> + Div<Output = T> + LinearScalar,
{
	type Real = T;
	#[inline]
	fn fma(self, y: Self, z: Self) -> Self {
		(self * y) + z
	}

	fn conj(self) -> Self {
		Self::new(self.re, -self.im)
	}

	fn sqrt(self) -> Self {
		let two = T::one() + T::one();
		let r = (self.re * self.re + self.im * self.im).sqrt();
		let re = ((r + self.re) / two).sqrt();

		let mut im = ((r - self.re) / two).sqrt();

		if self.im < T::zero() {
			im = -im;
		}

		Self::new(re, im)
	}

	fn abs(self) -> T {
		(self.re * self.re + self.im * self.im).sqrt()
	}

	fn re(self) -> T {
		self.re
	}
}

impl<T: std::fmt::Display> std::fmt::Display for Complex<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} + {}i", self.re, self.im)
	}
}

