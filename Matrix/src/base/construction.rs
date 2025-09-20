use crate::base::{ArrayStorage, Matrix};
use crate::base::dimension::{Const};

macro_rules! transpose_array(
	[$($a: ident),*;] => {
		[$([$a]),*]
	};
	[$($a: ident),*; $($b: ident),*;] => {
		[$([$a, $b]),*]
	};
	[$($a: ident),*; $($b: ident),*; $($c: ident),*;] => {
		[$([$a, $b, $c]),*]
	};
	[$($a: ident),*; $($b: ident),*; $($c: ident),*; $($d: ident),*;] => {
		[$([$a, $b, $c, $d]),*]
	};
	[$($a: ident),*; $($b: ident),*; $($c: ident),*; $($d: ident),*; $($e: ident),*;] => {
		[$([$a, $b, $c, $d, $e]),*]
	};
	[$($a: ident),*; $($b: ident),*; $($c: ident),*; $($d: ident),*; $($e: ident),*; $($f: ident),*;] => {
		[$([$a, $b, $c, $d, $e, $f]),*]
	};
);

macro_rules! componentwise_constructores_impl(
	($($R: expr, $C: expr, [$($($args: ident),*);*] $(;)*)*) => {$(
		impl<T> Matrix<T, Const<$R>, Const<$C>, ArrayStorage<T, $R, $C>> {
			#[inline]
			#[allow(clippy::too_many_arguments)]
			pub const fn new($($($args: T),*),*) -> Self {
				unsafe {
					Self::from_data_statically_unchecked(
						ArrayStorage(
							transpose_array![
								$(
									$($args),*
								;)*
							]
						)
					)
				}
			}
		}
	)*}
);

componentwise_constructores_impl!(
	/*
	 * Square Matrices
	 */
	2, 2, [m11, m12;
			m21, m22];
	3, 3, [m11, m12, m13;
			m21, m22, m23;
			m31, m32, m33];
	4, 4, [m11, m12, m13, m14;
			m21, m22, m23, m24;
			m31, m32, m33, m34;
			m41, m42, m43, m44];
	5, 5, [m11, m12, m13, m14, m15;
			m21, m22, m23, m24, m25;
			m31, m32, m33, m34, m35;
			m41, m42, m43, m44, m45;
			m51, m52, m53, m54, m55];
	6, 6, [m11, m12, m13, m14, m15, m16;
			m21, m22, m23, m24, m25, m26;
			m31, m32, m33, m34, m35, m36;
			m41, m42, m43, m44, m45, m46;
			m51, m52, m53, m54, m55, m56;
			m61, m62, m63, m64, m65, m66];

	/*
	 * Rectangular Matrices with 2 rows
	 */
	2, 3, [m11, m12, m13;
			m21, m22, m23];
	2, 4, [m11, m12, m13, m14;
			m21, m22, m23, m24];
	2, 5, [m11, m12, m13, m14, m15;
			m21, m22, m23, m24, m25];
	2, 6, [m11, m12, m13, m14, m15, m16;
			m21, m22, m23, m24, m25, m26];

	/*
	 * Rectangular Matrices with 3 rows
	 */
	3, 2, [m11, m12;
			m21, m22;
			m31, m32];
	3, 4, [m11, m12, m13, m14;
			m21, m22, m23, m24;
			m31, m32, m33, m34];
	3, 5, [m11, m12, m13, m14, m15;
			m21, m22, m23, m24, m25;
			m31, m32, m33, m34, m35];
	3, 6, [m11, m12, m13, m14, m15, m16;
			m21, m22, m23, m24, m25, m26;
			m31, m32, m33, m34, m35, m36];

	/*
	 * Rectangular Matrices with 4 rows
	 */
	4, 2, [m11, m12;
			m21, m22;
			m31, m32;
			m41, m42];
	4, 3, [m11, m12, m13;
			m21, m22, m23;
			m31, m32, m33;
			m41, m42, m43];
	4, 5, [m11, m12, m13, m14, m15;
			m21, m22, m23, m24, m25;
			m31, m32, m33, m34, m35;
			m41, m42, m43, m44, m45];
	4, 6, [m11, m12, m13, m14, m15, m16;
			m21, m22, m23, m24, m25, m26;
			m31, m32, m33, m34, m35, m36;
			m41, m42, m43, m44, m45, m46];

	/*
	 * Rectangular Matrices with 5 rows
	 */
	5, 2, [m11, m12;
			m21, m22;
			m31, m32;
			m41, m42;
			m51, m52];
	5, 3, [m11, m12, m13;
			m21, m22, m23;
			m31, m32, m33;
			m41, m42, m43;
			m51, m52, m53];
	5, 4, [m11, m12, m13, m14;
			m21, m22, m23, m24;
			m31, m32, m33, m34;
			m41, m42, m43, m44;
			m51, m52, m53, m54];
	5, 6, [m11, m12, m13, m14, m15, m16;
			m21, m22, m23, m24, m25, m26;
			m31, m32, m33, m34, m35, m36;
			m41, m42, m43, m44, m45, m46;
			m51, m52, m53, m54, m55, m56];

	/*
	 * Rectangular Matrices with 5 rows
	 */
	6, 2, [m11, m12;
			m21, m22;
			m31, m32;
			m41, m42;
			m51, m52;
			m61, m62];
	6, 3, [m11, m12, m13;
			m21, m22, m23;
			m31, m32, m33;
			m41, m42, m43;
			m51, m52, m53;
			m61, m62, m63];
	6, 4, [m11, m12, m13, m14;
			m21, m22, m23, m24;
			m31, m32, m33, m34;
			m41, m42, m43, m44;
			m51, m52, m53, m54;
			m61, m62, m63, m64];
	6, 5, [m11, m12, m13, m14, m15;
			m21, m22, m23, m24, m25;
			m31, m32, m33, m34, m35;
			m41, m42, m43, m44, m45;
			m51, m52, m53, m54, m55;
			m61, m62, m63, m64, m65];

	/*
	 * Row Vectors
	*/
	1, 1, [x];
	1, 2, [x, y];
	1, 3, [x, y, z];
	1, 4, [x, y, z, w];
	1, 5, [x, y, z, w, a];
	1, 6, [x, y, z, w, a, b];

	/*
	 * Column Vectors
	*/
	2, 1, [x; y];
	3, 1, [x; y; z];
	4, 1, [x; y; z; w];
	5, 1, [x; y; z; w; a];
	6, 1, [x; y; z; w; a; b];
);
