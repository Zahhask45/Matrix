use matrix::{Matrix};

fn main(){
	let u = Matrix::from([
	[1., 0., 0.],
	[0., 1., 0.],
	[0., 0., 1.],
	]);
	println!("{}", u.row_echelon());
	// [1.0, 0.0, 0.0]
	// [0.0, 1.0, 0.0]
	// [0.0, 0.0, 1.0]
	let u = Matrix::from([
	[1., 2.],
	[3., 4.],
	]);
	println!("{}", u.row_echelon());
	// [1.0, 0.0]
	// [0.0, 1.0]
	let u = Matrix::from([
	[1., 2.],
	[2., 4.],
	]);
	println!("{}", u.row_echelon());
	// [1.0, 2.0]
	// [0.0, 0.0]
	let u = Matrix::from([
	[8., 5., -2., 4., 28.],
	[4., 2.5, 20., 4., -4.],
	[8., 5., 1., 4., 17.],
	]);
	println!("{}", u.row_echelon());
	// [1.0, 0.625, 0.0, 0.0, -12.1666667]
	// [0.0, 0.0, 1.0, 0.0, -3.6666667]
	// [0.0, 0.0, 0.0, 1.0, 29.5 ]

	
	// Complex identity
    let u = Matrix::from([
        [(1., 0.), (0., 0.), (0., 0.)],
        [(0., 0.), (1., 0.), (0., 0.)],
        [(0., 0.), (0., 0.), (1., 0.)],
    ]);

    println!("Complex identity:\n{}\n", u.row_echelon());


    // Complex full-rank 2x2
    let u = Matrix::from([
        [(1., 1.), (2., 0.)],
        [(0., 1.), (1., -1.)],
    ]);

    println!("Complex full rank:\n{}\n", u.row_echelon());


    // Complex dependent rows
    // Row 2 = (2 + i) * Row 1
    let u = Matrix::from([
        [(1., 1.), (2., -1.)],
        [(3., 1.), (5., -3.)],
    ]);

    println!("Complex dependent rows:\n{}\n", u.row_echelon());


    // Zero first column
    let u = Matrix::from([
        [(0., 0.), (1., 2.)],
        [(0., 0.), (3., -1.)],
    ]);

    println!("Complex zero column:\n{}\n", u.row_echelon());


    // Pivot selection by magnitude
    // |1| = 1
    // |3 + 4i| = 5
    // Therefore 3 + 4i should be chosen as the pivot.
    let u = Matrix::from([
        [(1., 0.), (2., 0.)],
        [(3., 4.), (1., 0.)],
    ]);

    println!("Complex magnitude pivot:\n{}\n", u.row_echelon());


    // Negative/imaginary pivot
    // Candidates:
    // 1 + i       -> |.| = sqrt(2)
    // -2 + 3i     -> |.| = sqrt(13)
    // The second row should become the pivot row.
    let u = Matrix::from([
        [(1., 1.), (2., 0.)],
        [(-2., 3.), (1., -1.)],
    ]);

    println!("Complex larger pivot:\n{}\n", u.row_echelon());


    // Complex elimination
    let u = Matrix::from([
        [(1., 1.), (2., 0.), (3., -1.)],
        [(2., 0.), (1., 2.), (4., 1.)],
        [(0., 1.), (3., -1.), (1., 0.)],
    ]);

    println!("Complex elimination:\n{}\n", u.row_echelon());


    // 3x4 complex matrix
    let u = Matrix::from([
        [(1., 1.), (2., 0.), (0., 1.), (3., -1.)],
        [(2., 2.), (4., 0.), (0., 2.), (6., -2.)],
        [(0., 1.), (1., -1.), (2., 0.), (1., 2.)],
    ]);

    println!("Complex rectangular:\n{}\n", u.row_echelon());


    // Matrix with several zero columns before pivots
    let u = Matrix::from([
        [(0., 0.), (0., 0.), (1., 1.), (2., 0.)],
        [(0., 0.), (3., 4.), (2., -1.), (1., 1.)],
        [(0., 0.), (0., 0.), (2., 2.), (4., 0.)],
    ]);

    println!("Complex delayed pivots:\n{}\n", u.row_echelon());
}
