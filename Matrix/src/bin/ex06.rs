use matrix::{Vector, cross_product};

fn main(){
	println!("\n========== CROSS PRODUCT TESTS ==========");
	
	// --------------------------------------------------
	// Basic real vectors
	// --------------------------------------------------
	
	let u = Vector::from([1., 0., 0.]);
	let v = Vector::from([0., 1., 0.]);
	
	let result = cross_product(&u, &v);
	println!("[1,0,0] × [0,1,0] = {}", result);
	// [0, 0, 1]
	
	
	let u = Vector::from([0., 1., 0.]);
	let v = Vector::from([1., 0., 0.]);
	
	let result = cross_product(&u, &v);
	println!("[0,1,0] × [1,0,0] = {}", result);
	// [0, 0, -1]
	
	
	// --------------------------------------------------
	// Same direction
	// --------------------------------------------------
	
	let u = Vector::from([1., 2., 3.]);
	let v = Vector::from([2., 4., 6.]);
	
	let result = cross_product(&u, &v);
	println!("[1,2,3] × [2,4,6] = {}", result);
	// [0, 0, 0]
	
	
	// --------------------------------------------------
	// Same vector
	// --------------------------------------------------
	
	let u = Vector::from([1., 2., 3.]);
	let result = cross_product(&u, &u);
	
	println!("[1,2,3] × [1,2,3] = {}", result);
	// [0, 0, 0]
	
	
	// --------------------------------------------------
	// Arbitrary vectors
	// --------------------------------------------------
	
	let u = Vector::from([1., 2., 3.]);
	let v = Vector::from([4., 5., 6.]);
	
	let result = cross_product(&u, &v);
	
	println!("[1,2,3] × [4,5,6] = {}", result);
	// [-3, 6, -3]
	
	
	// --------------------------------------------------
	// Negative values
	// --------------------------------------------------
	
	let u = Vector::from([-1., -2., -3.]);
	let v = Vector::from([4., 5., 6.]);
	
	let result = cross_product(&u, &v);
	
	println!("[-1,-2,-3] × [4,5,6] = {}", result);
	// [3, -6, 3]
	
	
	// --------------------------------------------------
	// Zero vector
	// --------------------------------------------------
	
	let u = Vector::from([0., 0., 0.]);
	let v = Vector::from([1., 2., 3.]);
	
	let result = cross_product(&u, &v);
	
	println!("[0,0,0] × [1,2,3] = {}", result);
	// [0, 0, 0]
	
	
	// --------------------------------------------------
	// Complex vectors
	// --------------------------------------------------
	
	let u = Vector::from([
	    (1., 0.),
	    (0., 0.),
	    (0., 0.),
	]);
	
	let v = Vector::from([
	    (0., 0.),
	    (1., 0.),
	    (0., 0.),
	]);
	
	let result = cross_product(&u, &v);
	
	println!("[1,0,0] × [0,1,0] (complex) = {}", result);
	// [1 + 0i, 0 + 0i, 0 + 1i]
	
	
	// --------------------------------------------------
	// Complex arbitrary vectors
	// --------------------------------------------------
	
	let u = Vector::from([
	    (1., 2.),
	    (3., 4.),
	    (5., 6.),
	]);
	
	let v = Vector::from([
	    (7., 8.),
	    (9., 10.),
	    (11., 12.),
	]);
	
	let result = cross_product(&u, &v);
	
	println!("complex arbitrary = {}", result);
	
	
	// --------------------------------------------------
	// Complex parallel vectors
	// --------------------------------------------------
	
	let u = Vector::from([
	    (1., 0.),
	    (2., 0.),
	    (3., 0.),
	]);
	
	let v = Vector::from([
	    (2., 0.),
	    (4., 0.),
	    (6., 0.),
	]);
	
	let result = cross_product(&u, &v);
	
	println!("complex parallel = {}", result);
	// [0 + 0i, 0 + 0i, 0 + 0i]
	
	
	// --------------------------------------------------
	// Complex zero vector
	// --------------------------------------------------
	
	let u = Vector::from([
	    (0., 0.),
	    (0., 0.),
	    (0., 0.),
	]);
	
	let v = Vector::from([
	    (1., 2.),
	    (3., 4.),
	    (5., 6.),
	]);
	
	let result = cross_product(&u, &v);
	
	println!("complex zero × vector = {}", result);
	// [0 + 0i, 0 + 0i, 0 + 0i]

	let u = Vector::from([1., 2., 3.]);
	let v = Vector::from([4., 5., 6.]);
	
	let uv = cross_product(&u, &v);
	let vu = cross_product(&v, &u);
	
	println!("u × v = {}", uv);
	println!("v × u = {}", vu);

	//u × v = [-3, 6, -3]
	//v × u = [3, -6, 3]
}
