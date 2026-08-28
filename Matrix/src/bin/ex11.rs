use matrix::{Matrix};

fn main() {
	let u = Matrix::from([
	[ 1., -1.],
	[-1., 1.],
	]);
	println!("{}", u.determinant());
	// 0.0
	let u = Matrix::from([
	[2., 0., 0.],
	[0., 2., 0.],
	[0., 0., 2.],
	]);
	println!("{}", u.determinant());
	// 8.0
	let u = Matrix::from([
	[8., 5., -2.],
	[4., 7., 20.],
	[7., 6., 1.],
	]);
	println!("{}", u.determinant());
	// -174.0
	let u = Matrix::from([
	[ 8., 5., -2., 4.],
	[ 4., 2.5, 20., 4.],
	[ 8., 5., 1., 4.],
	[28., -4., 17., 1.],
	]);
	println!("{}", u.determinant());
	// 1032

	let u = Matrix::from([
		[1., 2., 3., 4., 5.],
		[16., 17., 18., 14., 15.],
		[25., 1., 0., 74., 88.],
		[6., 7., 8., 9., 10.],
		[11., 12., 13., 14., 0.],
	]);
	println!("{}", u.determinant());
	// 8625
	
	let u = Matrix::from([
		[1., 2., 3., 4., 5.],
		[16., 17., 18., 14., 15.],
		[25., 1., 0., 74., 88.],
		[6., 7., 8., 9., 10.],
		[0., 0., 0., 0., 0.],
	]);
	println!("{}", u.determinant());
	// 0


	// 1x1
    let u = Matrix::from([
        [(3., 4.)],
    ]);

    println!("{}", u.determinant());
    // 3 + 4i


    // 2x2
    // det = (1+i)(2-i) - (2+0i)(0+3i)
    //     = 5 + 0i
    let u = Matrix::from([
        [(1., 1.), (2., 0.)],
        [(0., 3.), (2., -1.)],
    ]);

    println!("{}", u.determinant());
    // 5 + 0i


    // Complex diagonal matrix
    // det = (1+i)(2+2i)(3-i)
    let u = Matrix::from([
        [(1., 1.), (0., 0.), (0., 0.)],
        [(0., 0.), (2., 2.), (0., 0.)],
        [(0., 0.), (0., 0.), (3., -1.)],
    ]);

    println!("{}", u.determinant());
    // 12 + 12i


    // Matrix requiring a row swap
    let u = Matrix::from([
        [(0., 0.), (1., 0.)],
        [(2., 0.), (3., 0.)],
    ]);

    println!("{}", u.determinant());
    // -2 + 0i


    // Complex row swap with imaginary values
    let u = Matrix::from([
        [(0., 0.), (1., 1.)],
        [(2., 1.), (3., 0.)],
    ]);

    println!("{}", u.determinant());
    // -3 - 2i


    // Singular complex matrix
    // Row 2 = (2+i) * Row 1
    let u = Matrix::from([
        [(1., 1.), (2., 0.)],
        [(2., 3.), (3., 2.)],
    ]);

    println!("{}", u.determinant());


    // 3x3 complex matrix
    let u = Matrix::from([
        [(1., 1.), (2., 0.), (0., 1.)],
        [(0., 1.), (2., 1.), (3., 0.)],
        [(2., 0.), (0., 1.), (1., -1.)],
    ]);

    println!("{}", u.determinant());


    // 3x3 with a zero in the first column,
    // forcing the pivot search to continue.
    let u = Matrix::from([
        [(0., 0.), (2., 1.), (1., 0.)],
        [(0., 0.), (1., -1.), (3., 2.)],
        [(2., 3.), (0., 0.), (1., 1.)],
    ]);

    println!("{}", u.determinant());


    // Tests complex partial pivoting.
    // First column:
    // |1 + 0i| = 1
    // |3 + 4i| = 5
    // The second row should be selected as the pivot.
    let u = Matrix::from([
        [(1., 0.), (2., 0.)],
        [(3., 4.), (1., 0.)],
    ]);

    println!("{}", u.determinant());
    // -11 + 4i
}
