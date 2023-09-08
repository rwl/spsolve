//! Defines a generic trait for factorizing and solving sparse systems
//! of linear equations.

mod traits;

#[cfg(feature = "rlu")]
pub mod rlu;

#[cfg(feature = "lufact")]
pub mod amd;
#[cfg(feature = "lufact")]
pub mod lufact;

#[cfg(test)]
pub mod test;

pub use traits::*;
