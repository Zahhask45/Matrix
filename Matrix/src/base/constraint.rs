use crate::dimension::{Dim, DimName, Dyn};

/// Enforcing constraints
#[derive(Clone, Copy, Debug)]
pub struct ShapeConstraint;

/// `C1` and `R2` to be equivalent
pub trait AreMultipliable<R1:Dim, C1: Dim, R2: Dim, C2: Dim>: DimEq<C1, R2> {}

impl<R1: Dim, C1: Dim, R2: Dim, C2: Dim> AreMultipliable<R1, C1, R2, C2> for ShapeConstraint
where
	ShapeConstraint: DimEq<C1, R2>
{}

/// `D1` and `D2` to be equivalent
pub trait DimEq<D1: Dim, D2: Dim> {
	/// Is either equal to `D1` or `D2`, always cgoosing the one (if any) which is a type-level
	/// constant.
	type Representative: Dim;

	/// This constructs a value of type `Representative` with the
	/// correct value
	fn representative(d1: D1, d2: D2) -> Option<Self::Representative> {
		if d1.value() != d2.value() {
			None
		} else {
			Some(Self::Representative::from_usize(d1.value()))
		}
	}
}

impl<D: Dim> DimEq<D, D> for ShapeConstraint {
	type Representative = D;
}

impl<D: DimName> DimEq<D, Dyn> for ShapeConstraint {
	type Representative = D;
}

impl<D: DimName> DimEq<Dyn, D> for ShapeConstraint {
	type Representative = D;
}

macro_rules! equality_trait_decl(
	($($doc: expr, $Trait: ident),* $(,)*) => {$(
		#[doc = $doc]
		pub trait $Trait<D1: Dim, D2: Dim>: DimEq<D1, D1> + DimEq<D2, D1> {
			type Representative: Dim;

			fn representative(d1: D1, d2: D2) -> Option<<Self as $Trait<D1, D2>>::Representative> {
				<Self as DimEq<D1, D2>>::representative(d1, d2)
					.map(|common_dim| <Self as $Trait<D1, D2>>::Representative::from_usize(common_dim.value()))
			}
		}

		impl<D: Dim> $Trait<D, D> for ShapeConstraint {
			type Representative = D;
		}
		
		impl<D: DimName> $Trait<D, Dyn> for ShapeConstraint {
			type Representative = D;
		}
		
		impl<D: DimName> $Trait<Dyn, D> for ShapeConstraint {
			type Representative = D;
		}
		
	)*}
);

equality_trait_decl!(
	"Constrains `D1` and `D2` to be equivalent. \
	They are both assumed to be the number of \
	rows of a matrix.",
	SameNumbersOfRows,
	"Constrains `D1` and `D2` to be equivalent. \
	They are both assumed to be the number of \
	columns of a matrix.",
	SameNumbersOfColumns
);


pub trait SameDimension<D1: Dim, D2: Dim>:
	SameNumbersOfRows<D1, D2> + SameNumbersOfColumns<D1, D2>
{
	type Representative: Dim;
}

impl<D: Dim> SameDimension<D, D> for ShapeConstraint {
	type Representative = D;
}

impl<D: DimName> SameDimension<D, Dyn> for ShapeConstraint {
	type Representative = D;
}

impl<D: DimName> SameDimension<Dyn, D> for ShapeConstraint {
	type Representative = D;
}

