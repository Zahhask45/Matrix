use std::ptr;

use crate::base::Scalar;
use crate::base::allocator::Allocator;
use crate::base::default_allocator::DefaultAllocator;
use crate::base::dimension::{Dim, U1};

/*
 * Aliases for allocation results
 */

pub type SameShapeStorage<T, R1, C1, R2, C2> =
	<DefaultAllocator as Allocator<SameShapeR<R1, R2>, SameShapeC<C1, C2>>>::Buffer<T>;


/// The owned data storage that can be allocated from `S`
pub type Owned<T, R, C = U1> = <DefaultAllocator as Allocator<R, C>>::Buffer<T>;

/// # Safety
pub unsafe trait RawStorage<T, R: Dim, C: Dim = U1>: Sized {
	type RStride: Dim;

	type CStride: Dim;

	fn ptr(&self) -> *const T;

	fn shape(&self) -> (R, C);

	fn strides(&self) -> (Self::RStride, Self::CStride);
}

/// # Safety
pub unsafe trait Storage<T: Scalar, R: Dim, C: Dim = U1>: RawStorage<T, R, C> {
	fn into_owned(self) -> Owned<T, R, C>
	where
		DefaultAllocator: Allocator<R, C>;

	/// Clones this data storage to one that does not contain any reference
	fn clone_owned(&self) -> Owned<T, R, C>
	where
		DefaultAllocator: Allocator<R, C>;

	/// Drops the storage without calling the destructores on the contained elements.
	fn forget_elements(self);
}

/// # Safety
pub unsafe trait RawStorageMut<T, R: Dim, C: Dim = U1>: RawStorage<T, R, C> {
	fn ptr_mut(&mut self) -> *mut T;
}


/// Trait shared by all mutable matrix data storage that don't contain any uninitialized elements.
///
/// # Safety
///
/// See safety note for `Storage`, and `RawStorageMut`
pub unsafe trait StorageMut<T: Scalar, R: Dim, C: Dim = U1>:
	Storage<T, R, C> + RawStorageMut<T, R, C>{}

unsafe impl<S, T: Scalar, R, C> StorageMut<T, R, C> for S
where
	R: Dim,
	C: Dim,
	S: Storage<T, R, C> + RawStorageMut<T, R, C>,
{
}


/// # Safety
pub unsafe trait IsContiguous {}
