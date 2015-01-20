extern crate llvmint;
extern crate simdty;

fn main() {
    arch::main();
}

#[cfg(any(target_arch = "x86",
          target_arch = "x86_64"))]
mod arch {
    use simdty::f32x4;

    use llvmint::x86;
    pub fn main() {
        let a = f32x4(1.0, 2.0, 3.0, 4.0);
        let b = f32x4(6.0, -7.0, 8.0, -9.0);
        let c = unsafe {x86::sse_max_ps(a, b)};

        println!("{:?}", c);

        avx();
    }

    #[cfg(not(avx))]
    fn avx() { println!("avx: disabled") }

    #[cfg(avx)]
    fn avx() {
        use simdty::f32x8;
        let a = f32x8(1.0, 3.0, 5.0, 7.0,
                      9.0, 11.0, 13.0, 15.0);
        let b = f32x8(15.0, 13.0, 11.0, 9.0,
                      7.0, 5.0, 3.0, 1.0);

        let c = unsafe {x86::avx_max_ps_256(a, b)};
        println!("avx: {:?}", c)
    }
}

#[cfg(any(target_arch = "arm"))]
mod arch {
    use simdty::f32x4;

    use llvmint::arm;
    pub fn main() {
        let a = f32x4(1.0, 2.0, 3.0, 4.0);
        let b = f32x4(6.0, -7.0, 8.0, -9.0);
        let c = unsafe {arm::neon_vmax_v4f32(a, b)};

        println!("{:?}", c);
    }
}
