use std::any::Any;
use std::fmt::Debug;
use std::mem::MaybeUninit;

use crate::StorageMut;
use crate::base::dimension::{Dim, U1};
use crate::base::constraint::{SameNumberOfRows, SameNumberOfColumns, ShapeConstrait};
use crate::base::Scalar;
use crate::storage::{IsContiguous, RawStorageMut};

pub trait Allocator<R: Dim, C: Dim = U1>: Any + Sized {
	/// The type of buffer this allocator can instantiate.
	type Buffer<T: Scalar>: StorageMut<T, R, C> + IsContiguous + Clone + Debug;
	/// The type of buffer uninitialized components  this allocator can instantiate.
	type BufferUninit<T: Scalar>: RawStorageMut<MaybeUninit<T>, R, C> + IsContiguous;

	/// Allocate a buffer with the given number of rows and columns without initializing its content.
	fn allocate_uninit<T: Scalar>(nrows: R, ncols: C) -> Self::BufferUninit<T>;

	/// Assumes a data buffer to be initialized.
	///
	/// # Safety
	/// The user must sure that every single entry of the buffer has been initialized,
	/// or Undefined Behavior will immediately occur.
	unsafe fn assume_init<T: Scalar>(unitit: Self::BufferUninit<T>) -> Self::Buffer<T>;

}


pub trait SameShapeAllocator<R1, C1, R2, C2>:
	Allocator<R1, C1> + Allocator<SameShapeR<R1, R2>, SameShapeC<C1, C2>>
where
	R1: Dim,
	R2: Dim,
	C1: Dim,
	C2: Dim,
	ShapeConstrait: SameNumberOfRows<R1, R2> + SameNumberOfColumns<C1, C2>,
{}
