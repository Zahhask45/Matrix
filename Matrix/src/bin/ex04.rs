use matrix::{Complex, Vector, Matrix};


fn main() {
    // Real vectors
    let u = Vector::from([0.0, 0.0, 0.0]);
    println!(
        "Real [0, 0, 0]       -> norm_1: {}, norm: {}, norm_inf: {}",
        u.norm_1(),
        u.norm(),
        u.norm_inf()
    );
    // 0.0, 0.0, 0.0

    let u = Vector::from([1.0, 2.0, 3.0]);
    println!(
        "Real [1, 2, 3]       -> norm_1: {}, norm: {}, norm_inf: {}",
        u.norm_1(),
        u.norm(),
        u.norm_inf()
    );
    // 6.0, 3.7416575, 3.0

    let u = Vector::from([-1.0, -2.0]);
    println!(
        "Real [-1, -2]        -> norm_1: {}, norm: {}, norm_inf: {}",
        u.norm_1(),
        u.norm(),
        u.norm_inf()
    );
    // 3.0, 2.236068, 2.0


    // Complex vectors
    let u = Vector::from([
        (3.0, 4.0),
        (5.0, 12.0),
        (8.0, 15.0),
    ]);

    println!(
        "Complex [(3,4), (5,12), (8,15)] -> norm_1: {}, norm: {}, norm_inf: {}",
        u.norm_1(),
        u.norm(),
        u.norm_inf()
    );
    // 35.0, 21.977..., 17.0


    let u = Vector::from([
        (1.0, 0.0),
        (0.0, 1.0),
    ]);

    println!(
        "Complex [(1,0), (0,1)] -> norm_1: {}, norm: {}, norm_inf: {}",
        u.norm_1(),
        u.norm(),
        u.norm_inf()
    );
    // 2.0, 1.4142135, 1.0


    let u = Vector::from([
        (-3.0, 4.0),
        (0.0, -5.0),
    ]);

    println!(
        "Complex [(-3,4), (0,-5)] -> norm_1: {}, norm: {}, norm_inf: {}",
        u.norm_1(),
        u.norm(),
        u.norm_inf()
    );
    // 10.0, sqrt(50), 5.0

        println!("========== VECTOR TESTS ==========\n");
    
        // ---------------------------------------------------------
        // Real vectors
        // ---------------------------------------------------------
    
        let v = Vector::from([0.0, 0.0, 0.0]);
        println!("v = [0, 0, 0]");
        println!("  L1   = {}", v.norm_1());
        println!("  L2   = {}", v.norm());
        println!("  Linf = {}", v.norm_inf());
        println!();
    
        let v = Vector::from([1.0, 2.0, 3.0]);
        println!("v = [1, 2, 3]");
        println!("  L1   = {}", v.norm_1());
        println!("  L2   = {}", v.norm());
        println!("  Linf = {}", v.norm_inf());
        println!();
    
        let v = Vector::from([-1.0, -2.0, -3.0]);
        println!("v = [-1, -2, -3]");
        println!("  L1   = {}", v.norm_1());
        println!("  L2   = {}", v.norm());
        println!("  Linf = {}", v.norm_inf());
        println!();
    
        let v = Vector::from([3.0, 4.0]);
        println!("v = [3, 4]");
        println!("  L1   = {}", v.norm_1());
        println!("  L2   = {}", v.norm());
        println!("  Linf = {}", v.norm_inf());
        // Expected: 7, 5, 4
        println!();
    
        let v = Vector::from([-3.0, 4.0]);
        println!("v = [-3, 4]");
        println!("  L1   = {}", v.norm_1());
        println!("  L2   = {}", v.norm());
        println!("  Linf = {}", v.norm_inf());
        // Expected: 7, 5, 4
        println!();
    
        let v = Vector::from([10.0, 1.0, 2.0, 3.0]);
        println!("v = [10, 1, 2, 3]");
        println!("  L1   = {}", v.norm_1());
        println!("  L2   = {}", v.norm());
        println!("  Linf = {}", v.norm_inf());
        // Expected: 16, sqrt(114), 10
        println!();
    
        // ---------------------------------------------------------
        // Complex vectors
        // ---------------------------------------------------------
    
        let v = Vector::from([
            (0.0, 0.0),
            (0.0, 0.0),
            (0.0, 0.0),
        ]);
    
        println!("v = [0, 0, 0] complex");
        println!("  L1   = {}", v.norm_1());
        println!("  L2   = {}", v.norm());
        println!("  Linf = {}", v.norm_inf());
        println!();
    
        let v = Vector::from([
            (3.0, 4.0),
            (5.0, 12.0),
            (8.0, 15.0),
        ]);
    
        println!("v = [3+4i, 5+12i, 8+15i]");
        println!("  L1   = {}", v.norm_1());
        println!("  L2   = {}", v.norm());
        println!("  Linf = {}", v.norm_inf());
        // Expected:
        // L1   = 5 + 13 + 17 = 35
        // L2   = sqrt(25 + 169 + 289) = sqrt(483)
        // Linf = 17
        println!();
    
        let v = Vector::from([
            (1.0, 0.0),
            (0.0, 1.0),
        ]);
    
        println!("v = [1, i]");
        println!("  L1   = {}", v.norm_1());
        println!("  L2   = {}", v.norm());
        println!("  Linf = {}", v.norm_inf());
        // Expected: 2, sqrt(2), 1
        println!();
    
        let v = Vector::from([
            (-3.0, 4.0),
            (0.0, -5.0),
        ]);
    
        println!("v = [-3+4i, -5i]");
        println!("  L1   = {}", v.norm_1());
        println!("  L2   = {}", v.norm());
        println!("  Linf = {}", v.norm_inf());
        // Expected: 10, sqrt(50), 5
        println!();
    
        let v = Vector::from([
            (1.0, 2.0),
            (3.0, 4.0),
            (5.0, 6.0),
        ]);
    
        println!("v = [1+2i, 3+4i, 5+6i]");
        println!("  L1   = {}", v.norm_1());
        println!("  L2   = {}", v.norm());
        println!("  Linf = {}", v.norm_inf());
        // Expected:
        // sqrt(5) + 5 + sqrt(61)
        // sqrt(5 + 25 + 61) = sqrt(91)
        // sqrt(61)
        println!();
    
    
        println!("========== MATRIX TESTS ==========\n");
    
        // ---------------------------------------------------------
        // Real matrices
        // ---------------------------------------------------------
    
        let m = Matrix::from([
            [0.0, 0.0],
            [0.0, 0.0],
        ]);
    
        println!("M =");
        println!("{}", m);
        println!("  L1       = {}", m.l1_norm());
        println!("  L2,1     = {}", m.norm_2_1());
        println!("  Frobenius = {}", m.norm_frobenius());
        println!("  Linf      = {}", m.inf_norm());
        println!();
    
        let m = Matrix::from([
            [1.0, 2.0],
            [3.0, 4.0],
        ]);
    
        println!("M =");
        println!("{}", m);
        println!("  L1       = {}", m.l1_norm());
        println!("  L2,1     = {}", m.norm_2_1());
        println!("  Frobenius = {}", m.norm_frobenius());
        println!("  Linf      = {}", m.inf_norm());
    
        // Expected:
        //
        // L1:
        // 1 + 2 + 3 + 4 = 10
        //
        // L2,1:
        // sqrt(1² + 3²) + sqrt(2² + 4²)
        // = sqrt(10) + sqrt(20)
        //
        // Frobenius:
        // sqrt(1² + 2² + 3² + 4²)
        // = sqrt(30)
        //
        // Linf:
        // 4
        println!();
    
        let m = Matrix::from([
            [-1.0, -2.0],
            [-3.0, -4.0],
        ]);
    
        println!("M =");
        println!("{}", m);
        println!("  L1       = {}", m.l1_norm());
        println!("  L2,1     = {}", m.norm_2_1());
        println!("  Frobenius = {}", m.norm_frobenius());
        println!("  Linf      = {}", m.inf_norm());
        println!();
    
        let m = Matrix::from([
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ]);
    
        println!("M = Identity 3x3");
        println!("  L1       = {}", m.l1_norm());
        println!("  L2,1     = {}", m.norm_2_1());
        println!("  Frobenius = {}", m.norm_frobenius());
        println!("  Linf      = {}", m.inf_norm());
        // Expected:
        // L1 = 3
        // L2,1 = 3
        // Frobenius = sqrt(3)
        // Linf = 1
        println!();
    
        // ---------------------------------------------------------
        // Non-square matrix
        // ---------------------------------------------------------
    
        let m = Matrix::from([
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
        ]);
    
        println!("M = 2x3");
        println!("{}", m);
        println!("  L1       = {}", m.l1_norm());
        println!("  L2,1     = {}", m.norm_2_1());
        println!("  Frobenius = {}", m.norm_frobenius());
        println!("  Linf      = {}", m.inf_norm());
        // Expected:
        // L1 = 21
        // L2,1 = sqrt(17) + sqrt(29) + sqrt(45)
        // Frobenius = sqrt(91)
        // Linf = 6
        println!();
    
        // ---------------------------------------------------------
        // Single row
        // ---------------------------------------------------------
    
        let m = Matrix::from([
            [3.0, 4.0, 12.0],
        ]);
    
        println!("M = single row");
        println!("{}", m);
        println!("  L1       = {}", m.l1_norm());
        println!("  L2,1     = {}", m.norm_2_1());
        println!("  Frobenius = {}", m.norm_frobenius());
        println!("  Linf      = {}", m.inf_norm());
        // Expected:
        // L1 = 19
        // L2,1 = 19
        // Frobenius = 13
        // Linf = 12
        println!();
    
        // ---------------------------------------------------------
        // Single column
        // ---------------------------------------------------------
    
        let m = Matrix::from([
            [3.0],
            [4.0],
            [12.0],
        ]);
    
        println!("M = single column");
        println!("{}", m);
        println!("  L1       = {}", m.l1_norm());
        println!("  L2,1     = {}", m.norm_2_1());
        println!("  Frobenius = {}", m.norm_frobenius());
        println!("  Linf      = {}", m.norm_inf());
        // Expected:
        // L1 = 19
        // L2,1 = 13
        // Frobenius = 13
        // Linf = 12
        println!();
    
    
        println!("========== COMPLEX MATRIX TESTS ==========\n");
    
        // ---------------------------------------------------------
        // Complex 2x2
        // ---------------------------------------------------------
    
        let m = Matrix::from([
            [(1.0, 0.0), (0.0, 1.0)],
            [(3.0, 4.0), (5.0, 12.0)],
        ]);
    
        println!("M =");
        println!("{}", m);
        println!("  L1       = {}", m.l1_norm());
        println!("  L2,1     = {}", m.norm_2_1());
        println!("  Frobenius = {}", m.norm_frobenius());
        println!("  Linf      = {}", m.inf_norm());
        // Magnitudes:
        // 1, 1
        // 5, 13
        //
        // L1 = 20
        //
        // columns:
        // sqrt(1² + 5²) = sqrt(26)
        // sqrt(1² + 13²) = sqrt(170)
        //
        // L2,1 = sqrt(26) + sqrt(170)
        //
        // Frobenius = sqrt(1 + 1 + 25 + 169)
        //            = sqrt(196)
        //            = 14
        //
        // Linf = 13
        println!();
    
        // ---------------------------------------------------------
        // Complex identity
        // ---------------------------------------------------------
    
        let m = Matrix::from([
            [(1.0, 0.0), (0.0, 0.0)],
            [(0.0, 0.0), (1.0, 0.0)],
        ]);
    
        println!("Complex identity:");
        println!("{}", m);
        println!("  L1       = {}", m.l1_norm());
        println!("  L2,1     = {}", m.norm_2_1());
        println!("  Frobenius = {}", m.norm_frobenius());
        println!("  Linf      = {}", m.inf_norm());
        // Expected: 2, 2, sqrt(2), 1
        println!();
    
        // ---------------------------------------------------------
        // Complex values with negative real/imaginary parts
        // ---------------------------------------------------------
    
        let m = Matrix::from([
            [(-3.0, 4.0), (0.0, -5.0)],
            [(5.0, 12.0), (-8.0, -15.0)],
        ]);
    
        println!("Complex matrix with negative components:");
        println!("{}", m);
        println!("  L1       = {}", m.l1_norm());
        println!("  L2,1     = {}", m.norm_2_1());
        println!("  Frobenius = {}", m.norm_frobenius());
        println!("  Linf      = {}", m.inf_norm());
        // Magnitudes:
        // 5, 5
        // 13, 17
        //
        // L1 = 40
        //
        // L2,1 = sqrt(5² + 13²) + sqrt(5² + 17²)
        //       = sqrt(194) + sqrt(314)
        //
        // Frobenius = sqrt(25 + 25 + 169 + 289)
        //            = sqrt(508)
        //
        // Linf = 17
        println!();
}
