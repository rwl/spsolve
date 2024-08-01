use anyhow::{format_err, Result};

use crate::{FactorSolver, Solver};
use num_traits::NumAssignOps;

use rlu::{lsolve, ltsolve, solve, usolve, utsolve, Int, Matrix, Scalar};

#[derive(Default)]
pub struct RLU {
    pub control: amd::Control,
}

impl<I, S> Solver<I, S> for RLU
where
    I: Int + NumAssignOps,
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
        let (p, _p_inv, _info) = amd::order::<I>(I::from_usize(n), &a_p, &a_i, &self.control)
            .map_err(|st| format_err!("amd status: {:?}", st))?;

        solve(n, &a_i, &a_p, &a_x, Some(&p), b, trans);

        Ok(())
    }
}

impl<I, S> FactorSolver<I, S, (Vec<I>, Matrix<I, S>, Matrix<I, S>, Vec<Option<usize>>)> for RLU
where
    I: Int + NumAssignOps,
    S: Scalar,
{
    fn factor(
        &self,
        n: usize,
        a_i: &[I],
        a_p: &[I],
        a_x: &[S],
    ) -> Result<(Vec<I>, Matrix<I, S>, Matrix<I, S>, Vec<Option<usize>>)> {
        let (cp, _p_inv, _info) = amd::order::<I>(I::from(n).unwrap(), &a_p, &a_i, &self.control)
            .map_err(|st| format_err!("amd status: {:?}", st))?;

        let (l_mat, u_mat, rp) = rlu::lu_decomposition(n, &a_i, &a_p, &a_x, Some(&cp), true);
        Ok((cp, l_mat, u_mat, rp))
    }

    fn solve(
        &self,
        f: &(Vec<I>, Matrix<I, S>, Matrix<I, S>, Vec<Option<usize>>),
        b: &mut [S],
        trans: bool,
    ) -> Result<()> {
        let n = b.len();
        let (cp, l_mat, u_mat, rp) = f;

        let mut x = vec![S::zero(); n];
        for i in 0..n {
            x[rp[i].unwrap()] = b[i];
        }

        if !trans {
            lsolve(&l_mat, &mut x);
            usolve(&u_mat, &mut x);
        } else {
            ltsolve(&l_mat, &mut x);
            utsolve(&u_mat, &mut x);
        }

        for i in 0..n {
            b[cp[i].to_index()] = x[i];
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::RLU;
    use crate::test;
    use anyhow::Result;

    #[test]
    fn simple_test() -> Result<()> {
        let solver = RLU::default();
        test::simple_solver_test::<usize, f64, RLU>(solver)
    }

    #[test]
    #[cfg(feature = "matrix")]
    fn test_solver() -> Result<()> {
        let solver = RLU::default();
        test::test_solver_bbus(&solver, 1, 1e-11)
    }

    #[test]
    #[cfg(feature = "matrix")]
    fn test_factor_solver() -> Result<()> {
        let solver = RLU::default();
        test::test_factor_solver_bbus(&solver, 1, 1e-11)
    }
}
