use matrix::*;

fn main() {
    let u = Matrix::from([
    [1., 0.],
    [0., 1.],
    ]);
    let v = Vector::from([4., 2.]);
    println!("{}", u.mul_vec(&v));
    // [4.]
    // [2.]
    let u = Matrix::from([
    [2., 0.],
    [0., 2.],
    ]);
    let v = Vector::from([4., 2.]);
    println!("{}", u.mul_vec(&v));
    // [8.]
    // [4.]
    let u = Matrix::from([
    [2., -2.],
    [-2., 2.],
    ]);
    let v = Vector::from([4., 2.]);
    println!("{}", u.mul_vec(&v));
    // [4.]
    // [-4.]
    let u = Matrix::from([
    [1., 0.],
    [0., 1.],
    ]);
    
    let v = Matrix::from([
    [1., 0.],
    [0., 1.],
    ]);
    println!("{}", u.mul_mat(&v));
    // [1., 0.]
    // [0., 1.]
    let u = Matrix::from([
    [1., 0.],
    [0., 1.],
    ]);
    let v = Matrix::from([
    [2., 1.],
    [4., 2.],
    ]);
    println!("{}", u.mul_mat(&v));
    // [2., 1.]
    // [4., 2.]
    let u = Matrix::from([
    [3., -5.],
    [6., 8.],
    ]);
    let v = Matrix::from([
    [2., 1.],
    [4., 2.],
    ]);
    println!("{}", u.mul_mat(&v));
    // [-14., -7.]
    // [44., 22.]

    println!("========== COMPLEX MATRIX × VECTOR TESTS ==========");
    
    // Basic complex matrix × vector
    let m = Matrix::from([
        [(1.0, 1.0), (2.0, 0.0)],
        [(3.0, 0.0), (4.0, -1.0)],
    ]);
    
    let v = Vector::from([
        (1.0, 0.0),
        (0.0, 1.0),
    ]);
    
    println!("M =");
    println!("{}", m);
    println!("v =");
    println!("{}", v);
    println!("M × v =");
    println!("{}", m.mul_vec(&v));
    
    
    // Complex multiplication producing real values
    let m = Matrix::from([
        [(1.0, 1.0), (1.0, -1.0)],
        [(2.0, 0.0), (0.0, 2.0)],
    ]);
    
    let v = Vector::from([
        (1.0, 1.0),
        (1.0, -1.0),
    ]);
    
    println!("\nM =");
    println!("{}", m);
    println!("v =");
    println!("{}", v);
    println!("M × v =");
    println!("{}", m.mul_vec(&v));
    
    
    // Identity matrix
    let m = Matrix::from([
        [(1.0, 0.0), (0.0, 0.0)],
        [(0.0, 0.0), (1.0, 0.0)],
    ]);
    
    let v = Vector::from([
        (3.0, 4.0),
        (5.0, -2.0),
    ]);
    
    println!("\nComplex identity × vector:");
    println!("{}", m.mul_vec(&v));
    
    
    // Zero matrix
    let m = Matrix::from([
        [(0.0, 0.0), (0.0, 0.0)],
        [(0.0, 0.0), (0.0, 0.0)],
    ]);
    
    let v = Vector::from([
        (3.0, 4.0),
        (5.0, -2.0),
    ]);
    
    println!("\nComplex zero matrix × vector:");
    println!("{}", m.mul_vec(&v));
    
    
    // 3×2 matrix × 2D complex vector
    let m = Matrix::from([
        [(1.0, 2.0), (3.0, -1.0)],
        [(4.0, 0.0), (1.0, 3.0)],
        [(-2.0, 1.0), (2.0, 0.0)],
    ]);
    
    let v = Vector::from([
        (2.0, -1.0),
        (1.0, 2.0),
    ]);
    
    println!("\n3×2 complex matrix × 2D complex vector:");
    println!("M =");
    println!("{}", m);
    println!("v =");
    println!("{}", v);
    println!("M × v =");
    println!("{}", m.mul_vec(&v));
}

