use crate::{FactorSolver, Solver};
use anyhow::{format_err, Result};
use num_traits::{NumAssignOps, PrimInt};
use rlu::{Scalar, LU};
use std::fmt::Display;

/// Solver based on [AMD](https://crates.io/crates/amd) and [RLU](https://crates.io/crates/rlu).
#[derive(Default)]
pub struct RLU {
    pub control: amd::Control,
    pub options: rlu::Options,
}

impl<I, S> Solver<I, S> for RLU
where
    I: PrimInt + NumAssignOps + Display,
    S: Scalar,
{
    fn solve(&self, n: I, a_i: &[I], a_p: &[I], a_x: &[S], b: &mut [S], trans: bool) -> Result<()> {
        let (p, _p_inv, _info) = amd::order::<I>(n, &a_p, &a_i, &self.control)
            .map_err(|st| format_err!("amd status: {:?}", st))?;

        let lu = rlu::factor(n, &a_i, &a_p, &a_x, Some(&p), &self.options)
            .map_err(|err| format_err!("factor error: {}", err))?;

        rlu::solve(&lu, b, trans).map_err(|err| format_err!("solve error: {}", err))?;

        Ok(())
    }
}

impl<I, S> FactorSolver<I, S, LU<S>> for RLU
where
    I: PrimInt + NumAssignOps + Display,
    S: Scalar,
{
    fn factor(&self, n: I, a_i: &[I], a_p: &[I], a_x: &[S]) -> Result<LU<S>> {
        let (p, _p_inv, _info) = amd::order::<I>(n, &a_p, &a_i, &self.control)
            .map_err(|st| format_err!("amd status: {:?}", st))?;

        let lu = rlu::factor(n, &a_i, &a_p, &a_x, Some(&p), &self.options)
            .map_err(|err| format_err!("factor error: {}", err))?;
        Ok(lu)
    }

    fn solve(&self, f: &LU<S>, b: &mut [S], trans: bool) -> Result<()> {
        rlu::solve(f, b, trans).map_err(|err| format_err!("solve error: {}", err))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::RLU;
    use crate::test::simple_solver_test;
    use anyhow::Result;

    #[test]
    fn test_rlu() -> Result<()> {
        let solver = RLU::default();
        simple_solver_test::<usize, f64, RLU>(solver)
    }
}
