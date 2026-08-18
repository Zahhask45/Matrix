use matrix::{Matrix};

fn main() {
	println!("\n========== TRACE TESTS ==========");
	
	// 1×1
	let m = Matrix::from([
	    [5.0],
	]);
	
	println!("1×1 matrix:");
	println!("{}", m);
	println!("trace = {}", m.trace());
	
	// 2×2
	let m = Matrix::from([
	    [1.0, 2.0],
	    [3.0, 4.0],
	]);
	
	println!("\n2×2 matrix:");
	println!("{}", m);
	println!("trace = {}", m.trace());
	
	// 3×3
	let m = Matrix::from([
	    [1.0, 2.0, 3.0],
	    [4.0, 5.0, 6.0],
	    [7.0, 8.0, 9.0],
	]);
	
	println!("\n3×3 matrix:");
	println!("{}", m);
	println!("trace = {}", m.trace());
	
	// Negative values
	let m = Matrix::from([
	    [-1.0, 2.0],
	    [3.0, -4.0],
	]);
	
	println!("\nnegative values:");
	println!("{}", m);
	println!("trace = {}", m.trace());
	
	// Complex 2×2
	let m = Matrix::from([
	    [(1.0, 2.0), (3.0, 4.0)],
	    [(5.0, -1.0), (6.0, 3.0)],
	]);
	
	println!("\n2×2 complex matrix:");
	println!("{}", m);
	println!("trace = {}", m.trace());
	
	// Complex 3×3
	let m = Matrix::from([
	    [(1.0, 2.0), (3.0, 4.0), (5.0, 6.0)],
	    [(7.0, -1.0), (2.0, 3.0), (4.0, -2.0)],
	    [(8.0, 5.0), (9.0, 1.0), (-3.0, 7.0)],
	]);
	
	println!("\n3×3 complex matrix:");
	println!("{}", m);
	println!("trace = {}", m.trace());
}
