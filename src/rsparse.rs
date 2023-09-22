use crate::Solver;

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
    ) -> anyhow::Result<()> {
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
    fn test_solver() -> Result<()> {
        let solver = RSparse::default();
        test::test_bbus(&solver, 1, 1e-11)
    }
}
