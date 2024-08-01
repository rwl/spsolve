//! Defines a generic trait for factorizing and solving sparse systems
//! of linear equations.

mod traits;

#[cfg(feature = "gplu")]
pub mod gplu;

#[cfg(feature = "lufact")]
pub mod lufact;

#[cfg(feature = "klu")]
pub mod klu;

#[cfg(feature = "csparse")]
pub mod csparse;

#[cfg(feature = "rsparse")]
pub mod rsparse;

#[cfg(feature = "basiclu")]
pub mod basiclu;

#[cfg(feature = "matrix")]
pub mod matrix;

#[cfg(feature = "rlu")]
pub mod rlu;

#[cfg(test)]
pub mod test;

pub use traits::*;
