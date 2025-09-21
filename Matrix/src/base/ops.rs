use std::ops::{
	Add, AddAssign, Sub, SubAssign, Mul, MulAssign
};

use crate::base::dimension::{Dim, Dyn};

macro_rules! componentwise_binop_impl(
	($Trait: ident, $method: ident, $bound: ident;
	 $TraitAssign: ident, $method_assign: ident, $method_assign_statically_unchecked: ident,
	 $method_assign_statically_unchecked_rhs: ident;
	 $method_to: ident) => {

	 	impl<'b, T, R1, C1, R2, C2, SA, SB> $Trait<&'b Matrix<T, R2, C2, SB>> for Matrix<T, R1, C1, SA>
	 		where R1: Dim, C1: Dim, R2: Dim, C2: Dim,
	 			T: Scalar + $bound,
	 			SA: Storage<T, R1, C1>,
	 			SB: Storage<T, R2, C2>,
	 			DefaultAllocator: SameShapeAllocator<R1, C1, R2, C2>,
	 			ShapeConstraint: SameNumberOfRows<R1, R2> + SameNumberOfColumns<C1, C2> {
	 		type Output = MatrixSum<T, R1, C1, R2, C2>;	
	 	}
	 }
);

componentwise_binop_impl!(Add, add,
							AddAssign, add_assign);
