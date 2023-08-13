//! Helper functions to convert between vanilla array and variants.
//! 
//! This is the module-level documentation. 

use core::iter::zip;

/// Substract 1 from each value.
pub fn vanilla_to_m1(from: &[u32], to: &mut [u32]) {
    for (i, j) in zip(from.iter(), to.iter_mut()) {
        assert_ne!(*i, 0_u32, "`m1` must be used on only positive u32");
        *j = *i - 1;
    }
}

/// Same as `vanilla_to_m1()` but modify the original array.
pub fn vanilla_to_m1_self(data: &mut [u32]) {
    for i in data.iter_mut() {
        assert_ne!(*i, 0_u32, "`m1` must be used on only positive u32");
        *i = *i - 1;
    }
}

/// Add 1 to each value.
pub fn m1_to_vanilla(from: &[u32], to: &mut [u32]) {
    for (i, j) in zip(from.iter(), to.iter_mut()) {
        *j = *i + 1;
    }
}

/// Same as `m1_to_vanilla()` but modify the original array.
pub fn m1_to_vanilla_self(data: &mut [u32]) {
    for i in data.iter_mut() {
        *i = *i + 1;
    }
}

/// Transforms the original input into the difference between consecutive values, then zigzag it. If $x > 0$, $zigzag(x) = 2x$, while $x < 0$, $zigzag(x) = -2x - 1$.
pub fn vanilla_to_d1z(from: &[u32], to: &mut [u32]) {
    let mut pre_val = from[0];
    for (i, j) in zip(from.iter(), to.iter_mut()) {
        if *i < pre_val {
            *j = 2 * (pre_val - *i) - 1;
        } else {
            *j = 2 * (*i - pre_val);
        }
        pre_val = *i;
    }
}

/// Same as `vanilla_to_d1z()` but modify the original array.
pub fn vanilla_to_d1z_self(data: &mut [u32]) {
    let mut pre_val = data[0];
    for i in data.iter_mut() {
        let curr_val = *i;
        if curr_val < pre_val {
            *i = 2 * (pre_val - curr_val) - 1;
        } else {
            *i = 2 * (curr_val - pre_val);
        }
        pre_val = curr_val;
    }
}

/// Reverse the delta and zigzag transformation (`vanilla_to_d1z`).
pub fn d1z_to_vanilla(from: &[u32], to: &mut [u32], initial: u32) {
    let mut pre_val = initial;
    for (i, j) in zip(from.iter(), to.iter_mut()) {
        let m = *i % 2;
        let x = *i / 2;
        if m > 0 {
            *j = pre_val - x - 1;
        } else {
            *j = pre_val + x;
        }
        pre_val = *j;
    }
}

/// Same as `d1z_to_vanilla()` but modify the original array.
pub fn d1z_to_vanilla_self(data: &mut [u32], initial: u32) {
    let mut pre_val = initial;
    for i in data.iter_mut() {
        let m = *i % 2;
        let x = *i / 2;
        if m > 0 {
            *i = pre_val - x - 1;
        } else {
            *i = pre_val + x;
        }
        pre_val = *i;
    }
}