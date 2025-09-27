use std::cmp;
use std::ptr;
use std::mem::ManuallyDrop;
use std::mem::MaybeUninit;

//same as use crate::base::Const
use super::Const;
use crate::base::Scalar;
use crate::base::allocator::Allocator;
use crate::base::array_storage::ArrayStorage;

#[derive(Copy, Clone, Debug)]
pub struct DefaultAllocator;


// Static - Static
impl<const R: usize, const C: usize> Allocator<Const<R>, Const<C>> for DefaultAllocator {
	type Buffer<T: Scalar> = ArrayStorage<T, R, C>;
	type BufferUninit<T: Scalar> = ArrayStorage<MaybeUninit<T>, R, C>;

	#[inline(always)]
	fn allocate_uninit<T: Scalar>(_: Const<R>, _: Const<C>) -> ArrayStorage<MaybeUninit<T>, R, C>{
		// SAFETY: Uninitialized `[MaybeUninit<_>; _]` is valid.
		let array: [[MaybeUninit<T>; R]; C] = unsafe {MaybeUninit::uninit().assume_init()};
		ArrayStorage(array)
	}

	#[inline(always)]
	unsafe fn assume_init<T: Scalar>(
		uninit: ArrayStorage<MaybeUninit<T>, R, C>,
	) -> ArrayStorage<T, R, C> {
		unsafe {
			// Safety:
			// * Caller guarantees that all elements of the array are initialized
			// * `MaybeUninit<T>` and T are guaranteed to have the same layout
			// * `MaybeUninit` does not drop, so there are no double-droops
			// And thus the conversion is safe
			ArrayStorage((&uninit as *const _ as *const[_; C]).read())
		}
	}
}
