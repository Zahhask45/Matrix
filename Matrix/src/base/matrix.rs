use std::fmt;
use std::marker::PhantomData;
use std::ops::{Add, Mul, Sub};

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

impl<T, R, C, S> Default for Matrix<T, R, C, S>
where
	T: Scalar,
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

impl<T, const R: usize> From<[(T, T); R]> for Vector<Complex<T>, Const<R>, ArrayStorage<Complex<T>, R, 1>>
where T: Copy,
{
	fn from(value: [(T, T); R]) -> Self {
		let mut data = [Complex::new(value[0].0, value[0].1); R];
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

