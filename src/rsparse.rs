use crate::{FactorSolver, Solver};
use anyhow::Result;
use rsparse::data::{Nmrc, Symb};
use rsparse::{lsolve, ltsolve, lu, sqr, usolve, utsolve};

pub struct RSparse {
    /// - -1:natural,
    /// - 0:Cholesky,
    /// - 1:LU,
    /// - 2:QR
    pub order: i8,

    /// Partial pivoting tolerance (`sym ? 0.001 : 1`).
    pub tol: f64,
}

impl Default for RSparse {
    fn default() -> Self {
        Self { order: 1, tol: 1.0 }
    }
}

impl Solver<usize, f64> for RSparse {
    fn solve(
        &self,
        n: usize,
        a_i: &[usize],
        a_p: &[usize],
        a_x: &[f64],
        b: &mut [f64],
        _trans: bool,
    ) -> Result<()> {
        let nzmax = a_x.len();

        let mut a = rsparse::data::Sprs::new();
        a.nzmax = usize::max(nzmax, 1);
        a.m = n;
        a.n = n;
        a.p = a_p.iter().map(|&i| i as isize).collect();
        a.i = a_i.to_vec();
        a.x = a_x.to_vec();

        let mut b_vec = b.to_vec();
        rsparse::lusol(&a, &mut b_vec, self.order, self.tol);
        b.clone_from_slice(&b_vec);

        Ok(())
    }
}

impl FactorSolver<usize, f64, (usize, Symb, Nmrc)> for RSparse {
    fn factor(
        &self,
        n: usize,
        a_i: &[usize],
        a_p: &[usize],
        a_x: &[f64],
    ) -> Result<(usize, Symb, Nmrc)> {
        let nzmax = a_x.len();

        let mut a = rsparse::data::Sprs::new();
        a.nzmax = usize::max(nzmax, 1);
        a.m = n;
        a.n = n;
        a.p = a_p.iter().map(|&i| i as isize).collect();
        a.i = a_i.to_vec();
        a.x = a_x.to_vec();

        let mut s;
        let n;
        s = sqr(&a, self.order, false); // ordering and symbolic analysis
        n = lu(&a, &mut s, self.tol); // numeric LU factorization

        Ok((a.n, s, n))
    }

    fn solve(&self, f: &(usize, Symb, Nmrc), b: &mut [f64], trans: bool) -> Result<()> {
        let (an, s, n) = f;

        let mut x = vec![0.0; *an];

        ipvec(*an, &n.pinv, b, &mut x); // x = P*b
        if !trans {
            lsolve(&n.l, &mut x); // x = L\x
            usolve(&n.u, &mut x); // x = U\x
        } else {
            utsolve(&n.u, &mut x); // x = U'\x
            ltsolve(&n.l, &mut x); // x = L'\x
        }
        ipvec(*an, &s.q, &x, b); // b = Q*x

        Ok(())
    }
}

/// x(P) = b, for dense vectors x and b; P=None denotes identity
///
fn ipvec(n: usize, p: &Option<Vec<isize>>, b: &[f64], x: &mut [f64]) {
    for k in 0..n {
        if p.is_some() {
            x[p.as_ref().unwrap()[k] as usize] = b[k];
        } else {
            x[k] = b[k];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RSparse;
    use crate::test;
    use anyhow::Result;

    #[test]
    fn simple_test() -> Result<()> {
        let solver = RSparse::default();
        test::simple_solver_test::<usize, f64, RSparse>(solver)
    }

    #[test]
    #[cfg(feature = "matrix")]
    fn test_solver() -> Result<()> {
        let solver = RSparse::default();
        test::test_solver_bbus(&solver, 1, 1e-11)
    }

    #[test]
    #[cfg(feature = "matrix")]
    fn test_factor_solver() -> Result<()> {
        let solver = RSparse::default();
        test::test_factor_solver_bbus(&solver, 1, 1e-11)
    }
}
