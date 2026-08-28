use matrix::{Matrix};

fn main() {
	let u = Matrix::from([
	[1., 0., 0.],
	[0., 1., 0.],
	[0., 0., 1.],
	]);
	match u.inverse() {
	    Ok(inverse) => println!("{}", inverse),
	    Err(error) => println!("{:?}", error),
	}
	// [1.0, 0.0, 0.0]
	// [0.0, 1.0, 0.0]
	// [0.0, 0.0, 1.0]
	let u = Matrix::from([
	[2., 0., 0.],
	[0., 2., 0.],
	[0., 0., 2.],
	]);
	match u.inverse() {
	    Ok(inverse) => println!("{}", inverse),
	    Err(error) => println!("{:?}", error),
	}
	// [0.5, 0.0, 0.0]
	// [0.0, 0.5, 0.0]
	// [0.0, 0.0, 0.5]
	let u = Matrix::from([
	[8., 5., -2.],
	[4., 7., 20.],
	[7., 6., 1.],
	]);
	match u.inverse() {
	    Ok(inverse) => println!("{}", inverse),
	    Err(error) => println!("{:?}", error),
	}
	// [0.649425287, 0.097701149, -0.655172414]
	// [-0.781609195, -0.126436782, 0.965517241]
	// [0.143678161, 0.074712644, -0.206896552]

	// ─────────────────────────────────────────────
    // 1. Simple 2x2
    // ─────────────────────────────────────────────

    let u = Matrix::from([
        [4., 7.],
        [2., 6.],
    ]);

    match u.inverse() {
        Ok(inverse) => println!("2x2:\n{}\n", inverse),
        Err(error) => println!("2x2: {:?}", error),
    }

    // Expected:
    // [0.6, -0.7]
    // [-0.2, 0.4]


    // ─────────────────────────────────────────────
    // 2. Singular matrix
    // ─────────────────────────────────────────────

    let u = Matrix::from([
        [1., 2.],
        [2., 4.],
    ]);

    match u.inverse() {
        Ok(inverse) => println!("singular:\n{}\n", inverse),
        Err(error) => println!("singular: {:?}", error),
    }

    // Expected:
    // SingularMatrix


    // ─────────────────────────────────────────────
    // 3. 3x3
    // ─────────────────────────────────────────────

    let u = Matrix::from([
        [1., 2., 3.],
        [0., 1., 4.],
        [5., 6., 0.],
    ]);

    match u.inverse() {
        Ok(inverse) => println!("3x3:\n{}\n", inverse),
        Err(error) => println!("3x3: {:?}", error),
    }

    // Expected:
    // [ -24,  18,  5]
    // [  20, -15, -4]
    // [  -5,   4,  1]


    // ─────────────────────────────────────────────
    // 4. Identity
    // ─────────────────────────────────────────────

    let u = Matrix::from([
        [1., 0., 0.],
        [0., 1., 0.],
        [0., 0., 1.],
    ]);

    match u.inverse() {
        Ok(inverse) => println!("identity:\n{}\n", inverse),
        Err(error) => println!("identity: {:?}", error),
    }

    // Expected:
    // [1, 0, 0]
    // [0, 1, 0]
    // [0, 0, 1]


    // ─────────────────────────────────────────────
    // 5. Complex 2x2
    // ─────────────────────────────────────────────

    let u = Matrix::from([
        [(1., 2.), (3., -1.)],
        [(2., 0.), (1., 4.)],
    ]);

    match u.inverse() {
        Ok(inverse) => println!("complex 2x2:\n{}\n", inverse),
        Err(error) => println!("complex 2x2: {:?}", error),
    }


    // ─────────────────────────────────────────────
    // 6. Complex singular
    // ─────────────────────────────────────────────

    let u = Matrix::from([
        [(1., 2.), (2., 4.)],
        [(2., 4.), (4., 8.)],
    ]);

    match u.inverse() {
        Ok(inverse) => println!("complex singular:\n{}\n", inverse),
        Err(error) => println!("complex singular: {:?}", error),
    }


    // ─────────────────────────────────────────────
    // 7. Complex diagonal
    // ─────────────────────────────────────────────

    let u = Matrix::from([
        [(2., 1.), (0., 0.)],
        [(0., 0.), (1., -1.)],
    ]);

    match u.inverse() {
        Ok(inverse) => println!("complex diagonal:\n{}\n", inverse),
        Err(error) => println!("complex diagonal: {:?}", error),
    }
}
