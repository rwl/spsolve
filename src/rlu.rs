use crate::Solver;
use anyhow::{format_err, Error, Result};
use num_traits::{NumAssignOps, PrimInt};
use rlu::{Scalar, LU};
use std::fmt::Display;

/// Solver based on [AMD](https://crates.io/crates/amd) and [RLU](https://crates.io/crates/rlu).
#[derive(Default)]
pub struct RLU {
    control: amd::Control,
    options: rlu::Options,
}

impl<I, S> Solver<I, S, Vec<I>, LU<S>, Error> for RLU
where
    I: PrimInt + NumAssignOps + Display,
    S: Scalar,
{
    fn permute(&self, n: I, a_i: &[I], a_p: &[I]) -> Result<Option<Vec<I>>> {
        let (p, _p_inv, _info) = amd::order::<I>(n, &a_p, &a_i, &self.control)
            .map_err(|st| format_err!("amd status: {:?}", st))?;
        Ok(Some(p))
    }

    fn factor(&self, n: I, a_i: &[I], a_p: &[I], a_x: &[S], p: Option<&Vec<I>>) -> Result<LU<S>> {
        let lu = rlu::factor(n, &a_i, &a_p, &a_x, p.map(|v| v.as_slice()), &self.options)
            .map_err(|err| format_err!("factor error: {}", err))?;
        Ok(lu)
    }

    fn factor_solve(&self, f: &LU<S>, b: &mut [S], trans: bool) -> Result<()> {
        rlu::solve(f, b, trans).map_err(|err| format_err!("solve error: {}", err))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::RLU;
    use crate::test::simple_test;
    use anyhow::Result;

    #[test]
    fn test_rlu() -> Result<()> {
        let solver = RLU::default();
        simple_test(solver)
    }
}
