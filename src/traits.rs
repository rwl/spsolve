use anyhow::Result;

/// Solve sparse systems of linear equations.
pub trait Solver<I, S> {
    /// Solve for one or more right-hand-side vectors.
    fn solve(&self, n: I, a_i: &[I], a_p: &[I], a_x: &[S], b: &mut [S], trans: bool) -> Result<()>;
}

/// Factorize and solve sparse systems of linear equations.
pub trait FactorSolver<I, S, F> {
    /// Factorize the input matrix.
    fn factor(&self, n: I, a_i: &[I], a_p: &[I], a_x: &[S]) -> Result<F>;

    /// Solve for one or more right-hand-sides using matrix factors from [`factor`](Solver::factor).
    fn solve(&self, f: &F, b: &mut [S], trans: bool) -> Result<()>;
}
