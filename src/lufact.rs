use crate::Solver;
use anyhow::{format_err, Result};
use lufact::GP;
use suitesparse_sys::{amd_defaults, amd_order, AMD_CONTROL, AMD_INFO, AMD_OK};

/// Solver based on [AMD](https://crates.io/crates/amd_sys) and [LUFact](https://crates.io/crates/lufact).
pub struct LUFact {
    pub control: Vec<f64>,
    pub gp: GP,
}

impl Default for LUFact {
    fn default() -> Self {
        let mut control = vec![0.0; AMD_CONTROL as usize];
        unsafe {
            amd_defaults(control.as_mut_ptr());
        }
        Self {
            control,
            gp: GP::default(),
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
        let mut gp = self.gp.clone();
        if gp.col_perm.is_none() {
            let mut p = vec![0; n as usize];
            let mut control = self.control.clone();
            let mut info = vec![0.0; AMD_INFO as usize];
            unsafe {
                let rv = amd_order(
                    n as i32,
                    a_p.as_ptr(),
                    a_i.as_ptr(),
                    p.as_mut_ptr(),
                    control.as_mut_ptr(),
                    info.as_mut_ptr(),
                );
                if rv != AMD_OK as i32 {
                    return Err(format_err!("amd error: {}", rv));
                }
            }
            gp.col_perm = Some(p);
        }

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
    fn test_lufact() -> Result<()> {
        let solver = LUFact::default();
        simple_solver_test(solver)
    }
}
