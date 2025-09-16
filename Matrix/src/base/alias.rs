use crate::base::dimension::Dyn;
use crate::base::vec_storage::VecStorage;
use crate::base::{U1, U2, U3, U4, U5, U6};
use crate::base::{ArrayStorage, Const, Matrix};


// Static sized collumn-major matrix
pub type SMatrix<T, const R: usize, const C: usize> =
	Matrix<T, Const<R>, Const<C>, ArrayStorage<T, R, C>>;

// Dynamically sized collumn-major matrix
pub type DMatrix<T> = Matrix<T, Dyn, Dyn, VecStorage<T, Dyn, Dyn>>;

pub type Matrix1<T> = Matrix<T, U1, U1, ArrayStorage<T, 1, 1>>;
pub type Matrix2<T> = Matrix<T, U2, U2, ArrayStorage<T, 2, 2>>;
pub type Matrix3<T> = Matrix<T, U3, U3, ArrayStorage<T, 3, 3>>;
pub type Matrix4<T> = Matrix<T, U4, U4, ArrayStorage<T, 4, 4>>;
pub type Matrix5<T> = Matrix<T, U5, U5, ArrayStorage<T, 5, 5>>;
pub type Matrix6<T> = Matrix<T, U6, U6, ArrayStorage<T, 6, 6>>;



/*
 *
 * Column Vectors
 *
 */

// Static sized collumn Vector
pub type SVector<T, const D: usize> = Matrix<T, Const<D>, U1, ArrayStorage<T, D, 1>>;

// Dynamic sized collumn Vector
pub type DVector<T> = Matrix<T, Dyn, U1, VecStorage<T, Dyn, U1>>;

pub type Vector1<T> = Matrix<T, U1, U1, ArrayStorage<T, 1, 1>>;
pub type Vector2<T> = Matrix<T, U2, U1, ArrayStorage<T, 2, 1>>;
pub type Vector3<T> = Matrix<T, U3, U1, ArrayStorage<T, 3, 1>>;
pub type Vector4<T> = Matrix<T, U4, U1, ArrayStorage<T, 4, 1>>;
pub type Vector5<T> = Matrix<T, U5, U1, ArrayStorage<T, 5, 1>>;
pub type Vector6<T> = Matrix<T, U6, U1, ArrayStorage<T, 6, 1>>;

