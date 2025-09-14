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
