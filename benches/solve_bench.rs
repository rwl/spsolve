use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use spsolve::matrix;
use spsolve::Solver;

#[derive(Clone)]
struct Input {
    n: usize,
    a_p: Vec<usize>,
    a_i: Vec<usize>,
    a_x: Vec<f64>,
    trans: bool,
}

fn benchmark_inputs(c: &mut Criterion, group_name: &str, inputs: &[Input]) {
    let mut group = c.benchmark_group(group_name);
    group.sample_size(10);

    for input in inputs.iter() {
        let n = input.n;
        let n_rhs = n / 1;
        let x = (0..n)
            .map(|i| 1.0 + i as f64 / n as f64)
            .collect::<Vec<f64>>();
        let rhs = x.repeat(n_rhs);

        group.throughput(Throughput::Elements(input.n as u64));

        #[cfg(feature = "rlu")]
        group.bench_with_input(BenchmarkId::new("rlu::solve", input.n), input, |b, d| {
            b.iter(|| {
                let mut b = rhs.clone();

                let solver = spsolve::rlu::RLU::default();
                solver
                    .solve(d.n, &d.a_i, &d.a_p, &d.a_x, &mut b, d.trans)
                    .unwrap();
                black_box(b);
            });
        });

        #[cfg(feature = "lufact")]
        group.bench_with_input(BenchmarkId::new("lufact::solve", input.n), input, |b, d| {
            b.iter(|| {
                let mut b = rhs.clone();

                let solver = spsolve::lufact::LUFact::default();
                solver
                    .solve(d.n, &d.a_i, &d.a_p, &d.a_x, &mut b, d.trans)
                    .unwrap();

                black_box(b);
            });
        });

        #[cfg(feature = "csparse")]
        group.bench_with_input(
            BenchmarkId::new("csparse::solve", input.n),
            input,
            |b, d| {
                b.iter(|| {
                    let mut b = rhs.clone();

                    let solver = spsolve::csparse::CSparse::default();
                    solver
                        .solve(d.n, &d.a_i, &d.a_p, &d.a_x, &mut b, d.trans)
                        .unwrap();

                    black_box(b);
                });
            },
        );

        #[cfg(feature = "klu")]
        group.bench_with_input(BenchmarkId::new("klu::solve", input.n), input, |b, d| {
            b.iter(|| {
                let mut b = rhs.clone();

                let solver = spsolve::klu::KLU::default();
                solver
                    .solve(d.n, &d.a_i, &d.a_p, &d.a_x, &mut b, d.trans)
                    .unwrap();

                black_box(b);
            });
        });
    }
    group.finish();
}

pub fn bbus_solve_benchmark(c: &mut Criterion) {
    let trans = false;
    let inputs = [
        matrix::activsg2000_bbus(!trans),
        // matrix::activsg10k_bbus(!trans),
        // matrix::activsg25k_bbus(!trans),
        // matrix::activsg70k_bbus(!trans),
    ]
    .into_iter()
    .map(|(n, a_p, a_i, a_x)| Input {
        n,
        a_p,
        a_i,
        a_x,
        trans,
    })
    .collect::<Vec<Input>>();

    benchmark_inputs(c, "solve(bbus)", &inputs);
}

pub fn jac_solve_benchmark(c: &mut Criterion) {
    let trans = false;
    let inputs = [
        matrix::activsg2000_jac(!trans),
        // matrix::activsg10k_jac(!trans),
        // matrix::activsg25k_jac(!trans),
        // matrix::activsg70k_jac(!trans),
    ]
    .into_iter()
    .map(|(n, a_p, a_i, a_x)| Input {
        n,
        a_p,
        a_i,
        a_x,
        trans,
    })
    .collect::<Vec<Input>>();

    benchmark_inputs(c, "solve(jac)", &inputs);
}

criterion_group!(
    benches,
    bbus_solve_benchmark,
    // ybus_solve_benchmark,
    // jac_solve_benchmark
);
criterion_main!(benches);
