use crate::Solver;

pub struct BasicLU {}

impl Default for BasicLU {
    fn default() -> Self {
        Self {}
    }
}

impl Solver<usize, f64> for BasicLU {
    fn solve(
        &self,
        n: usize,
        a_i: &[usize],
        a_p: &[usize],
        a_x: &[f64],
        b: &mut [f64],
        trans: bool,
    ) -> anyhow::Result<()> {
        let a_i: Vec<i64> = a_i.iter().map(|&i| i as i64).collect();
        let a_p: Vec<i64> = a_p.iter().map(|&i| i as i64).collect();

        let mut lu = basiclu::Object::new();
        lu.initialize(n);

        lu.factorize(&a_p, &a_p[1..], &a_i, &a_x);

        lu.solve(&b.to_vec(), b, trans);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::BasicLU;
    use crate::test;
    use anyhow::Result;

    #[test]
    fn simple_test() -> Result<()> {
        let solver = BasicLU::default();
        test::simple_solver_test::<usize, f64, BasicLU>(solver)
    }

    #[test]
    #[cfg(feature = "matrix")]
    fn test_solver() -> Result<()> {
        let solver = BasicLU::default();
        test::test_bbus(&solver, 1, 1e-09)
    }
}
