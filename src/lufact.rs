use crate::Solver;
use anyhow::{format_err, Result};
use lufact::GP;
use num_traits::ToPrimitive;
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

impl<I> Solver<I, f64> for LUFact
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
        let a_i: Vec<i32> = a_i.iter().map(|i| i.to_i32().unwrap()).collect();
        let a_p: Vec<i32> = a_p.iter().map(|i| i.to_i32().unwrap()).collect();

        let mut gp = self.gp.clone();
        if gp.col_perm.is_none() {
            let mut p = vec![0; n];
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
            m: n as i32,
            n: n as i32,
            nnz: a_x.len() as i32,
            base: 0,
            colptr: a_p,
            rowind: a_i,
        };

        let mut lu = match lufact::dgstrf(&gp, n as i32, n as i32, &a_x, &mut a_desc) {
            Ok(lu) => lu,
            Err(rv) => {
                return Err(format_err!("dgstrf error: {}", rv));
            }
        };

        let rv = lufact::dgstrs(
            &gp,
            if trans { 'T' } else { 'N' },
            n as i32,
            (b.len() / n) as i32,
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
        simple_solver_test::<i32, f64, LUFact>(solver)
    }

    #[test]
    #[cfg(feature = "matrix")]
    fn test_solver() -> Result<()> {
        let solver = LUFact::default();
        crate::test::test_bbus(&solver, 1, 1e-11)
    }
}
