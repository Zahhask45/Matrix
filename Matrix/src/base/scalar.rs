use std::fmt::Debug;
use std::ops::{Add, Mul, Sub, Div};

pub trait Scalar: 'static + Clone + PartialEq + PartialOrd + Debug {
	fn zero() -> Self;
	fn one() -> Self;
}


impl Scalar for f32 {
	fn zero() -> Self{ 0.0}
	fn one() -> Self{ 1.0}
}

pub trait LinearScalar: Scalar + Default + Copy + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self>
where Self::Real: LinearScalar,
{
	type Real;

	fn fma(self, y: Self, z: Self) -> Self;
	fn conj(self) -> Self;
	fn sqrt(self) -> Self;
	fn abs(self) -> Self::Real;
	fn re(self) -> Self::Real;
}

impl LinearScalar for f32 {
	type Real = f32;
	
	fn fma(self, y: Self, z: Self) -> Self{
		self.mul_add(y, z)
	}
	fn conj(self) -> Self {
		self
	}
	fn sqrt(self) -> Self {
		if self.is_nan(){
			return f32::NAN;
		}
		
		if self < 0.0 {
			return f32::NAN;
		}
		
		if self.is_infinite() {
	        return f32::INFINITY;
	    }
	    
		if self == 0.0 {
			return 0.0;
		}

		let mut x  = self;

		loop {
			let next = (x + self / x) / 2.0;

			if next == x {
				return x;
			}

			x = next
		}
	}
	fn abs(self) -> Self::Real {
		if self < 0.0{
			return -self;
		}
		self
	}

	fn re(self) -> Self::Real {
		self
	}
}

