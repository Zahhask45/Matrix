use std::fmt;
use std::marker::PhantomData;
use std::ops::{Add, Mul, Sub, AddAssign, Neg};

use crate::base::{ArrayStorage, Const, Dim, LinearScalar, Scalar, U1, Complex};

pub type Vector<T, D, S> = Matrix<T, D, U1, S>;

#[derive(Clone, Copy)]
pub struct Matrix<T, R, C, S> {
	pub data: S,
	_phantoms: PhantomData<(T, R, C)>,
}

impl<T, R: Dim, C: Dim, S: fmt::Debug> fmt::Debug for Matrix<T, R, C, S> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
		self.data.fmt(f)
	}
}

impl<K, R, C, S> Default for Matrix<K, R, C, S>
where
	K: LinearScalar,
	R: Dim,
	C: Dim,
	S: Default,
{
	fn default() -> Self{
		Matrix {
			data: Default::default(),
			_phantoms: PhantomData,
		}
	}
}

impl<K> Vector<K, Const<3>, ArrayStorage<K, 3, 1>> {
    pub fn new(x: K, y: K, z: K) -> Self {
        Matrix {
            data: ArrayStorage([[x, y, z]]),
            _phantoms: PhantomData,
        }
    }
}

impl<T, R, C, S> Matrix<T, R, C, S> {
	/// # Safety
	#[inline(always)]
	pub const unsafe fn from_data_statically_unchecked(data: S) -> Matrix<T, R, C, S> {
		Matrix {
			data,
			_phantoms: PhantomData,
		}
	}
}

impl<K, const D: usize> Matrix<K, Const<D>, Const<D>, ArrayStorage<K, D, D>>
where K: LinearScalar {
	pub fn identity() -> Self {
		let mut result = Matrix {
			data: ArrayStorage([[K::default();D]; D]),
			_phantoms: PhantomData,
		};

		for i in 0..D {
			result.data.0[i][i] = K::one();
		}

		result
	}
}

impl<K, const R: usize, const C: usize> Matrix<K, Const<R>, Const<C>, ArrayStorage<K, R, C>>
where K: LinearScalar {
	pub fn diagonal<const D: usize>(values: [K; D]) -> Self {
		assert!(D <= R.min(C));
		let mut result = Matrix {
		    data: ArrayStorage([[K::default(); R]; C]),
		    _phantoms: PhantomData,
		};
		for idx in 0..D {
			result.data.0[idx][idx] = values[idx];
		}

		result
	}

	pub fn to_diagonal(&self) -> Self {
		let mut result = Matrix {
		    data: ArrayStorage([[K::default(); R]; C]),
		    _phantoms: PhantomData,
		};

		for idx in 0..R.min(C) {
			result.data.0[idx][idx] = self.data.0[idx][idx];
		}

		result
	}
}

impl<T, const R: usize, const C: usize> Add for Matrix<T, Const<R>, Const<C>, ArrayStorage<T, R, C>>
where T: Scalar + Copy + Add<Output = T>,
{
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		let mut result = self;

		for col in 0..C {
			for row in 0..R {
				result.data.0[col][row] = result.data.0[col][row] + rhs.data.0[col][row];
			}
		}

		result
	}
}

impl<T, const R: usize, const C: usize> Mul<f32> for Matrix<T, Const<R>, Const<C>, ArrayStorage<T, R, C>>
where T: Scalar + Copy + Mul<f32, Output = T>,
{
	type Output = Self;
	fn mul(self, scalar: f32) -> Self::Output {
		let mut result = self;

		for col in 0..C {
			for row in 0..R {
				result.data.0[col][row] = result.data.0[col][row] * scalar;
			}
		}

		result
	}
}

impl<T, const R: usize, const C: usize> Matrix<T, Const<R>, Const<C>, ArrayStorage<T, R, C>>
where
	T: Scalar + Copy,
{
	#[inline]
	pub fn shape(&self) -> (usize, usize) {
		(R, C)
	}

	#[inline]
	pub fn is_square(&self) -> bool {
		R == C
	}

	#[inline]
	pub fn add(&mut self, rhs: &Self)
	where
		T: Add<Output = T>,
	{
		for col in 0..C {
			for row in 0..R {
				self.data.0[col][row] = self.data.0[col][row] + rhs.data.0[col][row];
			}
		}
	}

	#[inline]
	pub fn sub(&mut self, rhs: &Self)
	where
		T: Sub<Output = T>,
	{
		for col in 0..C {
			for row in 0..R {
				self.data.0[col][row] = self.data.0[col][row] - rhs.data.0[col][row];
			}
		}
	}

	#[inline]
	pub fn scl(&mut self, scalar: T)
	where
		T: Mul<Output = T>,
	{
		for col in 0..C {
			for row in 0..R {
				self.data.0[col][row] = self.data.0[col][row] * scalar;
			}
		}
	}

	#[inline]
	pub fn reshape<const R2: usize, const C2: usize>(
		self,
	) -> Matrix<T, Const<R2>, Const<C2>, ArrayStorage<T, R2, C2>>
	where
		T: Default,
	{
		assert_eq!(R * C, R2 * C2, "reshape: element count mismatch");

		let mut out = [[T::default(); R2]; C2];
		let src = self.data.as_slice();

		for (idx, value) in src.iter().copied().enumerate() {
			let col = idx / R2;
			let row = idx % R2;
			out[col][row] = value;
		}

		Matrix {
			data: ArrayStorage(out),
			_phantoms: PhantomData,
		}
	}
}

impl<K, const D: usize> Matrix::<K, Const<D>, Const<D>, ArrayStorage<K, D, D>>
where K: LinearScalar<Real = f32>
{
	pub fn is_identity(&self) -> bool{
		let epsilon = 0.0001;
		for row in 0..D {
	        for col in 0..D {
	            let expected = if row == col {
	                K::one()
	            } else {
	                K::zero()
	            };

	            if (self.data.0[col][row] - expected).abs() > epsilon {
	                return false;
	            }
	        }
	    }

	    true
	}
}

impl<T, const R: usize> From<[(T, T); R]> for Vector<Complex<T>, Const<R>, ArrayStorage<Complex<T>, R, 1>>
where T: Copy + Default,
{
	fn from(value: [(T, T); R]) -> Self {
		let mut data = [Complex::default(); R];
		for i in 0..R {
			data[i] = Complex::new(value[i].0, value[i].1);
		}
		Matrix {
			data: ArrayStorage([data]),
			_phantoms: PhantomData,
		}
	}
}

impl<const R: usize> From<[f32; R]> for Vector<f32, Const<R>, ArrayStorage<f32, R, 1>> {
	fn from(value: [f32; R]) -> Self {
		Matrix {
			data: ArrayStorage([value]),
			_phantoms: PhantomData,
		}
	}
}



impl<const R: usize, const C: usize> From<[[f32; C]; R]>
	for Matrix<f32, Const<R>, Const<C>, ArrayStorage<f32, R, C>>
{
	fn from(rows: [[f32; C]; R]) -> Self {
		// Store as column-major: data[col][row].
		let mut cols = [[0.0; R]; C];
		for (row_idx, row) in rows.iter().enumerate() {
			for (col_idx, value) in row.iter().copied().enumerate() {
				cols[col_idx][row_idx] = value;
			}
		}

		Matrix {
			data: ArrayStorage(cols),
			_phantoms: PhantomData,
		}
	}
}


impl<T, const R: usize, const C: usize> From<[[(T, T); C]; R]>
	for Matrix<Complex<T>, Const<R>, Const<C>,  ArrayStorage<Complex<T>, R, C>>
where T: Copy + Default,
{
	fn from(rows: [[(T, T); C]; R]) -> Self {
		let mut cols = [[Complex::default(); R]; C];
		for row in 0..R {
			for col in 0..C {
				cols[col][row] = Complex::new(rows[row][col].0, rows[row][col].1);
				
			}
		}
		Matrix {
			data: ArrayStorage(cols),
			_phantoms: PhantomData,
		}
	}
}

impl<T: fmt::Display + Copy, const R: usize, const C: usize> fmt::Display
	for Matrix<T, Const<R>, Const<C>, ArrayStorage<T, R, C>>
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
		for row in 0..R {
			write!(f, "[")?;
			for col in 0..C {
				write!(f, "{}", self.data.0[col][row])?;
				if col + 1 < C {
					write!(f, ", ")?;
				}
			}
			write!(f, "]")?;
			if row + 1 < R {
				writeln!(f)?;
			}
		}
		Ok(())
	}
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinearCombinationError {
	Empty,
	LengthMismatch {matrices: usize, coefs: usize}
}

pub fn linear_combination<K, const R: usize, const C: usize>(
	m: &[Matrix<K, Const<R>, Const<C>, ArrayStorage<K, R, C>>],
	coefs: &[K]) -> Result<Matrix<K, Const<R>, Const<C>, ArrayStorage<K, R, C>>, LinearCombinationError>
where
	K: LinearScalar,
{
	if m.is_empty() {
		return Err(LinearCombinationError::Empty);
	}
	
	if m.len() != coefs.len(){
		return Err(LinearCombinationError::LengthMismatch{
			matrices: m.len(),
			coefs: coefs.len()
		});
	}
	let mut out = m[0];
	out.scl(coefs[0]);

	for idx in 1..m.len(){
		for col in 0..C {
			for row in 0..R {
				let z = out.data.0[col][row];
				let y = m[idx].data.0[col][row];
				out.data.0[col][row] = coefs[idx].fma(y, z)
			}
		}
	}
	Ok(out)
}

pub fn lerp<V>(u: V, v: V, t: f32) -> V
where V: Add<Output = V> + Mul<f32, Output = V>,
{
	u * (1.0 - t) + v * t
}

impl<K, const R: usize> Vector::<K, Const<R>, ArrayStorage<K, R, 1>>
where K: LinearScalar,
{
	pub fn dot(&self, rhs: Vector<K, Const<R>, ArrayStorage<K, R, 1>>) -> K{
		let mut result = K::default();
		for row in 0..R {
			result = result + self.data.0[0][row] * rhs.data.0[0][row].conj();
		}
		result
	}
}


impl<K, const R: usize> Vector::<K, Const<R>, ArrayStorage<K, R, 1>>
where K: LinearScalar<Real = f32>, f32: Default + AddAssign<<K as LinearScalar>::Real>
{
	pub fn norm_1(&self) -> f32{
		let mut result = f32::default();
		for row in 0..R {
			result += self.data.0[0][row].abs();
		}
		result
	}

	pub fn norm(&self) -> f32{
		self.dot(*self).abs().sqrt()
	}

	pub fn norm_inf(&self) -> f32{
		let mut result = f32::default();

		for row in 0..R {
			result = result.max(self.data.0[0][row].abs());
		}

		result
	}
}


impl<K, const R: usize, const C: usize> Matrix::<K, Const<R>, Const<C>, ArrayStorage<K, R, C>>
where K: LinearScalar<Real = f32>, f32: Default + AddAssign<<K as LinearScalar>::Real>
{
	pub fn l1_norm(&self) -> f32{
		let mut result = f32::default();

		for col in 0..C {
			for row in 0..R {
				result += self.data.0[col][row].abs();
			}
		}
		result
	}

	pub fn norm_2_1(&self) -> f32{
		let mut result = f32::default();

		for col in 0..C {
			let mut column = f32::default();

			for row in 0..R {
				let value = self.data.0[col][row].abs();

				column += value * value;
			}
			result += column.sqrt();
		}

		result
	}

	pub fn norm_frobenius(&self) -> f32{
		let mut result = f32::default();

		for col in 0..C {
			for row in 0..R {
				let value = self.data.0[col][row].abs();
				result += value * value;
			}
		}
		result.sqrt()
	}

	pub fn inf_norm(&self) -> f32 {
		let mut result = f32::default();

		for col in 0..C {
			for row in 0..R {
				result = result.max(self.data.0[col][row].abs());
			}
		}
		result
	}
	
}

pub fn angle_cos<K, const R: usize>(
	u: &Vector<K, Const<R>, ArrayStorage<K, R, 1>>,
	v: &Vector<K, Const<R>, ArrayStorage<K, R, 1>>) -> f32
where
	K: LinearScalar<Real = f32>,
{
	u.dot(*v).re() / (u.norm() * v.norm())
}

pub fn cross_product<K>(
	u: &Vector<K, Const<3>, ArrayStorage<K, 3, 1>>,
	v: &Vector<K, Const<3>, ArrayStorage<K, 3, 1>>) -> Vector<K, Const<3>, ArrayStorage<K, 3, 1>>
where
	K: LinearScalar,
{
	Vector::new(
		u.data.0[0][1] * v.data.0[0][2] - u.data.0[0][2] * v.data.0[0][1],
		u.data.0[0][2] * v.data.0[0][0] - u.data.0[0][0] * v.data.0[0][2],
		u.data.0[0][0] * v.data.0[0][1] - u.data.0[0][1] * v.data.0[0][0],
	)
}

impl<K, const R: usize, const C: usize> Matrix::<K, Const<R>, Const<C>, ArrayStorage<K, R, C>>
where K: LinearScalar
{
	pub fn mul_vec(&self, vec: &Vector<K, Const<C>, ArrayStorage<K, C, 1>>) -> Vector<K, Const<R>, ArrayStorage<K, R, 1>>
	{
		let mut result = Vector {
		    data: ArrayStorage([[K::default(); R]]),
		    _phantoms: PhantomData,
		};
		
		for row in 0..R {
			let mut sum = K::default();
			for col in 0..C {
				sum = self.data.0[col][row].fma(vec.data.0[0][col], sum);
			}
			result.data.0[0][row] = sum;
		}
		result
	}

	pub fn mul_mat<const P: usize>(&self,
		mat: &Matrix<K, Const<C>, Const<P>, ArrayStorage<K, C, P>>) -> Matrix::<K, Const<R>, Const<P>, ArrayStorage<K, R, P>> {
		let mut result = Matrix {
		    data: ArrayStorage([[K::default(); R]; P]),
		    _phantoms: PhantomData,
		};

        for row in 0..R {
			for col in 0..P {
            	let mut sum = K::default();
            	
            	for k in 0..C {
	            	sum = self.data.0[k][row].fma(mat.data.0[col][k], sum);
            	}
				result.data.0[col][row] = sum;
	        }
	    } 

		result
	}
}

impl<K, const D: usize> Matrix::<K, Const<D>, Const<D>, ArrayStorage<K, D, D>>
where K: LinearScalar
{
	pub fn trace(&self) -> K{
		let mut result = K::one();

		for idx in 0..D {
			result = result + self.data.0[idx][idx];
		}

		result
	}
}



impl<K, const D: usize> Matrix::<K, Const<D>, Const<D>, ArrayStorage<K, D, D>>
where K: LinearScalar + Neg<Output = K>
{

	pub fn determinant(&self) -> K{
		let mut result = K::one();
		let (gaussian, swap_count) = self.gaussian_elimination();

		for idx in 0..D {
			result = result * gaussian.data.0[idx][idx];
		}

		if swap_count % 2 != 0 {
			result = -result;
		}

		result
	}

}


#[derive(Debug)]
pub enum InverseError {
    SingularMatrix,
}


impl<K, const D: usize> Matrix::<K, Const<D>, Const<D>, ArrayStorage<K, D, D>>
where K: LinearScalar<Real = f32>
{
	pub fn inverse(&self) -> Result<Matrix::<K, Const<D>, Const<D>, ArrayStorage<K, D, D>>, InverseError>{
		let mut to_identity = *self;
		let mut identity = Matrix::identity();
		
		let mut row_pivot = 0;
		let mut col_pivot = 0;

		while row_pivot < D && col_pivot < D{

			let (col, row, _) = match to_identity.find_pivot(col_pivot, row_pivot) {
				Some(pivot) => pivot,
				None => return Err(InverseError::SingularMatrix),
			};

			col_pivot = col;

			//swap lines if needed

			if row != row_pivot {
				for col_id in 0..D {
					(to_identity.data.0[col_id][row_pivot], to_identity.data.0[col_id][row]) = (to_identity.data.0[col_id][row], to_identity.data.0[col_id][row_pivot]);
					(identity.data.0[col_id][row_pivot], identity.data.0[col_id][row]) = (identity.data.0[col_id][row], identity.data.0[col_id][row_pivot]);
				}
			}
		
			// normilize row if pivot != 1
			if to_identity.data.0[col_pivot][row_pivot] != K::one(){
				let scalar = to_identity.data.0[col_pivot][row_pivot];
				for col_id in 0..D{
					to_identity.data.0[col_id][row_pivot] = to_identity.data.0[col_id][row_pivot] * (K::one() / scalar);
					identity.data.0[col_id][row_pivot] = identity.data.0[col_id][row_pivot] * (K::one() / scalar);
				}
			}

			// subtract by other row ex: r2 - xr1 || check by col row
			for row_id in 0..D{
				if row_id == row_pivot {
					continue;
				}
				if to_identity.data.0[col_pivot][row_id] != K::zero(){
					let scalar = to_identity.data.0[col_pivot][row_id];
					for col_id in 0..D{
						to_identity.data.0[col_id][row_id] = to_identity.data.0[col_id][row_id] - (scalar * to_identity.data.0[col_id][row_pivot]);
						identity.data.0[col_id][row_id] = identity.data.0[col_id][row_id] - (scalar * identity.data.0[col_id][row_pivot]);
					}
				}
			}


			row_pivot += 1;
			col_pivot += 1;
		}
		if to_identity.is_identity(){
			return Ok(identity)
		}
		Err(InverseError::SingularMatrix)
	}
}




impl<K, const R: usize, const C: usize> Matrix::<K, Const<R>, Const<C>, ArrayStorage<K, R, C>>
where K: LinearScalar
{
	pub fn transpose(&self) ->  Matrix::<K, Const<C>, Const<R>, ArrayStorage<K, C, R>>{
		let mut result = Matrix {
		    data: ArrayStorage([[K::default(); C]; R]),
		    _phantoms: PhantomData,
		};

		for col in 0..C {
			for row in 0..R {
				result.data.0[row][col] = self.data.0[col][row];
			}
		}

		result	
	}

	fn find_pivot(&self, col_pivot: usize, row_pivot: usize) -> Option<(usize, usize, K)>{
		for col in col_pivot..C {
	        let mut best_row: Option<(usize, K)> = None;
	
	        for row in row_pivot..R {
	            let value = self.data.0[col][row];
	
	            if value != K::zero() {
	                match best_row {
	                    None => {
	                        best_row = Some((row, value));
	                    }
	                    Some((_, current)) => {
	                        if value.abs().partial_cmp(&current.abs()).unwrap().is_gt() {
	                            best_row = Some((row, value));
	                        }
	                    }
	                }
	            }
	        }
	
	        if let Some((row, value)) = best_row {
	            return Some((col, row, value));
	        }
	    }
	
	    None
	}

	pub fn row_echelon(&self) -> Matrix::<K, Const<R>, Const<C>, ArrayStorage<K, R, C>>{
		let mut result = *self;

		let mut row_pivot = 0;
		let mut col_pivot = 0;

		while row_pivot < R && col_pivot < C{

			// find the pivot value improve to find highest or K::one()
			let (col, row, _) = match result.find_pivot(col_pivot, row_pivot) {
				Some(pivot) => pivot,
				None => return result,
			};

			col_pivot = col;

			//swap lines if needed

			if row != row_pivot {
				for col_id in 0..C {
					(result.data.0[col_id][row_pivot], result.data.0[col_id][row]) = (result.data.0[col_id][row], result.data.0[col_id][row_pivot]);
				}
			}
		
			// normilize row if pivot != 1
			if result.data.0[col_pivot][row_pivot] != K::one(){
				let scalar = result.data.0[col_pivot][row_pivot];
				for col_id in 0..C{
					result.data.0[col_id][row_pivot] = result.data.0[col_id][row_pivot] * (K::one() / scalar);
				}
			}

			// subtract by other row ex: r2 - xr1 || check by col row
			for row_id in 0..R{
				if row_id == row_pivot {
					continue;
				}
				if result.data.0[col_pivot][row_id] != K::zero(){
					let scalar = result.data.0[col_pivot][row_id];
					for col_id in col_pivot..C{
						result.data.0[col_id][row_id] = result.data.0[col_id][row_id] - (scalar * result.data.0[col_id][row_pivot]);
					}
				}
			}


			row_pivot += 1;
			col_pivot += 1;
		}

		result
	}

	fn gaussian_elimination(&self) -> (Matrix::<K, Const<R>, Const<C>, ArrayStorage<K, R, C>>, u32){
		let mut result = *self;
		let mut swap_count = 0;
		
		let mut row_pivot = 0;
		let mut col_pivot = 0;

		while row_pivot < R && col_pivot < C{

			let (col, row, _) = match result.find_pivot(col_pivot, row_pivot) {
				Some(pivot) => pivot,
				None => return (result, swap_count),
			};

			col_pivot = col;

			//swap lines if needed

			if row != row_pivot {
				swap_count += 1;
				for col_id in 0..C {
					(result.data.0[col_id][row_pivot], result.data.0[col_id][row]) = (result.data.0[col_id][row], result.data.0[col_id][row_pivot]);
				}
			}

			// subtract by other row ex: r2 - xr1 || check by col row
			let pivot_value = result.data.0[col_pivot][row_pivot];
			
			for row_id in row_pivot+1..R{
				if result.data.0[col_pivot][row_id] != K::zero(){
					let scalar = result.data.0[col_pivot][row_id] / pivot_value;
					for col_id in col_pivot..C{
						result.data.0[col_id][row_id] = result.data.0[col_id][row_id] - (scalar * result.data.0[col_id][row_pivot]);
					}
				}
			}


			row_pivot += 1;
			col_pivot += 1;
		}

		(result, swap_count)
	}

	fn is_non_zero_row(&self, row: usize) -> bool{
		for col in 0..C{
			if self.data.0[col][row] != K::zero() { return true}
		}

		false
	}

	pub fn rank(&self) -> usize{
		let mut rank: usize = 0;
		let (matrix, _) = self.gaussian_elimination();

		for row in 0..R{
			if matrix.is_non_zero_row(row) { rank += 1; }
		}

		rank
	}
}

impl Matrix::<f32, Const<4>, Const<4>, ArrayStorage<f32, 4, 4>>
{
	pub fn projection(fov: f32, ratio: f32, near: f32, far: f32) -> Self{
	    let mut result = Self {
   		    data: ArrayStorage([[0.; 4]; 4]),
   		    _phantoms: PhantomData,
   		};
	
	    let tan_half_fov = (fov / 2.).tan();
	
	    result.data.0[0][0] = 1. / (ratio * tan_half_fov);
	    result.data.0[1][1] = 1. / tan_half_fov;
	    
	    // Z: [0, 1] NDC
		result.data.0[2][2] = -far / (far - near);
		result.data.0[3][2] = -(far * near) / (far - near);
		
		result.data.0[2][3] = -1.;
	
	    result
	}
}

impl<K, const R: usize, const C: usize> Matrix::<K, Const<R>, Const<C>, ArrayStorage<K, R, C>>
where K: LinearScalar + fmt::Display,
{
	pub fn print_col_major(&self){
		for col in 0..C {
			for row in 0..R {
				print!("{}", self.data.0[col][row]);
				if row + 1 < R {
					print!(", ");
				}
			}
			if col + 1 < C {
				println!();
			}
		}
	}
}
