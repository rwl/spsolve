use crate::amd;
use crate::Solver;
use anyhow::{format_err, Result};

/// Solver based on [AMD](https://crates.io/crates/amd_sys) and [LUFact](https://crates.io/crates/lufact).
pub struct LUFact {
    pub control: Vec<f64>,
}

impl Default for LUFact {
    fn default() -> Self {
        Self {
            control: amd::defaults(),
        }
    }
}

impl Solver<i32, f64> for LUFact {
    fn solve(
        &self,
        n: i32,
        a_i: &[i32],
        a_p: &[i32],
        a_x: &[f64],
        b: &mut [f64],
        trans: bool,
    ) -> Result<()> {
        let mut control = self.control.clone();

        let (p, _info) = amd::order(n as i32, &a_p, &a_i, &mut control)
            .map_err(|rv| format_err!("amd error: {}", rv))
            .unwrap();

        let mut gp = lufact::GP::default(); // TODO: clone
        gp.col_perm = Some(p);

        let mut a_desc = lufact::CSC {
            m: n,
            n,
            nnz: a_x.len() as i32,
            base: 0,
            colptr: a_p.to_vec(),
            rowind: a_i.to_vec(),
        };

        let mut lu = match lufact::dgstrf(&gp, n, n, &a_x, &mut a_desc) {
            Ok(lu) => lu,
            Err(rv) => {
                return Err(format_err!("dgstrf error: {}", rv));
            }
        };

        let rv = lufact::dgstrs(
            &gp,
            if trans { 'T' } else { 'N' },
            n,
            b.len() as i32 / n,
            &mut lu,
            1,
            1,
            b,
            1,
            1, /*, -1*/
        );
        if rv != 0 {
            return Err(format_err!("dgstrs error: {}", rv));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::LUFact;
    use crate::test::simple_solver_test;
    use anyhow::Result;

    #[test]
    fn test_rlu() -> Result<()> {
        let solver = LUFact::default();
        simple_solver_test(solver)
    }
}
