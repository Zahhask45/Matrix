pub mod allocator;
pub mod constraint;
pub mod default_allocator;
pub mod dimension;
mod ops;
pub mod storage;

mod alias;
mod array_storage;
#[macro_use]
mod construction;
mod matrix;
mod scalar;


mod vec_storage;

pub use self::matrix::*;
pub use self::scalar::*;

pub use self::default_allocator::*;
pub use self::dimension::*;

pub use self::alias::*;
pub use self::array_storage::*;
pub use self::storage::*;
pub use self::vec_storage::*;
