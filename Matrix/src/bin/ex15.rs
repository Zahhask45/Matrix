use matrix::*;

fn main() {
    println!("=== EX15: Complex Numbers & Multi-Field Tests ===");

    // ----------------------------------------------------
    // 1. Complex Vector Addition, Subtraction & Scaling
    // ----------------------------------------------------
    println!("\n--- 1. Complex Vector Ops (Add, Sub, Scale) ---");
    let mut u_c = Vector::from([(1.0, 2.0), (3.0, -4.0)]); // [(1 + 2i), (3 - 4i)]
    let v_c = Vector::from([(5.0, -1.0), (2.0, 6.0)]);    // [(5 - 1i), (2 + 6i)]

    println!("u_c initial:\n{}", u_c);
    u_c.add(&v_c);
    println!("u_c + v_c:\n{}", u_c);
    // Expected: [6 + 1i], [5 + 2i]

    u_c.sub(&v_c);
    println!("(u_c + v_c) - v_c:\n{}", u_c);
    // Expected back to original: [1 + 2i], [3 - 4i]

    u_c.scl(Complex::new(2.0, 0.0));
    println!("u_c * 2.0:\n{}", u_c);
    // Expected: [2 + 4i], [6 - 8i]

    // ----------------------------------------------------
    // 2. Complex Linear Combination
    // ----------------------------------------------------
    println!("\n--- 2. Complex Linear Combination ---");
    let c1 = Vector::from([(1.0, 0.0), (0.0, 1.0)]); // [(1 + 0i), (0 + 1i)]
    let c2 = Vector::from([(0.0, -1.0), (2.0, 0.0)]); // [(0 - 1i), (2 + 0i)]

    let coefs = [Complex::new(2.0, 1.0), Complex::new(0.0, 3.0)];
    let lin_comb = linear_combination(&[c1, c2], &coefs).unwrap();
    println!("linear_combination([c1, c2], [2+1i, 3i]):\n{}", lin_comb);
    // Calculation:
    // row 0: (2 + 1i)*(1 + 0i) + (3i)*(0 - 1i) = (2 + 1i) + 3 = 5 + 1i
    // row 1: (2 + 1i)*(0 + 1i) + (3i)*(2 + 0i) = (-1 + 2i) + 6i = -1 + 8i

    // ----------------------------------------------------
    // 3. Complex Linear Interpolation (lerp)
    // ----------------------------------------------------
    println!("\n--- 3. Complex Vector & Matrix Lerp ---");
    let v_start = Vector::from([(0.0, 0.0), (10.0, -20.0)]);
    let v_end   = Vector::from([(4.0, 8.0), (0.0, 0.0)]);

    println!("lerp(v_start, v_end, t = 0.0):\n{}", lerp(v_start, v_end, 0.0));
    println!("lerp(v_start, v_end, t = 0.5):\n{}", lerp(v_start, v_end, 0.5));
    println!("lerp(v_start, v_end, t = 1.0):\n{}", lerp(v_start, v_end, 1.0));

    // ----------------------------------------------------
    // 4. Multi-dimensional Real & Complex Tests
    // ----------------------------------------------------
    println!("\n--- 4. Real 3D Vector & Matrix Lerp ---");
    let vec3_a = Vector::from([1.0, 2.0, 3.0]);
    let vec3_b = Vector::from([10.0, 20.0, 30.0]);
    println!("3D Vector lerp (t = 0.25):\n{}", lerp(vec3_a, vec3_b, 0.25));

    let mat_a = Matrix::from([[1.0, 2.0], [3.0, 4.0]]);
    let mat_b = Matrix::from([[5.0, 10.0], [15.0, 20.0]]);
    println!("2x2 Matrix lerp (t = 0.5):\n{}", lerp(mat_a, mat_b, 0.5));


    {
        let mat_a = Matrix::from([[(-70.9, -63.2), (-48.6, -87.2)], [(54.6, 69.2), (-9.4, -62.4)]]);
        let mat_b = Matrix::from([[(-60.5, 94.8), (66.0, -53.1)], [(-47.2, 2.2), (-86.5, -58.8)]]);
        println!("2x2 Matrix lerp (t = 0.3):\n{}", lerp(mat_a, mat_b, 0.3));
    }

    println!("\n=== All Tests Executed Successfully ===");
    
    // ----------------------------------------------------
    // 5. Dot Product of Real & Complex Tests
    // ----------------------------------------------------
    {
        let u = Vector::from([(1.9, 50.4), (-63.9, -35.8)]);
        let v = Vector::from([(-27.0, 10.3), (33.1, 59.4)]);

        println!("{}", u.dot(v));
    }



    
}
