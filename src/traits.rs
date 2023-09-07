/// Solve sparse systems of linear equations.
pub trait Solver<I: Copy, S, P, F, E> {
    /// Permute the input matrix.
    fn permute(&self, _n: I, _a_i: &[I], _a_p: &[I]) -> Result<Option<P>, E> {
        Ok(None)
    }

    /// Factorize the input matrix, optionally using a permutation from [`permute`](Solver::permute).
    fn factor(&self, n: I, a_i: &[I], a_p: &[I], a_x: &[S], p: Option<&P>) -> Result<F, E>;

    /// Solve for one or more right-hand-sides using matrix factors from [`factor`](Solver::factor).
    fn factor_solve(&self, f: &F, b: &mut [S], trans: bool) -> Result<(), E>;

    /// Solve for one or more right-hand-side vectors.
    fn solve(
        &self,
        n: I,
        a_i: &[I],
        a_p: &[I],
        a_x: &[S],
        b: &mut [S],
        trans: bool,
    ) -> Result<(), E> {
        let p = self.permute(n, a_i, a_p)?;
        let f = self.factor(n, a_i, a_p, a_x, p.as_ref())?;
        self.factor_solve(&f, b, trans)
    }
}
