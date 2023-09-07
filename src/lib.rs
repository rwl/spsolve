//! Defines a generic trait for factorizing and solving sparse systems
//! of linear equations.

mod traits;

#[cfg(feature = "rlu")]
pub mod rlu;

#[cfg(test)]
pub mod test;

pub use traits::*;
