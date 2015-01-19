extern crate llvmint;
extern crate simdty;

use simdty::f32x4;

fn main() {
    let a = f32x4(1.0, 2.0, 3.0, 4.0);
    let b = f32x4(6.0, -7.0, 8.0, -9.0);
    let c = unsafe { llvmint::x86::sse_max_ps(a, b) };

    println!("{:?}", c);
}
