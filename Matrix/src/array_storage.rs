use std::ops::{Mul};

#[derive(Copy, Clone)]
pub struct ArrayStorage<T, const R: usize, const C: usize>(pub [[T; R]; C]);


