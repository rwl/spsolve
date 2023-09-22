# spsolve

This crate defines a `Solver` trait for solving sparse systems of linear
equations of the form:

```
    Ax = b
```

The trait is implemented using various open source libraries.
They can be enabled using their associated feature.
The following solvers are currently supported:

- CSparse (C, LGPL)
- BasicLU (C, MIT)
- KLU (C, LGPL)
- RLU (Rust, BSD)
- LUFact (Fortran, Apache/MIT)
- RSparse (Rust, MIT (LGPL?))

A benchmark for comparing solver performance is included.
Test matrix data can be accessed by enabling the `matrix` feature. 
Solvers can be profiled using `cpuprofiler` and [pprof](github.com/google/pprof).

## Benchmarks

DC power flow B matrix (1x RHS):

![lines](/target/criterion/solve(bbus,nrhs=1)/report/lines.svg)

AC power flow Jacobian matrix (1x RHS):

![lines](/target/criterion/solve(jac,nrhs=1)/report/lines.svg)

## License

Licensed under either of the

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0) or
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.