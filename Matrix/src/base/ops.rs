use std::ops::{
	Add, AddAssign, Sub, SubAssign, Mul, MulAssign
};

use crate::base::allocator::{SameShapeAllocator};
use crate::base::constraints::{SameNumberOfColumns, SameNumberOfRows, ShapeConstraint};
use crate::base::storage::{Storage};
use crate::base::dimension::{Dim, Dyn};
use crate::base::{DefaultAllocator, Matrix, Scalar};

macro_rules! componentwise_binop_impl(
	($Trait: ident, $method: ident;
	 $TraitAssign: ident, $method_assign: ident, $method_assign_statically_unchecked: ident,
	 $method_assign_statically_unchecked_rhs: ident;
	 $method_to: ident, $method_to_statically_uncheked_uninit: ident) => {

	 	impl<'b, T, R1, C1, R2, C2, SA, SB> $Trait<&'b Matrix<T, R2, C2, SB>> for Matrix<T, R1, C1, SA>
	 		where R1: Dim, C1: Dim, R2: Dim, C2: Dim,
	 			T: Scalar,
	 			SA: Storage<T, R1, C1>,
	 			SB: Storage<T, R2, C2>,
	 			DefaultAllocator: SameShapeAllocator<R1, C1, R2, C2>,
	 			ShapeConstraint: SameNumberOfRows<R1, R2> + SameNumberOfColumns<C1, C2> {
	 		type Output = MatrixSum<T, R1, C1, R2, C2>;	

	 		#[inline]
	 		fn $method(self, rhs: &'b Matrix<T, R2, C2, SB>) -> Self::Output {
	 			assert_eq!(self.shape(), rhs.shape(), "Matrix add/sub dimension mismatch");
	 			let mut res = self.into_owned_sum::<R2, C2>();
	 			res.$method_assign_statically_unchecked_rhs(rhs);
	 			res
	 		}
	 	}
	 }
);

componentwise_binop_impl!(Add, add;
						  AddAssign, add_assign, add_assign_statically_unchecked, add_assign_statically_unchecked_mut;
						  add_to, add_to_statically_unchecked_ininit);
