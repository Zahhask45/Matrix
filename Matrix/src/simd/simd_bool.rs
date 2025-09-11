use crate::simd::SimdValue;
use std::ops::{BitAnd, BitOr, BitXor, Not};

pub trait SimdBool:
	Copy
	+ BitAnd<Self, Output = Self>
	+ BitOr<Self, Output = Self>
	+ BitXor<Self, Output = Self>
	+ Not<Output = Self>
{
	fn bitmask(self) -> u64;
	fn and(self) -> bool;
	fn or(self) -> bool;
	fn xor(self) -> bool;
	fn all(self) -> bool;
	fn any(self) -> bool;
	fn none(self) -> bool;
}

impl SimdBool for bool {
	#[inline(always)]
	fn bitmask(self) -> u64{
		self as u64
	}

	#[inline(always)]
	fn and(self) -> bool {
		self
	}
	
	#[inline(always)]
	fn or(self) -> bool {
		self
	}
	
	#[inline(always)]
	fn xor(self) -> bool {
		self
	}
	
	#[inline(always)]
	fn all(self) -> bool {
		self
	}
	
	#[inline(always)]
	fn any(self) -> bool {
		self
	}
	
	#[inline(always)]
	fn none(self) -> bool {
		!self
	}
}
