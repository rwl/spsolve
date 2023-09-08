pub const CONTROL: usize = amd_sys::AMD_CONTROL as usize;
pub const INFO: usize = amd_sys::AMD_INFO as usize;
pub const OK: i32 = amd_sys::AMD_OK as i32;
pub const OK_BUT_JUMBLED: i32 = amd_sys::AMD_OK_BUT_JUMBLED as i32;

pub fn defaults() -> Vec<f64> {
    let mut control = vec![0.0; CONTROL];
    unsafe { amd_sys::amd_defaults(control.as_mut_ptr()) }
    control
}

pub fn control(control: &mut [f64]) {
    unsafe { amd_sys::amd_control(control.as_mut_ptr()) }
}

pub fn info(info: &mut [f64]) {
    unsafe { amd_sys::amd_info(info.as_mut_ptr()) }
}

pub fn order(
    n: i32,
    a_p: &[i32],
    a_i: &[i32],
    control: &mut [f64],
) -> Result<(Vec<i32>, Vec<f64>), i32> {
    let mut p = vec![0; n as usize];
    let mut info = vec![0.0; INFO];
    let rv = unsafe {
        amd_sys::amd_order(
            n,
            a_p.as_ptr(),
            a_i.as_ptr(),
            p.as_mut_ptr(),
            control.as_mut_ptr(),
            info.as_mut_ptr(),
        )
    };
    if rv != OK && rv != OK_BUT_JUMBLED {
        Err(rv)
    } else {
        Ok((p, info))
    }
}

#[cfg(test)]
mod tests {
    //! AMD, Copyright (c) 1996-2022, Timothy A. Davis, Patrick R. Amestoy, and
    //! Iain S. Duff.  All Rights Reserved.
    //! SPDX-License-Identifier: BSD-3-clause

    use crate::amd;

    #[test]
    fn simple() {
        let n = 5;
        let a_p = vec![0, 2, 6, 10, 12, 14];
        let a_i = vec![
            0, 1, // 1st column
            0, 1, 2, 4, // 2nd column
            1, 2, 3, 4, // 3rd column
            2, 3, // 4th column
            1, 4, // 5th column
        ];
        let mut control = amd::defaults();

        let (p, _info) = amd::order(n as i32, &a_p, &a_i, &mut control).unwrap();

        println!("P = {:?}", p);
        // Output:
        //   P = [0, 3, 2, 4, 1]

        // amd::info(&mut info);

        [0, 3, 2, 4, 1].iter().zip(p).for_each(|(&expect, p)| {
            assert_eq!(expect, p);
        });
    }
}
