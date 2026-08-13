use matrix::{Complex, Vector};

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
}
