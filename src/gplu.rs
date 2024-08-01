use crate::{FactorSolver, Solver};
use anyhow::{format_err, Result};
use gplu::{Scalar, LU};
use num_traits::{NumAssignOps, PrimInt};
use std::fmt::Display;

/// Solver based on [AMD](https://crates.io/crates/amd) and [GPLU](https://crates.io/crates/gplu).
#[derive(Default)]
pub struct GPLU {
    pub control: amd::Control,
    pub options: gplu::Options,
}

impl<I, S> Solver<I, S> for GPLU
where
    I: PrimInt + NumAssignOps + Display,
    S: Scalar,
{
    fn solve(
        &self,
        n: usize,
        a_i: &[I],
        a_p: &[I],
        a_x: &[S],
        b: &mut [S],
        trans: bool,
    ) -> Result<()> {
        let (p, _p_inv, _info) = amd::order::<I>(I::from(n).unwrap(), &a_p, &a_i, &self.control)
            .map_err(|st| format_err!("amd status: {:?}", st))?;

        let lu = gplu::factor(
            I::from(n).unwrap(),
            &a_i,
            &a_p,
            &a_x,
            Some(&p),
            &self.options,
        )
        .map_err(|err| format_err!("factor error: {}", err))?;

        gplu::solve(&lu, b, trans).map_err(|err| format_err!("solve error: {}", err))?;

        Ok(())
    }
}

impl<I, S> FactorSolver<I, S, LU<S>> for GPLU
where
    I: PrimInt + NumAssignOps + Display,
    S: Scalar,
{
    fn factor(&self, n: usize, a_i: &[I], a_p: &[I], a_x: &[S]) -> Result<LU<S>> {
        let (p, _p_inv, _info) = amd::order::<I>(I::from(n).unwrap(), &a_p, &a_i, &self.control)
            .map_err(|st| format_err!("amd status: {:?}", st))?;

        let lu = gplu::factor(
            I::from(n).unwrap(),
            &a_i,
            &a_p,
            &a_x,
            Some(&p),
            &self.options,
        )
        .map_err(|err| format_err!("factor error: {}", err))?;
        Ok(lu)
    }

    fn solve(&self, f: &LU<S>, b: &mut [S], trans: bool) -> Result<()> {
        gplu::solve(f, b, trans).map_err(|err| format_err!("solve error: {}", err))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::GPLU;
    use crate::test;
    use anyhow::Result;

    #[test]
    fn simple_test() -> Result<()> {
        let solver = GPLU::default();
        test::simple_solver_test::<usize, f64, GPLU>(solver)
    }

    #[test]
    #[cfg(feature = "matrix")]
    fn test_solver() -> Result<()> {
        let solver = GPLU::default();
        test::test_solver_bbus(&solver, 1, 1e-11)
    }

    #[test]
    #[cfg(feature = "matrix")]
    fn test_factor_solver() -> Result<()> {
        let solver = GPLU::default();
        test::test_factor_solver_bbus(&solver, 1, 1e-11)
    }
}
