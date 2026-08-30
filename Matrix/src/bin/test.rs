use matrix::*;

struct TestRunner {
    passed: usize,
    failed: usize,
}

impl TestRunner {
    fn new() -> Self {
        Self {
            passed: 0,
            failed: 0,
        }
    }

    fn suite(&mut self, name: &'static str) {
        println!("\n\x1b[1;34m=== Suite: {} ===\x1b[0m", name);
    }

    fn check(&mut self, name: &str, condition: bool) {
        if condition {
            self.passed += 1;
            println!("  \x1b[32m[PASS]\x1b[0m {}", name);
        } else {
            self.failed += 1;
            println!("  \x1b[31m[FAIL]\x1b[0m {}", name);
        }
    }

    fn check_approx_f32(&mut self, name: &str, actual: f32, expected: f32, eps: f32) {
        let diff = (actual - expected).abs();
        let pass = diff <= eps || (actual.is_nan() && expected.is_nan());
        if !pass {
            println!("         actual: {}, expected: {}, diff: {}", actual, expected, diff);
        }
        self.check(name, pass);
    }

    fn check_approx_complex(&mut self, name: &str, actual: Complex<f32>, expected: Complex<f32>, eps: f32) {
        let diff_re = (actual.re - expected.re).abs();
        let diff_im = (actual.im - expected.im).abs();
        let pass = diff_re <= eps && diff_im <= eps;
        if !pass {
            println!("         actual: {}, expected: {}, diff: (re: {}, im: {})", actual, expected, diff_re, diff_im);
        }
        self.check(name, pass);
    }

    fn summary(&self) {
        println!("\n\x1b[1m========================================\x1b[0m");
        println!("\x1b[1mTest Results Summary:\x1b[0m");
        println!("  Passed: \x1b[32m{}\x1b[0m", self.passed);
        println!("  Failed: \x1b[31m{}\x1b[0m", self.failed);
        println!("  Total:  {}", self.passed + self.failed);
        println!("\x1b[1m========================================\x1b[0m");
        if self.failed == 0 {
            println!("\x1b[1;32mALL TESTS PASSED SUCCESSFULLY! 🚀\x1b[0m\n");
        } else {
            println!("\x1b[1;31mSOME TESTS FAILED!\x1b[0m\n");
            std::process::exit(1);
        }
    }
}

const EPS: f32 = 1e-4;

fn main() {
    let mut runner = TestRunner::new();

    // =========================================================================
    // EX00: Add, Subtract, Scale (Real & Complex Vectors and Matrices)
    // =========================================================================
    runner.suite("EX00: Add, Sub, Scale");
    {
        // Real Vector Add
        let mut u = Vector::from([2.0, 3.0]);
        let v = Vector::from([5.0, 7.0]);
        u.add(&v);
        runner.check_approx_f32("Vector add (2,3)+(5,7) [0]", u.data.0[0][0], 7.0, EPS);
        runner.check_approx_f32("Vector add (2,3)+(5,7) [1]", u.data.0[0][1], 10.0, EPS);

        // Real Vector Sub
        let mut u = Vector::from([2.0, 3.0]);
        let v = Vector::from([5.0, 7.0]);
        u.sub(&v);
        runner.check_approx_f32("Vector sub (2,3)-(5,7) [0]", u.data.0[0][0], -3.0, EPS);
        runner.check_approx_f32("Vector sub (2,3)-(5,7) [1]", u.data.0[0][1], -4.0, EPS);

        // Real Vector Scale
        let mut u = Vector::from([2.0, 3.0]);
        u.scl(2.0);
        runner.check_approx_f32("Vector scl (2,3)*2 [0]", u.data.0[0][0], 4.0, EPS);
        runner.check_approx_f32("Vector scl (2,3)*2 [1]", u.data.0[0][1], 6.0, EPS);

        // Real Matrix Add
        let mut m1 = Matrix::from([[1.0, 2.0], [3.0, 4.0]]);
        let m2 = Matrix::from([[7.0, 4.0], [-2.0, 2.0]]);
        m1.add(&m2);
        runner.check_approx_f32("Matrix add [0][0]", m1.data.0[0][0], 8.0, EPS);
        runner.check_approx_f32("Matrix add [1][0]", m1.data.0[1][0], 6.0, EPS);
        runner.check_approx_f32("Matrix add [0][1]", m1.data.0[0][1], 1.0, EPS);
        runner.check_approx_f32("Matrix add [1][1]", m1.data.0[1][1], 6.0, EPS);

        // Real Matrix Sub
        let mut m1 = Matrix::from([[1.0, 2.0], [3.0, 4.0]]);
        let m2 = Matrix::from([[7.0, 4.0], [-2.0, 2.0]]);
        m1.sub(&m2);
        runner.check_approx_f32("Matrix sub [0][0]", m1.data.0[0][0], -6.0, EPS);
        runner.check_approx_f32("Matrix sub [1][0]", m1.data.0[1][0], -2.0, EPS);
        runner.check_approx_f32("Matrix sub [0][1]", m1.data.0[0][1], 5.0, EPS);
        runner.check_approx_f32("Matrix sub [1][1]", m1.data.0[1][1], 2.0, EPS);

        // Real Matrix Scale
        let mut m = Matrix::from([[1.0, 2.0], [3.0, 4.0]]);
        m.scl(2.0);
        runner.check_approx_f32("Matrix scl [0][0]", m.data.0[0][0], 2.0, EPS);
        runner.check_approx_f32("Matrix scl [1][0]", m.data.0[1][0], 4.0, EPS);
        runner.check_approx_f32("Matrix scl [0][1]", m.data.0[0][1], 6.0, EPS);
        runner.check_approx_f32("Matrix scl [1][1]", m.data.0[1][1], 8.0, EPS);

        // Complex Vector Add & Scale
        let mut uc = Vector::from([(1.0, 2.0), (3.0, -4.0)]);
        let vc = Vector::from([(5.0, -1.0), (2.0, 6.0)]);
        uc.add(&vc);
        runner.check_approx_complex("Complex Vector Add [0]", uc.data.0[0][0], Complex::new(6.0, 1.0), EPS);
        runner.check_approx_complex("Complex Vector Add [1]", uc.data.0[0][1], Complex::new(5.0, 2.0), EPS);

        uc.scl(Complex::new(2.0, 0.0));
        runner.check_approx_complex("Complex Vector Scale [0]", uc.data.0[0][0], Complex::new(12.0, 2.0), EPS);
        runner.check_approx_complex("Complex Vector Scale [1]", uc.data.0[0][1], Complex::new(10.0, 4.0), EPS);
    }

    // =========================================================================
    // EX01: Linear Combination
    // =========================================================================
    runner.suite("EX01: Linear Combination");
    {
        let e1 = Vector::from([1.0, 0.0, 0.0]);
        let e2 = Vector::from([0.0, 1.0, 0.0]);
        let e3 = Vector::from([0.0, 0.0, 1.0]);

        let res = linear_combination(&[e1, e2, e3], &[10.0, -2.0, 0.5]).unwrap();
        runner.check_approx_f32("Basis lin comb [0]", res.data.0[0][0], 10.0, EPS);
        runner.check_approx_f32("Basis lin comb [1]", res.data.0[0][1], -2.0, EPS);
        runner.check_approx_f32("Basis lin comb [2]", res.data.0[0][2], 0.5, EPS);

        let v1 = Vector::from([1.0, 2.0, 3.0]);
        let v2 = Vector::from([0.0, 10.0, -100.0]);
        let res2 = linear_combination(&[v1, v2], &[10.0, -2.0]).unwrap();
        runner.check_approx_f32("Vector lin comb [0]", res2.data.0[0][0], 10.0, EPS);
        runner.check_approx_f32("Vector lin comb [1]", res2.data.0[0][1], 0.0, EPS);
        runner.check_approx_f32("Vector lin comb [2]", res2.data.0[0][2], 230.0, EPS);

        // Complex linear combination
        let c1 = Vector::from([(1.0, 0.0), (0.0, 1.0)]);
        let c2 = Vector::from([(0.0, -1.0), (2.0, 0.0)]);
        let coefs = [Complex::new(2.0, 1.0), Complex::new(0.0, 3.0)];
        let res_c = linear_combination(&[c1, c2], &coefs).unwrap();
        runner.check_approx_complex("Complex lin comb [0]", res_c.data.0[0][0], Complex::new(5.0, 1.0), EPS);
        runner.check_approx_complex("Complex lin comb [1]", res_c.data.0[0][1], Complex::new(-1.0, 8.0), EPS);

        // Error cases
        let empty_mat: [Vector<f32, Const<2>, ArrayStorage<f32, 2, 1>>; 0] = [];
        let empty_coefs: [f32; 0] = [];
        runner.check("Empty linear combination returns error", matches!(linear_combination(&empty_mat, &empty_coefs), Err(LinearCombinationError::Empty)));

        let one_mat = [Vector::from([1.0, 2.0])];
        let two_coefs = [1.0, 2.0];
        runner.check("Length mismatch returns error", matches!(linear_combination(&one_mat, &two_coefs), Err(LinearCombinationError::LengthMismatch { .. })));
    }

    // =========================================================================
    // EX02: Linear Interpolation (lerp)
    // =========================================================================
    runner.suite("EX02: Linear Interpolation (lerp)");
    {
        runner.check_approx_f32("Scalar lerp(0, 1, 0)", lerp(0.0, 1.0, 0.0), 0.0, EPS);
        runner.check_approx_f32("Scalar lerp(0, 1, 1)", lerp(0.0, 1.0, 1.0), 1.0, EPS);
        runner.check_approx_f32("Scalar lerp(0, 1, 0.5)", lerp(0.0, 1.0, 0.5), 0.5, EPS);
        runner.check_approx_f32("Scalar lerp(21, 42, 0.3)", lerp(21.0, 42.0, 0.3), 27.3, EPS);

        let u = Vector::from([2.0, 1.0]);
        let v = Vector::from([4.0, 2.0]);
        let res_v = lerp(u, v, 0.3);
        runner.check_approx_f32("Vector lerp [0]", res_v.data.0[0][0], 2.6, EPS);
        runner.check_approx_f32("Vector lerp [1]", res_v.data.0[0][1], 1.3, EPS);

        let m1 = Matrix::from([[2.0, 1.0], [3.0, 4.0]]);
        let m2 = Matrix::from([[20.0, 10.0], [30.0, 40.0]]);
        let res_m = lerp(m1, m2, 0.5);
        runner.check_approx_f32("Matrix lerp [0][0]", res_m.data.0[0][0], 11.0, EPS);
        runner.check_approx_f32("Matrix lerp [1][0]", res_m.data.0[1][0], 5.5, EPS);
        runner.check_approx_f32("Matrix lerp [0][1]", res_m.data.0[0][1], 16.5, EPS);
        runner.check_approx_f32("Matrix lerp [1][1]", res_m.data.0[1][1], 22.0, EPS);
    }

    // =========================================================================
    // EX03: Dot Product
    // =========================================================================
    runner.suite("EX03: Dot Product");
    {
        let u = Vector::from([0.0, 0.0]);
        let v = Vector::from([1.0, 1.0]);
        runner.check_approx_f32("Zero dot [1,1]", u.dot(v), 0.0, EPS);

        let u = Vector::from([1.0, 1.0]);
        let v = Vector::from([1.0, 1.0]);
        runner.check_approx_f32("[1,1] dot [1,1]", u.dot(v), 2.0, EPS);

        let u = Vector::from([-1.0, 6.0]);
        let v = Vector::from([3.0, 2.0]);
        runner.check_approx_f32("[-1,6] dot [3,2]", u.dot(v), 9.0, EPS);

        // Complex dot product (Hermitian: u . v* )
        let u_c = Vector::from([(1.0, 2.0), (3.0, 4.0)]);
        let v_c = Vector::from([(5.0, 6.0), (7.0, 8.0)]);
        // (1+2i)(5-6i) = 5 - 6i + 10i + 12 = 17 + 4i
        // (3+4i)(7-8i) = 21 - 24i + 28i + 32 = 53 + 4i
        // sum = 70 + 8i
        let dot_c = u_c.dot(v_c);
        runner.check_approx_complex("Complex dot product", dot_c, Complex::new(70.0, 8.0), EPS);
    }

    // =========================================================================
    // EX04: Norms (Vector & Matrix)
    // =========================================================================
    runner.suite("EX04: Norms");
    {
        let v = Vector::from([0.0, 0.0, 0.0]);
        runner.check_approx_f32("Zero vector norm_1", v.norm_1(), 0.0, EPS);
        runner.check_approx_f32("Zero vector norm", v.norm(), 0.0, EPS);
        runner.check_approx_f32("Zero vector norm_inf", v.norm_inf(), 0.0, EPS);

        let v = Vector::from([1.0, 2.0, 3.0]);
        runner.check_approx_f32("Vector [1,2,3] norm_1", v.norm_1(), 6.0, EPS);
        runner.check_approx_f32("Vector [1,2,3] norm", v.norm(), (14.0_f32).sqrt(), EPS);
        runner.check_approx_f32("Vector [1,2,3] norm_inf", v.norm_inf(), 3.0, EPS);

        let v = Vector::from([-1.0, -2.0]);
        runner.check_approx_f32("Vector [-1,-2] norm_1", v.norm_1(), 3.0, EPS);
        runner.check_approx_f32("Vector [-1,-2] norm", v.norm(), (5.0_f32).sqrt(), EPS);
        runner.check_approx_f32("Vector [-1,-2] norm_inf", v.norm_inf(), 2.0, EPS);

        // Complex vector norms: [(3,4), (5,12), (8,15)]
        // Magnitudes: 5, 13, 17
        let vc = Vector::from([(3.0, 4.0), (5.0, 12.0), (8.0, 15.0)]);
        runner.check_approx_f32("Complex vector norm_1 (5+13+17)", vc.norm_1(), 35.0, EPS);
        runner.check_approx_f32("Complex vector norm (sqrt(25+169+289))", vc.norm(), (483.0_f32).sqrt(), EPS);
        runner.check_approx_f32("Complex vector norm_inf", vc.norm_inf(), 17.0, EPS);

        // Matrix norms
        let m = Matrix::from([[1.0, 2.0], [3.0, 4.0]]);
        runner.check_approx_f32("Matrix l1_norm (1+2+3+4)", m.l1_norm(), 10.0, EPS);
        runner.check_approx_f32("Matrix norm_2_1 (sqrt(10)+sqrt(20))", m.norm_2_1(), (10.0_f32).sqrt() + (20.0_f32).sqrt(), EPS);
        runner.check_approx_f32("Matrix norm_frobenius (sqrt(30))", m.norm_frobenius(), (30.0_f32).sqrt(), EPS);
        runner.check_approx_f32("Matrix inf_norm", m.inf_norm(), 4.0, EPS);
    }

    // =========================================================================
    // EX05: Cosine of Angle (angle_cos)
    // =========================================================================
    runner.suite("EX05: Cosine of Angle");
    {
        let u = Vector::from([1.0, 0.0]);
        let v = Vector::from([1.0, 0.0]);
        runner.check_approx_f32("Collinear same direction", angle_cos(&u, &v), 1.0, EPS);

        let u = Vector::from([1.0, 0.0]);
        let v = Vector::from([-1.0, 0.0]);
        runner.check_approx_f32("Collinear opposite direction", angle_cos(&u, &v), -1.0, EPS);

        let u = Vector::from([1.0, 0.0]);
        let v = Vector::from([0.0, 1.0]);
        runner.check_approx_f32("Perpendicular vectors", angle_cos(&u, &v), 0.0, EPS);

        let u = Vector::from([1.0, 0.0]);
        let v = Vector::from([1.0, 1.0]);
        runner.check_approx_f32("45 degree angle", angle_cos(&u, &v), 1.0 / (2.0_f32).sqrt(), EPS);

        let u = Vector::from([1.0, 2.0, 3.0]);
        let v = Vector::from([4.0, 5.0, 6.0]);
        // dot = 4 + 10 + 18 = 32. norm(u) = sqrt(14), norm(v) = sqrt(77)
        let expected = 32.0 / ((14.0_f32).sqrt() * (77.0_f32).sqrt());
        runner.check_approx_f32("[1,2,3] vs [4,5,6]", angle_cos(&u, &v), expected, EPS);
    }

    // =========================================================================
    // EX06: Cross Product
    // =========================================================================
    runner.suite("EX06: Cross Product");
    {
        let i = Vector::from([1.0, 0.0, 0.0]);
        let j = Vector::from([0.0, 1.0, 0.0]);
        let k = cross_product(&i, &j);
        runner.check_approx_f32("i x j = k [0]", k.data.0[0][0], 0.0, EPS);
        runner.check_approx_f32("i x j = k [1]", k.data.0[0][1], 0.0, EPS);
        runner.check_approx_f32("i x j = k [2]", k.data.0[0][2], 1.0, EPS);

        let k_rev = cross_product(&j, &i);
        runner.check_approx_f32("j x i = -k [2]", k_rev.data.0[0][2], -1.0, EPS);

        let u = Vector::from([1.0, 2.0, 3.0]);
        let v = Vector::from([4.0, 5.0, 6.0]);
        let uv = cross_product(&u, &v);
        // [-3, 6, -3]
        runner.check_approx_f32("u x v [0]", uv.data.0[0][0], -3.0, EPS);
        runner.check_approx_f32("u x v [1]", uv.data.0[0][1], 6.0, EPS);
        runner.check_approx_f32("u x v [2]", uv.data.0[0][2], -3.0, EPS);

        // Self cross product must be 0
        let uu = cross_product(&u, &u);
        runner.check_approx_f32("u x u [0]", uu.data.0[0][0], 0.0, EPS);
        runner.check_approx_f32("u x u [1]", uu.data.0[0][1], 0.0, EPS);
        runner.check_approx_f32("u x u [2]", uu.data.0[0][2], 0.0, EPS);
    }

    // =========================================================================
    // EX07: Matrix Multiplication (Matrix-Vector & Matrix-Matrix)
    // =========================================================================
    runner.suite("EX07: Matrix Multiplication");
    {
        let id2 = Matrix::<f32, Const<2>, Const<2>, ArrayStorage<f32, 2, 2>>::identity();
        let v = Vector::from([4.0, 2.0]);
        let res_v = id2.mul_vec(&v);
        runner.check_approx_f32("I * v [0]", res_v.data.0[0][0], 4.0, EPS);
        runner.check_approx_f32("I * v [1]", res_v.data.0[0][1], 2.0, EPS);

        let m1 = Matrix::from([[3.0, -5.0], [6.0, 8.0]]);
        let m2 = Matrix::from([[2.0, 1.0], [4.0, 2.0]]);
        let res_m = m1.mul_mat(&m2);
        // [[-14, -7], [44, 22]]
        runner.check_approx_f32("Matrix mul [0][0]", res_m.data.0[0][0], -14.0, EPS);
        runner.check_approx_f32("Matrix mul [1][0]", res_m.data.0[1][0], -7.0, EPS);
        runner.check_approx_f32("Matrix mul [0][1]", res_m.data.0[0][1], 44.0, EPS);
        runner.check_approx_f32("Matrix mul [1][1]", res_m.data.0[1][1], 22.0, EPS);

        // Rectangular 3x2 * 2x3 -> 3x3
        let a = Matrix::from([[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]]);
        let b = Matrix::from([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
        let ab = a.mul_mat(&b);
        runner.check_approx_f32("Rectangular mul [0][0]", ab.data.0[0][0], 9.0, EPS);
        runner.check_approx_f32("Rectangular mul [1][0]", ab.data.0[1][0], 12.0, EPS);
        runner.check_approx_f32("Rectangular mul [2][0]", ab.data.0[2][0], 15.0, EPS);
        runner.check_approx_f32("Rectangular mul [0][1]", ab.data.0[0][1], 19.0, EPS);
        runner.check_approx_f32("Rectangular mul [1][1]", ab.data.0[1][1], 26.0, EPS);
        runner.check_approx_f32("Rectangular mul [2][1]", ab.data.0[2][1], 33.0, EPS);
        runner.check_approx_f32("Rectangular mul [0][2]", ab.data.0[0][2], 29.0, EPS);
        runner.check_approx_f32("Rectangular mul [1][2]", ab.data.0[1][2], 40.0, EPS);
        runner.check_approx_f32("Rectangular mul [2][2]", ab.data.0[2][2], 51.0, EPS);
    }

    // =========================================================================
    // EX08: Trace
    // =========================================================================
    runner.suite("EX08: Matrix Trace");
    {
        let m1x1 = Matrix::from([[5.0]]);
        runner.check_approx_f32("1x1 trace", m1x1.trace(), 5.0, EPS);

        let m2x2 = Matrix::from([[1.0, 2.0], [3.0, 4.0]]);
        runner.check_approx_f32("2x2 trace (1+4)", m2x2.trace(), 5.0, EPS);

        let m3x3 = Matrix::from([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        runner.check_approx_f32("3x3 trace (1+5+9)", m3x3.trace(), 15.0, EPS);

        let m_neg = Matrix::from([[-2.0, -8.0, 4.0], [1.0, -23.0, 4.0], [0.0, 6.0, 4.0]]);
        runner.check_approx_f32("Negative trace (-2-23+4)", m_neg.trace(), -21.0, EPS);

        // Complex trace
        let m_c = Matrix::from([[(1.0, 2.0), (3.0, 4.0)], [(5.0, -1.0), (6.0, 3.0)]]);
        runner.check_approx_complex("Complex 2x2 trace ((1+2i)+(6+3i))", m_c.trace(), Complex::new(7.0, 5.0), EPS);
    }

    // =========================================================================
    // EX09: Transpose
    // =========================================================================
    runner.suite("EX09: Matrix Transpose");
    {
        let m = Matrix::from([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
        let t = m.transpose();
        runner.check_approx_f32("2x3 transpose [0][0]", t.data.0[0][0], 1.0, EPS);
        runner.check_approx_f32("2x3 transpose [1][0]", t.data.0[1][0], 4.0, EPS);
        runner.check_approx_f32("2x3 transpose [0][1]", t.data.0[0][1], 2.0, EPS);
        runner.check_approx_f32("2x3 transpose [1][1]", t.data.0[1][1], 5.0, EPS);
        runner.check_approx_f32("2x3 transpose [0][2]", t.data.0[0][2], 3.0, EPS);
        runner.check_approx_f32("2x3 transpose [1][2]", t.data.0[1][2], 6.0, EPS);

        // Transpose of transpose is original
        let tt = t.transpose();
        runner.check_approx_f32("(M^T)^T == M [0][0]", tt.data.0[0][0], m.data.0[0][0], EPS);
        runner.check_approx_f32("(M^T)^T == M [1][1]", tt.data.0[1][1], m.data.0[1][1], EPS);
    }

    // =========================================================================
    // EX10: Row Echelon Form (RREF)
    // =========================================================================
    runner.suite("EX10: Row Echelon Form (RREF)");
    {
        let id3 = Matrix::<f32, Const<3>, Const<3>, ArrayStorage<f32, 3, 3>>::identity();
        let rref = id3.row_echelon();
        runner.check("RREF of identity is identity", rref.is_identity());

        let m = Matrix::from([[1.0, 2.0], [3.0, 4.0]]);
        let rref = m.row_echelon();
        runner.check("RREF of invertible 2x2 is identity", rref.is_identity());

        // Linearly dependent rows: [[1, 2], [2, 4]] -> [[1, 2], [0, 0]]
        let m = Matrix::from([[1.0, 2.0], [2.0, 4.0]]);
        let rref = m.row_echelon();
        runner.check_approx_f32("Dependent RREF row 0 col 0", rref.data.0[0][0], 1.0, EPS);
        runner.check_approx_f32("Dependent RREF row 0 col 1", rref.data.0[1][0], 2.0, EPS);
        runner.check_approx_f32("Dependent RREF row 1 col 0", rref.data.0[0][1], 0.0, EPS);
        runner.check_approx_f32("Dependent RREF row 1 col 1", rref.data.0[1][1], 0.0, EPS);

        // Complex RREF
        let mc = Matrix::from([[(1.0, 0.0), (0.0, 0.0)], [(0.0, 0.0), (1.0, 0.0)]]);
        let rref_c = mc.row_echelon();
        runner.check_approx_complex("Complex identity RREF [0][0]", rref_c.data.0[0][0], Complex::new(1.0, 0.0), EPS);
        runner.check_approx_complex("Complex identity RREF [1][1]", rref_c.data.0[1][1], Complex::new(1.0, 0.0), EPS);
    }

    // =========================================================================
    // EX11: Determinant
    // =========================================================================
    runner.suite("EX11: Determinant");
    {
        let m1x1 = Matrix::from([[7.5]]);
        runner.check_approx_f32("1x1 determinant", m1x1.determinant(), 7.5, EPS);

        let m2x2 = Matrix::from([[1.0, -1.0], [-1.0, 1.0]]);
        runner.check_approx_f32("2x2 singular determinant", m2x2.determinant(), 0.0, EPS);

        let m3x3 = Matrix::from([[2.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 2.0]]);
        runner.check_approx_f32("3x3 diagonal 2*I determinant", m3x3.determinant(), 8.0, EPS);

        let m3x3_arb = Matrix::from([[8.0, 5.0, -2.0], [4.0, 7.0, 20.0], [7.0, 6.0, 1.0]]);
        runner.check_approx_f32("3x3 arbitrary determinant", m3x3_arb.determinant(), -174.0, 1e-2);

        let m4x4 = Matrix::from([
            [8.0, 5.0, -2.0, 4.0],
            [4.0, 2.5, 20.0, 4.0],
            [8.0, 5.0, 1.0, 4.0],
            [28.0, -4.0, 17.0, 1.0],
        ]);
        runner.check_approx_f32("4x4 determinant", m4x4.determinant(), 1032.0, 1e-1);

        // Complex 2x2 determinant: (1+i)(2-i) - (2)(3i) = (3+i) - 6i = 3 - 5i
        let mc = Matrix::from([[(1.0, 1.0), (2.0, 0.0)], [(0.0, 3.0), (2.0, -1.0)]]);
        runner.check_approx_complex("Complex 2x2 det", mc.determinant(), Complex::new(3.0, -5.0), 1e-2);
    }

    // =========================================================================
    // EX12: Inverse
    // =========================================================================
    runner.suite("EX12: Matrix Inverse");
    {
        let id3 = Matrix::<f32, Const<3>, Const<3>, ArrayStorage<f32, 3, 3>>::identity();
        let inv_id = id3.inverse().unwrap();
        runner.check("Inverse of identity is identity", inv_id.is_identity());

        let diag = Matrix::from([[2.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 2.0]]);
        let inv_diag = diag.inverse().unwrap();
        runner.check_approx_f32("Inverse diagonal [0][0]", inv_diag.data.0[0][0], 0.5, EPS);
        runner.check_approx_f32("Inverse diagonal [1][1]", inv_diag.data.0[1][1], 0.5, EPS);
        runner.check_approx_f32("Inverse diagonal [2][2]", inv_diag.data.0[2][2], 0.5, EPS);

        // 2x2 inverse
        let m = Matrix::from([[4.0, 7.0], [2.0, 6.0]]);
        let inv = m.inverse().unwrap();
        runner.check_approx_f32("2x2 inv [0][0]", inv.data.0[0][0], 0.6, EPS);
        runner.check_approx_f32("2x2 inv [1][0]", inv.data.0[1][0], -0.7, EPS);
        runner.check_approx_f32("2x2 inv [0][1]", inv.data.0[0][1], -0.2, EPS);
        runner.check_approx_f32("2x2 inv [1][1]", inv.data.0[1][1], 0.4, EPS);

        // Verify A * A^-1 == Identity
        let prod = m.mul_mat(&inv);
        runner.check("A * A^-1 == I", prod.is_identity());

        // Singular matrix fails
        let singular = Matrix::from([[1.0, 2.0], [2.0, 4.0]]);
        runner.check("Singular matrix returns Err", matches!(singular.inverse(), Err(InverseError::SingularMatrix)));
    }

    // =========================================================================
    // EX13: Rank
    // =========================================================================
    runner.suite("EX13: Matrix Rank");
    {
        let id3 = Matrix::<f32, Const<3>, Const<3>, ArrayStorage<f32, 3, 3>>::identity();
        runner.check("Identity 3x3 rank is 3", id3.rank() == 3);

        let zero3 = Matrix::<f32, Const<3>, Const<3>, ArrayStorage<f32, 3, 3>>::default();
        runner.check("Zero 3x3 rank is 0", zero3.rank() == 0);

        let m1 = Matrix::from([
            [1.0, 2.0, 0.0, 0.0],
            [2.0, 4.0, 0.0, 0.0],
            [-1.0, 2.0, 1.0, 1.0],
        ]);
        runner.check("3x4 matrix rank is 2", m1.rank() == 2);

        let m2 = Matrix::from([
            [8.0, 5.0, -2.0],
            [4.0, 7.0, 20.0],
            [7.0, 6.0, 1.0],
            [21.0, 18.0, 7.0],
        ]);
        runner.check("4x3 matrix rank is 3", m2.rank() == 3);

        let m1x1_nonzero = Matrix::from([[5.0]]);
        runner.check("1x1 nonzero rank is 1", m1x1_nonzero.rank() == 1);

        let m1x1_zero = Matrix::from([[0.0]]);
        runner.check("1x1 zero rank is 0", m1x1_zero.rank() == 0);
    }

    // =========================================================================
    // EX14: Projection Matrix
    // =========================================================================
    runner.suite("EX14: Projection Matrix");
    {
        let fov = 90.0_f32.to_radians();
        let ratio = 1.0;
        let near = 0.1;
        let far = 100.0;
        let proj = Matrix::<f32, Const<4>, Const<4>, ArrayStorage<f32, 4, 4>>::projection(fov, ratio, near, far);

        runner.check_approx_f32("Proj [0][0]", proj.data.0[0][0], 1.0, EPS);
        runner.check_approx_f32("Proj [1][1]", proj.data.0[1][1], 1.0, EPS);
        runner.check_approx_f32("Proj [2][2] Z-depth", proj.data.0[2][2], -far / (far - near), EPS);
        runner.check_approx_f32("Proj [3][2] translation", proj.data.0[3][2], -(far * near) / (far - near), EPS);
        runner.check_approx_f32("Proj [2][3] W-clip", proj.data.0[2][3], -1.0, EPS);
    }

    // =========================================================================
    // EX15 / BONUS: Pseudo-Inverse (Moore-Penrose)
    // =========================================================================
    runner.suite("Bonus: Moore-Penrose Pseudo-Inverse");
    {
        // For an invertible square matrix, pseudo-inverse == inverse
        let m = Matrix::from([[4.0, 7.0], [2.0, 6.0]]);
        let pinv = m.pseudo_inverse().unwrap();
        let inv = m.inverse().unwrap();
        runner.check_approx_f32("Pinv == Inv [0][0]", pinv.data.0[0][0], inv.data.0[0][0], EPS);
        runner.check_approx_f32("Pinv == Inv [1][1]", pinv.data.0[1][1], inv.data.0[1][1], EPS);

        // Moore-Penrose Property 1: A * A^+ * A == A
        let a = Matrix::from([[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]]);
        let a_pinv = a.pseudo_inverse().unwrap();
        let a_recon = a.mul_mat(&a_pinv).mul_mat(&a);
        runner.check_approx_f32("Moore-Penrose A*A^+*A == A [0][0]", a_recon.data.0[0][0], a.data.0[0][0], 1e-2);
        runner.check_approx_f32("Moore-Penrose A*A^+*A == A [1][1]", a_recon.data.0[1][1], a.data.0[1][1], 1e-2);
        runner.check_approx_f32("Moore-Penrose A*A^+*A == A [0][2]", a_recon.data.0[0][2], a.data.0[0][2], 1e-2);
    }

    runner.summary();
}
