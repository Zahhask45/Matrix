use std::any::{Any, TypeId};
use std::fmt::Debug;
use std::cmp;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Dyn(pub usize);

pub unsafe trait Dim: Any + Debug + Copy + PartialEq + Send + Sync {
	#[inline(always)]
	fn is<D: Dim>() -> bool {
		TypeId::of::<Self>() == TypeId::of::<D>()
	}

	fn try_to_usize() ->Option<usize>;

	fn value(&self) -> usize;

	fn from_usize(dim: usize) -> Self;
}

unsafe impl Dim for Dyn {
	#[inline]
	fn try_to_usize() -> Option<usize>{
		None
	}

	#[inline]
	fn from_usize(dim: usize) -> Self{
		Self(dim)
	}
	
	#[inline]
	fn value(&self) -> usize{
		self.0
	}
}

