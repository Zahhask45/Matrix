use crate::base::dimension::{Dim, DimName, Dyn, U1};
use crate::base::{Scalar, Vector};

/// A Vec-based matrix data storage. It may be dynamically-sized.
#[derive(Eq, Debug, Clone, PartialEq)]
pub struct VecStorage<T, R: Dim, C: Dim> {
	data: Vec<T>,
	nrows: R,
	ncols: C,
}

impl<T> Default for VecStorage<T, Dyn, Dyn> {
	fn default() -> Self {
		Self{
			data: Vec::new(),
			nrows: Dyn::from_usize(0),
			ncols: Dyn::from_usize(0),
		}
	}
}

impl<T, C: DimName> Default for VecStorage<T, Dyn, C> {
	fn default() -> Self {
		Self {
			data: Vec::new(),
			nrows: Dyn::from_usize(0),
			ncols: C::name(),
		}
	}
}

impl <T, R: Dim, C: Dim> VecStorage<T, R, C> {
	#[inline]
	pub fn new(nrows: R, ncols: C, data: Vec<T>) -> Self {
		assert!(nrows.value() * ncols.value() == data.len(),
		"Data storage buffer dimension mismatch.");

		Self {data, nrows, ncols}
	}

	#[inline]
	#[must_use]
	pub const fn as_vec(&self) -> &Vec<T> {
		&self.data
	}

	#[inline]
	#[must_use]
	pub const fn len(&self) -> usize {
		self.data.len()
	}

	#[inline]
	#[must_use]
	pub const fn is_empty(&self) -> bool {
		self.data.len() == 0
	}

	#[inline]
	pub fn as_slice(&self) -> &[T] {
		&self.data[..]
	}
}
