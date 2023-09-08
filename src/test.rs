use crate::Solver;
use anyhow::Result;
use num_traits::{Float, FromPrimitive, PrimInt};

/// Simple 10x10 matrix test.
///
/// ```txt
///     A = {2.10,     ,     ,     ,     ,     ,     , 0.14, 0.09,     }
///         {    , 1.10,     ,     , 0.06,     ,     ,     ,     , 0.03}
///         {    ,     , 1.70,     ,     ,     ,     ,     ,     , 0.04}
///         {    ,     ,     , 1.00,     ,     , 0.32, 0.19, 0.32, 0.44}
///         {    , 0.06,     ,     , 1.60,     ,     ,     ,     ,     }
///         {    ,     ,     ,     ,     , 2.20,     ,     ,     ,     }
///         {    ,     ,     , 0.32,     ,     , 1.90,     ,     , 0.43}
///         {0.14,     ,     , 0.19,     ,     ,     , 1.10, 0.22,     }
///         {0.09,     ,     , 0.32,     ,     ,     , 0.22, 2.40,     }
///         {    , 0.03, 0.04, 0.44,     ,     , 0.43,     ,     , 3.20}
///
///     b = {0.403, 0.28, 0.55, 1.504, 0.812, 1.32, 1.888, 1.168, 2.473, 3.695}
///
///     x = {0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0}
/// ```
pub fn simple_solver_test<I, F, S>(solver: S) -> Result<()>
where
    I: PrimInt + FromPrimitive,
    F: Float,
    S: Solver<I, F>,
{
    let n: I = I::from(10).unwrap();
    let a_i: Vec<I> = [
        0, 7, 8, 1, 4, 9, 2, 9, 3, 6, 7, 8, 9, 1, 4, 5, 3, 6, 9, 0, 3, 7, 8, 0, 3, 7, 8, 1, 2, 3,
        6, 9,
    ]
    .iter()
    .map(|&i| I::from(i).unwrap())
    .collect();
    let a_p: Vec<I> = [0, 3, 6, 8, 13, 15, 16, 19, 23, 27, 32]
        .iter()
        .map(|&i| I::from(i).unwrap())
        .collect();
    let a_x: Vec<F> = [
        2.1, 0.14, 0.09, 1.1, 0.06, 0.03, 1.7, 0.04, 1.0, 0.32, 0.19, 0.32, 0.44, 0.06, 1.6, 2.2,
        0.32, 1.9, 0.43, 0.14, 0.19, 1.1, 0.22, 0.09, 0.32, 0.22, 2.4, 0.03, 0.04, 0.44, 0.43, 3.2,
    ]
    .iter()
    .map(|&i| F::from(i).unwrap())
    .collect();

    let mut b: Vec<F> = [
        0.403, 0.28, 0.55, 1.504, 0.812, 1.32, 1.888, 1.168, 2.473, 3.695,
    ]
    .iter()
    .map(|&i| F::from(i).unwrap())
    .collect();

    solver.solve(n, &a_i, &a_p, &a_x, &mut b, false)?;

    let x = b;

    (1..=10).zip(x).for_each(|(i, x)| {
        let expect = (i as f64) / 10.0;
        assert!(
            f64::abs(x.to_f64().unwrap() - expect) < 1e-12,
            "x[{}] error",
            i - 1
        );
    });

    Ok(())
}
