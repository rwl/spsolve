use crate::matrix;
use crate::Solver;

use anyhow::Result;
use approx::assert_abs_diff_eq;
use sparsetools::{csc_matvec, csr_matvec};

pub fn test_bbus(solver: &dyn Solver<usize, f64>, nrhs: usize, epsilon: f64) -> Result<()> {
    let trans = false;
    for (n, a_p, a_i, a_x) in [
        matrix::activsg2000_bbus(!trans),
        matrix::activsg10k_bbus(!trans),
        // matrix::activsg25k_bbus(!trans),
        // matrix::activsg70k_bbus(!trans),
    ] {
        let mut b0 = Vec::default();
        for _ in 0..nrhs {
            b0.extend((0..n).map(|i| 1.0 + i as f64 / n as f64))
        }

        let b = test_solver::<usize, f64>(solver, n, a_i, a_p, a_x, &b0, trans)?;

        for i in 0..n {
            assert_abs_diff_eq!(b[i], b0[i], epsilon = epsilon);
        }
    }
    Ok(())
}

fn test_solver<I, S>(
    solver: &dyn Solver<I, S>,
    n: I,
    a_i: Vec<I>,
    a_p: Vec<I>,
    a_x: Vec<S>,
    x: &Vec<S>,
    trans: bool,
) -> Result<Vec<S>>
where
    I: sparsetools::Integer,
    S: sparsetools::Scalar,
{
    let un = n.to_usize().unwrap();
    let mut b = Vec::<S>::with_capacity(x.len());

    for x_i in x.chunks_exact(un) {
        let mut b_i = vec![S::zero(); n.to_usize().unwrap()];
        if trans {
            csr_matvec(n, n, &a_p, &a_i, &a_x, x_i, &mut b_i);
        } else {
            csc_matvec(n, n, &a_p, &a_i, &a_x, x_i, &mut b_i);
        }

        b.extend(b_i);
    }

    solver.solve(n, &a_i, &a_p, &a_x, &mut b, trans)?;

    Ok(b)
}
