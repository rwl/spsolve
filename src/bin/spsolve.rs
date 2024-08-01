use std::time::Instant;

#[cfg(feature = "cpuprofiler")]
use cpuprofiler::PROFILER;

use spsolve::Solver;

fn main() {
    let trans = false;
    // let (n, a_p, a_i, a_x) = spsolve::matrix::activsg2000_bbus(!trans);
    let (n, a_p, a_i, a_x) = spsolve::matrix::activsg10k_bbus(!trans);
    // let (n, a_p, a_i, a_x) = spsolve::matrix::activsg25k_bbus(!trans);
    let mut b: Vec<f64> = (0..n).map(|i| 1.0 + i as f64 / n as f64).collect();

    let solver = spsolve::gplu::GPLU::default();

    let t0 = Instant::now();

    #[cfg(feature = "cpuprofiler")]
    PROFILER.lock().unwrap().start("./spsolve.profile").unwrap();

    solver.solve(n, &a_i, &a_p, &a_x, &mut b, trans).unwrap();

    #[cfg(feature = "cpuprofiler")]
    PROFILER.lock().unwrap().stop().unwrap();

    println!("t = {:#?}", t0.elapsed());
}
