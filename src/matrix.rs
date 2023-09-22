use num_complex::Complex64;

use sprs::io::read_matrix_market;
use sprs::num_kinds::PrimitiveKind;
use sprs::num_matrixmarket::{MatrixMarketConjugate, MatrixMarketRead};

use std::ops::{Add, Neg};
use std::path::PathBuf;

pub fn activsg200_bbus(csc: bool) -> (usize, Vec<usize>, Vec<usize>, Vec<f64>) {
    read_matrix_data("powers", "ACTIVSg200_Bbus", csc)
}

pub fn activsg200_ybus(csc: bool) -> (usize, Vec<usize>, Vec<usize>, Vec<Complex64>) {
    read_matrix_data("powers", "ACTIVSg200_Ybus", csc)
}

pub fn activsg200_jac(csc: bool) -> (usize, Vec<usize>, Vec<usize>, Vec<f64>) {
    read_matrix_data("powers", "ACTIVSg200_Jac", csc)
}

pub fn activsg500_bbus(csc: bool) -> (usize, Vec<usize>, Vec<usize>, Vec<f64>) {
    read_matrix_data("powers", "ACTIVSg500_Bbus", csc)
}

pub fn activsg500_ybus(csc: bool) -> (usize, Vec<usize>, Vec<usize>, Vec<Complex64>) {
    read_matrix_data("powers", "ACTIVSg500_Ybus", csc)
}

pub fn activsg500_jac(csc: bool) -> (usize, Vec<usize>, Vec<usize>, Vec<f64>) {
    read_matrix_data("powers", "ACTIVSg500_Jac", csc)
}

pub fn activsg2000_bbus(csc: bool) -> (usize, Vec<usize>, Vec<usize>, Vec<f64>) {
    read_matrix_data("powers", "ACTIVSg2000_Bbus", csc)
}

pub fn activsg2000_ybus(csc: bool) -> (usize, Vec<usize>, Vec<usize>, Vec<Complex64>) {
    read_matrix_data("powers", "ACTIVSg2000_Ybus", csc)
}

pub fn activsg2000_jac(csc: bool) -> (usize, Vec<usize>, Vec<usize>, Vec<f64>) {
    read_matrix_data("powers", "ACTIVSg2000_Jac", csc)
}

pub fn activsg10k_bbus(csc: bool) -> (usize, Vec<usize>, Vec<usize>, Vec<f64>) {
    read_matrix_data("powers", "ACTIVSg10k_Bbus", csc)
}

pub fn activsg10k_ybus(csc: bool) -> (usize, Vec<usize>, Vec<usize>, Vec<Complex64>) {
    read_matrix_data("powers", "ACTIVSg10k_Ybus", csc)
}

pub fn activsg10k_jac(csc: bool) -> (usize, Vec<usize>, Vec<usize>, Vec<f64>) {
    read_matrix_data("powers", "ACTIVSg10k_Jac", csc)
}

pub fn activsg25k_bbus(csc: bool) -> (usize, Vec<usize>, Vec<usize>, Vec<f64>) {
    read_matrix_data("powers", "ACTIVSg25k_Bbus", csc)
}

pub fn activsg25k_ybus(csc: bool) -> (usize, Vec<usize>, Vec<usize>, Vec<Complex64>) {
    read_matrix_data("powers", "ACTIVSg25k_Ybus", csc)
}

pub fn activsg25k_jac(csc: bool) -> (usize, Vec<usize>, Vec<usize>, Vec<f64>) {
    read_matrix_data("powers", "ACTIVSg25k_Jac", csc)
}

pub fn activsg70k_bbus(csc: bool) -> (usize, Vec<usize>, Vec<usize>, Vec<f64>) {
    read_matrix_data("powers", "ACTIVSg70k_Bbus", csc)
}

pub fn activsg70k_ybus(csc: bool) -> (usize, Vec<usize>, Vec<usize>, Vec<Complex64>) {
    read_matrix_data("powers", "ACTIVSg70k_Ybus", csc)
}

pub fn activsg70k_jac(csc: bool) -> (usize, Vec<usize>, Vec<usize>, Vec<f64>) {
    read_matrix_data("powers", "ACTIVSg70k_Jac", csc)
}

fn read_matrix_data<
    S: Clone
        + PrimitiveKind
        + MatrixMarketRead
        + MatrixMarketConjugate
        + Neg<Output = S>
        + Add<Output = S>,
>(
    subdir: &str,
    name: &str,
    csc: bool,
) -> (usize, Vec<usize>, Vec<usize>, Vec<S>) {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("matrix");
    d.push(subdir);
    d.push(name.to_string() + ".mtx");

    let tri = read_matrix_market(d.to_str().unwrap()).unwrap();

    let a = if csc { tri.to_csc() } else { tri.to_csr() };

    let n = a.cols();
    let a_p = a.indptr().into_raw_storage().to_vec();
    let a_i = a.indices().to_vec();
    let data = a.data().to_vec();

    (n, a_p, a_i, data)
}
