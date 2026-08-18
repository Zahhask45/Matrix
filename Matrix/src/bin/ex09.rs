use matrix::{Matrix};

fn main() {
	println!("\n========== TRANSPOSE TESTS ==========");
	
	// 1×1
	let m = Matrix::from([
	    [5.0],
	]);
	
	println!("1×1 matrix:");
	println!("{}", m);
	println!("transpose:");
	println!("{}", m.transpose());
	
	
	// 2×2
	let m = Matrix::from([
	    [1.0, 2.0],
	    [3.0, 4.0],
	]);
	
	println!("\n2×2 matrix:");
	println!("{}", m);
	println!("transpose:");
	println!("{}", m.transpose());
	
	
	// 2×3
	let m = Matrix::from([
	    [1.0, 2.0, 3.0],
	    [4.0, 5.0, 6.0],
	]);
	
	println!("\n2×3 matrix:");
	println!("{}", m);
	println!("transpose:");
	println!("{}", m.transpose());
	
	
	// 3×2
	let m = Matrix::from([
	    [1.0, 2.0],
	    [3.0, 4.0],
	    [5.0, 6.0],
	]);
	
	println!("\n3×2 matrix:");
	println!("{}", m);
	println!("transpose:");
	println!("{}", m.transpose());
	
	
	// 3×5
	let m = Matrix::from([
	    [1.0,  2.0,  3.0,  4.0,  5.0],
	    [6.0,  7.0,  8.0,  9.0, 10.0],
	    [11.0, 12.0, 13.0, 14.0, 15.0],
	]);
	
	println!("\n3×5 matrix:");
	println!("{}", m);
	println!("transpose:");
	println!("{}", m.transpose());
	
	
	// 1×4
	let m = Matrix::from([
	    [1.0, 2.0, 3.0, 4.0],
	]);
	
	println!("\n1×4 matrix:");
	println!("{}", m);
	println!("transpose:");
	println!("{}", m.transpose());
	
	
	// 4×1
	let m = Matrix::from([
	    [1.0],
	    [2.0],
	    [3.0],
	    [4.0],
	]);
	
	println!("\n4×1 matrix:");
	println!("{}", m);
	println!("transpose:");
	println!("{}", m.transpose());
	
	
	// Complex 2×3
	let m = Matrix::from([
	    [(1.0, 2.0), (3.0, -1.0), (5.0, 4.0)],
	    [(2.0, 0.0), (4.0, 3.0), (6.0, -2.0)],
	]);
	
	println!("\n2×3 complex matrix:");
	println!("{}", m);
	println!("transpose:");
	println!("{}", m.transpose());
	
	
	// Complex 3×2
	let m = Matrix::from([
	    [(1.0, 2.0), (3.0, -1.0)],
	    [(5.0, 4.0), (2.0, 0.0)],
	    [(4.0, 3.0), (6.0, -2.0)],
	]);
	
	println!("\n3×2 complex matrix:");
	println!("{}", m);
	println!("transpose:");
	println!("{}", m.transpose());
}
