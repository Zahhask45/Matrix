use matrix::{Vector, angle_cos};

fn main(){
	println!("========== COSINE TESTS ==========");
	
	// Same direction
	let u = Vector::from([1., 0.]);
	let v = Vector::from([1., 0.]);
	println!("same direction: {}", angle_cos(&u, &v));
	// 1.0
	
	// Opposite direction
	let u = Vector::from([1., 0.]);
	let v = Vector::from([-1., 0.]);
	println!("opposite direction: {}", angle_cos(&u, &v));
	// -1.0
	
	// Perpendicular
	let u = Vector::from([1., 0.]);
	let v = Vector::from([0., 1.]);
	println!("perpendicular: {}", angle_cos(&u, &v));
	// 0.0
	
	// 45 degrees
	let u = Vector::from([1., 0.]);
	let v = Vector::from([1., 1.]);
	println!("45 degrees: {}", angle_cos(&u, &v));
	// 0.70710677
	
	// 60 degrees
	let u = Vector::from([1., 0.]);
	let v = Vector::from([0.5, 0.8660254]);
	println!("60 degrees: {}", angle_cos(&u, &v));
	// ~0.5
	
	// 120 degrees
	let u = Vector::from([1., 0.]);
	let v = Vector::from([-0.5, 0.8660254]);
	println!("120 degrees: {}", angle_cos(&u, &v));
	// ~-0.5

	// [1,2,3] and [4,5,6]
	let u = Vector::from([1., 2., 3.]);
	let v = Vector::from([4., 5., 6.]);
	
	println!("[1,2,3] vs [4,5,6]: {}", angle_cos(&u, &v));
	// 0.97463185
	
	// Orthogonal in 3D
	let u = Vector::from([1., 2., 0.]);
	let v = Vector::from([-2., 1., 0.]);
	
	println!("orthogonal 3D: {}", angle_cos(&u, &v));
	// 0.0
	
	// Negative components
	let u = Vector::from([-1., -2., -3.]);
	let v = Vector::from([1., 2., 3.]);
	
	println!("opposite 3D: {}", angle_cos(&u, &v));
	// -1.0

	let u = Vector::from([1., 2., 3.]);
	let v = Vector::from([10., 20., 30.]);
	
	println!("same direction, different magnitude: {}", angle_cos(&u, &v));
	// 1.0

	let u = Vector::from([1., 2., 3.]);
	let v = Vector::from([-10., -20., -30.]);
	
	println!("opposite direction, different magnitude: {}", angle_cos(&u, &v));
	// -1.0

	// Same complex vector
	let u = Vector::from([(1., 0.), (0., 1.)]);
	let v = Vector::from([(1., 0.), (0., 1.)]);
	
	println!("complex same: {}", angle_cos(&u, &v));
	// 1.0

	// Orthogonal-looking complex vectors
	let u = Vector::from([(1., 0.), (0., 0.)]);
	let v = Vector::from([(0., 0.), (1., 0.)]);
	
	println!("complex orthogonal: {}", angle_cos(&u, &v));
	// 0.0

	let u = Vector::from([(1., 2.), (3., 4.)]);
	let v = Vector::from([(5., 6.), (7., 8.)]);
	
	println!("complex arbitrary: {}", angle_cos(&u, &v));

	let u = Vector::from([0., 0.]);
	let v = Vector::from([1., 2.]);
	
	println!("zero vector: {}", angle_cos(&u, &v));
}
