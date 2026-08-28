use matrix::{Matrix};

fn main(){
let u = Matrix::from([
    [1., 0., 0.],
    [0., 1., 0.],
    [0., 0., 1.],
]);
println!("{}", u.rank());
// 3

let u = Matrix::from([
    [ 1., 2., 0., 0.],
    [ 2., 4., 0., 0.],
    [-1., 2., 1., 1.],
]);
println!("{}", u.rank());
// 2

let u = Matrix::from([
    [ 8., 5., -2.],
    [ 4., 7., 20.],
    [ 7., 6., 1.],
    [21., 18., 7.],
]);
println!("{}", u.rank());
// 3

// 1x1
let u = Matrix::from([
    [5.],
]);
println!("{}", u.rank());
// 1

let u = Matrix::from([
    [0.],
]);
println!("{}", u.rank());
// 0

// Complex
let u = Matrix::from([
    [(1., 0.), (0., 0.)],
    [(0., 0.), (1., 0.)],
]);
println!("{}", u.rank());
// 2

let u = Matrix::from([
    [(1., 2.), (2., 4.)],
    [(3., 1.), (6., 2.)],
]);
println!("{}", u.rank());
// 1

let u = Matrix::from([
    [(1., 2.), (0., 1.), (3., 0.)],
    [(0., 0.), (1., 1.), (2., 2.)],
    [(2., 4.), (2., 3.), (8., 4.)],
]);
println!("{}", u.rank());
// 3

// Complex 1x1
let u = Matrix::from([
    [(3., 4.)],
]);
println!("{}", u.rank());
// 1

let u = Matrix::from([
    [(0., 0.)],
]);
println!("{}", u.rank());
// 0
}
