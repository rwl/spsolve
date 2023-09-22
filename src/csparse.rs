use anyhow::{format_err, Result};
use num_traits::ToPrimitive;
use std::alloc::{alloc, Layout};
use suitesparse_sys::{cs_di_lusol, cs_di_sparse};

use crate::Solver;

pub struct CSparse {
    /// Fill-reducing ordering
    /// - `0`: natural,
    /// - `1`: Chol (A+A'),
    /// - `2`: LU (A'*A with no dense rows),
    /// - `3`: QR (A'*A)
    pub order: i32,

    /// Partial pivoting tolerance (`sym ? 0.001 : 1`).
    pub tol: f64,
}

impl Default for CSparse {
    fn default() -> Self {
        Self { order: 2, tol: 1.0 }
    }
}

impl<I> Solver<I, f64> for CSparse
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
        _trans: bool,
    ) -> Result<()> {
        let n = n as i32;
        let mut a_i: Vec<i32> = a_i.iter().map(|i| i.to_i32().unwrap()).collect();
        let mut a_p: Vec<i32> = a_p.iter().map(|i| i.to_i32().unwrap()).collect();
        let mut a_x = a_x.to_vec();
        let nzmax = a_x.len() as i32;

        unsafe {
            let cs = alloc(Layout::new::<cs_di_sparse>()) as *mut cs_di_sparse;
            if cs.is_null() {
                return Err(format_err!("error allocating cs_di_sparse"));
            }
            (*cs).nzmax = i32::max(nzmax, 1);
            (*cs).m = n;
            (*cs).n = n;
            (*cs).p = a_p.as_mut_ptr();
            (*cs).i = a_i.as_mut_ptr();
            (*cs).x = a_x.as_mut_ptr();
            (*cs).nz = -1; // compressed column

            let rv = cs_di_lusol(self.order, cs, b.as_mut_ptr(), self.tol);
            if rv != 1 {
                return Err(format_err!("error calling cs_di_lusol"));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::CSparse;
    use crate::test;
    use anyhow::Result;

    #[test]
    fn test_csparse() -> Result<()> {
        let solver = CSparse::default();
        test::simple_solver_test::<i32, f64, CSparse>(solver)
    }

    #[test]
    fn test_solver() -> Result<()> {
        let mut solver = CSparse::default();
        solver.order = 2;
        // solver.tol = 1e-15;
        test::test_bbus(&solver, 1, 1e-10)
    }
}
