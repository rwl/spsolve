use anyhow::{format_err, Result};
use num_traits::ToPrimitive;
use std::alloc::{alloc, Layout};
use suitesparse_sys::{
    klu_analyze, klu_common, klu_defaults, klu_factor, klu_free_numeric, klu_free_symbolic,
    klu_numeric, klu_solve, klu_symbolic, klu_tsolve,
};

use crate::Solver;

pub struct KLU {}

impl Default for KLU {
    fn default() -> Self {
        Self {}
    }
}

impl<I> Solver<I, f64> for KLU
where
    I: ToPrimitive,
{
    fn solve(
        &self,
        n: usize,
        a_i: &[I],
        a_p: &[I],
        a_x: &[f64],
        b: &mut [f64],
        trans: bool,
    ) -> Result<()> {
        let n = n as i32;
        let a_i: Vec<i32> = a_i.iter().map(|i| i.to_i32().unwrap()).collect();
        let a_p: Vec<i32> = a_p.iter().map(|i| i.to_i32().unwrap()).collect();

        unsafe {
            let common = alloc(Layout::new::<klu_common>()) as *mut klu_common;
            if common.is_null() {
                return Err(format_err!("error allocating common"));
            }
            if klu_defaults(common) != 1 {
                return Err(format_err!("error calling klu_defaults"));
            }

            let mut symbolic = klu_analyze(n, a_p.as_ptr(), a_i.as_ptr(), common);
            if symbolic.is_null() {
                return Err(format_err!("error calling klu_analyze"));
            }

            let mut numeric =
                klu_factor(a_p.as_ptr(), a_i.as_ptr(), a_x.as_ptr(), symbolic, common);
            if numeric.is_null() {
                klu_free_symbolic(&mut symbolic as *mut *mut klu_symbolic, common);
                return Err(format_err!("error calling klu_factor"));
            }

            let nrhs = b.len() as i32 / n;
            let rv = if trans {
                klu_tsolve(symbolic, numeric, n, nrhs, b.as_mut_ptr(), common)
            } else {
                klu_solve(symbolic, numeric, n, nrhs, b.as_mut_ptr(), common)
            };
            klu_free_numeric(&mut numeric as *mut *mut klu_numeric, common);
            klu_free_symbolic(&mut symbolic as *mut *mut klu_symbolic, common);
            if rv != 1 {
                return Err(format_err!("error calling klu_solve"));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::KLU;
    use crate::test;
    use anyhow::Result;

    #[test]
    fn simple_test() -> Result<()> {
        let solver = KLU::default();
        test::simple_solver_test::<usize, f64, KLU>(solver)
    }

    #[test]
    #[cfg(feature = "matrix")]
    fn test_solver() -> Result<()> {
        let solver = KLU::default();
        test::test_bbus(&solver, 1, 1e-8)
    }
}
