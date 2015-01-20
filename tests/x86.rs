#![cfg(any(target_arch = "x86",
           target_arch = "x86_64"))]

extern crate llvmint;
extern crate simdty;

use simdty::f32x4;

#[test]
fn sse_max() {
    let a = f32x4(1.0, 2.0, 3.0, 4.0);
    let b = f32x4(6.0, -7.0, 8.0, -9.0);
    let c = unsafe { llvmint::x86::sse_max_ps(a, b) };

    assert_eq!(c.0, 6.0);
    assert_eq!(c.1, 2.0);
    assert_eq!(c.2, 8.0);
    assert_eq!(c.3, 4.0);
}

#[test]
fn sse_sqrt() {
    let a = f32x4(0.0, 1.0, 4.0, 9.0);
    let b = unsafe {llvmint::sqrt_v4f32(a)};

    assert_eq!(b.0, 0.0);
    assert_eq!(b.1, 1.0);
    assert_eq!(b.2, 2.0);
    assert_eq!(b.3, 3.0);
}
