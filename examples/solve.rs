use spsolve::klu::KLU;
use spsolve::lufact::LUFact;
use spsolve::gplu::GPLU;
use spsolve::Solver;

fn main() {
    let rlu_solver = GPLU::default();
    let lufact_solver = LUFact::default();
    let klu_solver = KLU::default();
    let solvers: Vec<&dyn Solver<i32, f64>> = vec![&rlu_solver, &lufact_solver, &klu_solver];

    let n = 10;
    let a_i = vec![
        0, 7, 8, 1, 4, 9, 2, 9, 3, 6, 7, 8, 9, 1, 4, 5, 3, 6, 9, 0, 3, 7, 8, 0, 3, 7, 8, 1, 2, 3,
        6, 9,
    ];
    let a_p = vec![0, 3, 6, 8, 13, 15, 16, 19, 23, 27, 32];
    let a_x = vec![
        2.1, 0.14, 0.09, 1.1, 0.06, 0.03, 1.7, 0.04, 1.0, 0.32, 0.19, 0.32, 0.44, 0.06, 1.6, 2.2,
        0.32, 1.9, 0.43, 0.14, 0.19, 1.1, 0.22, 0.09, 0.32, 0.22, 2.4, 0.03, 0.04, 0.44, 0.43, 3.2,
    ];

    let b = vec![
        0.403, 0.28, 0.55, 1.504, 0.812, 1.32, 1.888, 1.168, 2.473, 3.695,
    ];

    for solver in solvers {
        let mut b = b.clone();
        solver.solve(n, &a_i, &a_p, &a_x, &mut b, false).unwrap();
        println!("x = {:?}", b);
    }
}
