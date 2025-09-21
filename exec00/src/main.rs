use matrix::base::*;

fn main() {
    let mat: Matrix3<f64> = Matrix3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
    let mat2: Matrix3<f64> = Matrix3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
    let mat3 = mat + mat2;
    println!("{:?}", mat3);
}
