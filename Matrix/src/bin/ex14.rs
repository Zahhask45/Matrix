use matrix::{Matrix, Const, ArrayStorage};

fn main() {
    // 90° FOV, square window, near = 0.1, far = 100.0
    let p = Matrix::<f32, Const<4>, Const<4>, ArrayStorage<f32, 4, 4>>::projection(
        90.0_f32.to_radians(),
        1.0,
        0.1,
        100.0,
    );

    println!("90° FOV, ratio 1:1:");
    p.print_col_major();
    println!();


    // 90° FOV, 16:9 window
    let p = Matrix::<f32, Const<4>, Const<4>, ArrayStorage<f32, 4, 4>>::projection(
        90.0_f32.to_radians(),
        16.0 / 9.0,
        0.1,
        100.0,
    );

    println!("90° FOV, ratio 16:9:");
    p.print_col_major();
    println!();


    // 60° FOV, 16:9 window
    let p = Matrix::<f32, Const<4>, Const<4>, ArrayStorage<f32, 4, 4>>::projection(
        60.0_f32.to_radians(),
        16.0 / 9.0,
        0.1,
        100.0,
    );

    println!("60° FOV, ratio 16:9:");
    p.print_col_major();
    println!();


    // 45° FOV, 4:3 window
    let p = Matrix::<f32, Const<4>, Const<4>, ArrayStorage<f32, 4, 4>>::projection(
        45.0_f32.to_radians(),
        4.0 / 3.0,
        0.1,
        100.0,
    );

    println!("45° FOV, ratio 4:3:");
    p.print_col_major();
    println!();


    // Different near/far planes
    let p = Matrix::<f32, Const<4>, Const<4>, ArrayStorage<f32, 4, 4>>::projection(
        90.0_f32.to_radians(),
        1.0,
        1.0,
        10.0,
    );

    println!("90° FOV, ratio 1:1, near = 1, far = 10:");
    p.print_col_major();
}
