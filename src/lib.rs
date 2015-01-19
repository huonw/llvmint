#![feature(simd, simd_ffi, link_llvm_intrinsics)]
#![allow(non_snake_case)]

//! Bindings to (almost) all LLVM intrinsics.
//!
//! Intrinsics are categorised into modules by the architecture on
//! which they are supported (however, see [Platform
//! support][#platform-support] for a caveat), with certain intrinsics
//! available at the top level. These are raw bindings and absolutely
//! zero error checking is performed.
//!
//! # Naming
//!
//! The intrinsics are named entirely for their internal LLVM names,
//! with the `llvm` prefix stripped, `.` replaced by `_` and, if it
//! exists, a platform prefix replaced by being placed in a module of
//! that name. For example, `llvm.adjust.trampoline` becomes
//! `::adjust_trampoline` and `llvm.x86.addcarry.u32` becomes
//! `::x86::addcarry_u32`.
//!
//! # Platform support
//!
//! An intrinsic being available in a certain module (or at the top
//! level) does not guarantee that it is supported on all CPUs of that
//! architecture (resp. all CPUs), e.g. the x86::avx512 are only
//! supported on the very latest x86-64 CPUs, not on older x86
//! processors.
//!
//! Using an intrinsic in a configuration that is not supported will
//! likely cause LLVM assertions or general badness along those lines.

extern crate simdty;

extern {
    #[link_name = "llvm.adjust.trampoline"]
    pub fn adjust_trampoline(a: *mut i8) -> *mut i8;
    #[link_name = "llvm.assume"]
    pub fn assume(a: bool) -> ();
    #[link_name = "llvm.clear.cache"]
    pub fn clear_cache(a: *mut i8, b: *mut i8) -> ();
    #[link_name = "llvm.debugtrap"]
    pub fn debugtrap() -> ();
    #[link_name = "llvm.donothing"]
    pub fn donothing() -> ();
    #[link_name = "llvm.eh.dwarf.cfa"]
    pub fn eh_dwarf_cfa(a: i32) -> *mut i8;
    #[link_name = "llvm.eh.return.i32"]
    pub fn eh_return_i32(a: i32, b: *mut i8) -> ();
    #[link_name = "llvm.eh.return.i64"]
    pub fn eh_return_i64(a: i64, b: *mut i8) -> ();
    #[link_name = "llvm.eh.sjlj.callsite"]
    pub fn eh_sjlj_callsite(a: i32) -> ();
    #[link_name = "llvm.eh.sjlj.functioncontext"]
    pub fn eh_sjlj_functioncontext(a: *mut i8) -> ();
    #[link_name = "llvm.eh.sjlj.longjmp"]
    pub fn eh_sjlj_longjmp(a: *mut i8) -> ();
    #[link_name = "llvm.eh.sjlj.lsda"]
    pub fn eh_sjlj_lsda() -> *mut i8;
    #[link_name = "llvm.eh.sjlj.setjmp"]
    pub fn eh_sjlj_setjmp(a: *mut i8) -> i32;
    #[link_name = "llvm.eh.typeid.for"]
    pub fn eh_typeid_for(a: *mut i8) -> i32;
    #[link_name = "llvm.eh.unwind.init"]
    pub fn eh_unwind_init() -> ();
    #[link_name = "llvm.experimental.patchpoint.i64"]
    pub fn experimental_patchpoint_i64(a: i64, b: i32, c: *mut i8, d: i32, ...) -> i64;
    #[link_name = "llvm.experimental.patchpoint.void"]
    pub fn experimental_patchpoint_void(a: i64, b: i32, c: *mut i8, d: i32, ...) -> ();
    #[link_name = "llvm.experimental.stackmap"]
    pub fn experimental_stackmap(a: i64, b: i32, ...) -> ();
    #[link_name = "llvm.flt.rounds"]
    pub fn flt_rounds() -> i32;
    #[link_name = "llvm.frameaddress"]
    pub fn frameaddress(a: i32) -> *mut i8;
    #[link_name = "llvm.gcread"]
    pub fn gcread(a: *mut i8, b: *mut *mut i8) -> *mut i8;
    #[link_name = "llvm.gcroot"]
    pub fn gcroot(a: *mut *mut i8, b: *mut i8) -> ();
    #[link_name = "llvm.gcwrite"]
    pub fn gcwrite(a: *mut i8, b: *mut i8, c: *mut *mut i8) -> ();
    #[link_name = "llvm.init.trampoline"]
    pub fn init_trampoline(a: *mut i8, b: *mut i8, c: *mut i8) -> ();
    #[link_name = "llvm.lifetime.end"]
    pub fn lifetime_end(a: i64, b: *mut i8) -> ();
    #[link_name = "llvm.lifetime.start"]
    pub fn lifetime_start(a: i64, b: *mut i8) -> ();
    #[link_name = "llvm.longjmp"]
    pub fn longjmp(a: *mut i8, b: i32) -> ();
    #[link_name = "llvm.pcmarker"]
    pub fn pcmarker(a: i32) -> ();
    #[link_name = "llvm.prefetch"]
    pub fn prefetch(a: *mut i8, b: i32, c: i32, d: i32) -> ();
    #[link_name = "llvm.readcyclecounter"]
    pub fn readcyclecounter() -> i64;
    #[link_name = "llvm.returnaddress"]
    pub fn returnaddress(a: i32) -> *mut i8;
    #[link_name = "llvm.setjmp"]
    pub fn setjmp(a: *mut i8) -> i32;
    #[link_name = "llvm.siglongjmp"]
    pub fn siglongjmp(a: *mut i8, b: i32) -> ();
    #[link_name = "llvm.sigsetjmp"]
    pub fn sigsetjmp(a: *mut i8, b: i32) -> i32;
    #[link_name = "llvm.stackprotector"]
    pub fn stackprotector(a: *mut i8, b: *mut *mut i8) -> ();
    #[link_name = "llvm.stackprotectorcheck"]
    pub fn stackprotectorcheck(a: *mut *mut i8) -> ();
    #[link_name = "llvm.stackrestore"]
    pub fn stackrestore(a: *mut i8) -> ();
    #[link_name = "llvm.stacksave"]
    pub fn stacksave() -> *mut i8;
    #[link_name = "llvm.trap"]
    pub fn trap() -> ();
    #[link_name = "llvm.vacopy"]
    pub fn vacopy(a: *mut i8, b: *mut i8) -> ();
    #[link_name = "llvm.vaend"]
    pub fn vaend(a: *mut i8) -> ();
    #[link_name = "llvm.var.annotation"]
    pub fn var_annotation(a: *mut i8, b: *mut i8, c: *mut i8, d: i32) -> ();
    #[link_name = "llvm.vastart"]
    pub fn vastart(a: *mut i8) -> ();
}
/// LLVM intrinsics for the AMDGPU architecture.
pub mod AMDGPU {
    extern {
    }
}
/// LLVM intrinsics for the aarch64 architecture.
pub mod aarch64 {
    extern {
        #[link_name = "llvm.aarch64.clrex"]
        pub fn clrex() -> ();
        #[link_name = "llvm.aarch64.crc32b"]
        pub fn crc32b(a: i32, b: i32) -> i32;
        #[link_name = "llvm.aarch64.crc32cb"]
        pub fn crc32cb(a: i32, b: i32) -> i32;
        #[link_name = "llvm.aarch64.crc32ch"]
        pub fn crc32ch(a: i32, b: i32) -> i32;
        #[link_name = "llvm.aarch64.crc32cw"]
        pub fn crc32cw(a: i32, b: i32) -> i32;
        #[link_name = "llvm.aarch64.crc32cx"]
        pub fn crc32cx(a: i32, b: i64) -> i32;
        #[link_name = "llvm.aarch64.crc32h"]
        pub fn crc32h(a: i32, b: i32) -> i32;
        #[link_name = "llvm.aarch64.crc32w"]
        pub fn crc32w(a: i32, b: i32) -> i32;
        #[link_name = "llvm.aarch64.crc32x"]
        pub fn crc32x(a: i32, b: i64) -> i32;
        #[link_name = "llvm.aarch64.crypto.aesd"]
        pub fn crypto_aesd(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.aarch64.crypto.aese"]
        pub fn crypto_aese(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.aarch64.crypto.aesimc"]
        pub fn crypto_aesimc(a: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.aarch64.crypto.aesmc"]
        pub fn crypto_aesmc(a: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.aarch64.crypto.sha1c"]
        pub fn crypto_sha1c(a: ::simdty::i32x4, b: i32, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.aarch64.crypto.sha1h"]
        pub fn crypto_sha1h(a: i32) -> i32;
        #[link_name = "llvm.aarch64.crypto.sha1m"]
        pub fn crypto_sha1m(a: ::simdty::i32x4, b: i32, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.aarch64.crypto.sha1p"]
        pub fn crypto_sha1p(a: ::simdty::i32x4, b: i32, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.aarch64.crypto.sha1su0"]
        pub fn crypto_sha1su0(a: ::simdty::i32x4, b: ::simdty::i32x4, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.aarch64.crypto.sha1su1"]
        pub fn crypto_sha1su1(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.aarch64.crypto.sha256h"]
        pub fn crypto_sha256h(a: ::simdty::i32x4, b: ::simdty::i32x4, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.aarch64.crypto.sha256h2"]
        pub fn crypto_sha256h2(a: ::simdty::i32x4, b: ::simdty::i32x4, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.aarch64.crypto.sha256su0"]
        pub fn crypto_sha256su0(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.aarch64.crypto.sha256su1"]
        pub fn crypto_sha256su1(a: ::simdty::i32x4, b: ::simdty::i32x4, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.aarch64.dmb"]
        pub fn dmb(a: i32) -> ();
        #[link_name = "llvm.aarch64.dsb"]
        pub fn dsb(a: i32) -> ();
        #[link_name = "llvm.aarch64.hint"]
        pub fn hint(a: i32) -> ();
        #[link_name = "llvm.aarch64.isb"]
        pub fn isb(a: i32) -> ();
        #[link_name = "llvm.aarch64.ldaxr"]
        pub fn ldaxr(a: *mut i8) -> i64;
        #[link_name = "llvm.aarch64.ldxr"]
        pub fn ldxr(a: *mut i8) -> i64;
        #[link_name = "llvm.aarch64.neon.pmull64"]
        pub fn neon_pmull64(a: i64, b: i64) -> ::simdty::i8x16;
        #[link_name = "llvm.aarch64.neon.sqdmulls.scalar"]
        pub fn neon_sqdmulls_scalar(a: i32, b: i32) -> i64;
        #[link_name = "llvm.aarch64.neon.vcvtfp2hf"]
        pub fn neon_vcvtfp2hf(a: ::simdty::f32x4) -> ::simdty::i16x4;
        #[link_name = "llvm.aarch64.neon.vcvthf2fp"]
        pub fn neon_vcvthf2fp(a: ::simdty::i16x4) -> ::simdty::f32x4;
        #[link_name = "llvm.aarch64.sisd.fcvtxn"]
        pub fn sisd_fcvtxn(a: f64) -> f32;
        #[link_name = "llvm.aarch64.stlxp"]
        pub fn stlxp(a: i64, b: i64, c: *mut i8) -> i32;
        #[link_name = "llvm.aarch64.stlxr"]
        pub fn stlxr(a: i64, b: *mut i8) -> i32;
        #[link_name = "llvm.aarch64.stxp"]
        pub fn stxp(a: i64, b: i64, c: *mut i8) -> i32;
        #[link_name = "llvm.aarch64.stxr"]
        pub fn stxr(a: i64, b: *mut i8) -> i32;
    }
}
/// LLVM intrinsics for the arm architecture.
pub mod arm {
    extern {
        #[link_name = "llvm.arm.cdp"]
        pub fn cdp(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> ();
        #[link_name = "llvm.arm.cdp2"]
        pub fn cdp2(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> ();
        #[link_name = "llvm.arm.clrex"]
        pub fn clrex() -> ();
        #[link_name = "llvm.arm.crc32b"]
        pub fn crc32b(a: i32, b: i32) -> i32;
        #[link_name = "llvm.arm.crc32cb"]
        pub fn crc32cb(a: i32, b: i32) -> i32;
        #[link_name = "llvm.arm.crc32ch"]
        pub fn crc32ch(a: i32, b: i32) -> i32;
        #[link_name = "llvm.arm.crc32cw"]
        pub fn crc32cw(a: i32, b: i32) -> i32;
        #[link_name = "llvm.arm.crc32h"]
        pub fn crc32h(a: i32, b: i32) -> i32;
        #[link_name = "llvm.arm.crc32w"]
        pub fn crc32w(a: i32, b: i32) -> i32;
        #[link_name = "llvm.arm.dbg"]
        pub fn dbg(a: i32) -> ();
        #[link_name = "llvm.arm.dmb"]
        pub fn dmb(a: i32) -> ();
        #[link_name = "llvm.arm.dsb"]
        pub fn dsb(a: i32) -> ();
        #[link_name = "llvm.arm.get.fpscr"]
        pub fn get_fpscr() -> i32;
        #[link_name = "llvm.arm.hint"]
        pub fn hint(a: i32) -> ();
        #[link_name = "llvm.arm.isb"]
        pub fn isb(a: i32) -> ();
        #[link_name = "llvm.arm.ldaex"]
        pub fn ldaex(a: *mut i8) -> i32;
        #[link_name = "llvm.arm.ldrex"]
        pub fn ldrex(a: *mut i8) -> i32;
        #[link_name = "llvm.arm.mcr"]
        pub fn mcr(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> ();
        #[link_name = "llvm.arm.mcr2"]
        pub fn mcr2(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> ();
        #[link_name = "llvm.arm.mcrr"]
        pub fn mcrr(a: i32, b: i32, c: i32, d: i32, e: i32) -> ();
        #[link_name = "llvm.arm.mcrr2"]
        pub fn mcrr2(a: i32, b: i32, c: i32, d: i32, e: i32) -> ();
        #[link_name = "llvm.arm.mrc"]
        pub fn mrc(a: i32, b: i32, c: i32, d: i32, e: i32) -> i32;
        #[link_name = "llvm.arm.mrc2"]
        pub fn mrc2(a: i32, b: i32, c: i32, d: i32, e: i32) -> i32;
        #[link_name = "llvm.arm.neon.aesd"]
        pub fn neon_aesd(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.arm.neon.aese"]
        pub fn neon_aese(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.arm.neon.aesimc"]
        pub fn neon_aesimc(a: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.arm.neon.aesmc"]
        pub fn neon_aesmc(a: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.arm.neon.sha1c"]
        pub fn neon_sha1c(a: ::simdty::i32x4, b: i32, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.arm.neon.sha1h"]
        pub fn neon_sha1h(a: i32) -> i32;
        #[link_name = "llvm.arm.neon.sha1m"]
        pub fn neon_sha1m(a: ::simdty::i32x4, b: i32, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.arm.neon.sha1p"]
        pub fn neon_sha1p(a: ::simdty::i32x4, b: i32, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.arm.neon.sha1su0"]
        pub fn neon_sha1su0(a: ::simdty::i32x4, b: ::simdty::i32x4, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.arm.neon.sha1su1"]
        pub fn neon_sha1su1(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.arm.neon.sha256h"]
        pub fn neon_sha256h(a: ::simdty::i32x4, b: ::simdty::i32x4, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.arm.neon.sha256h2"]
        pub fn neon_sha256h2(a: ::simdty::i32x4, b: ::simdty::i32x4, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.arm.neon.sha256su0"]
        pub fn neon_sha256su0(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.arm.neon.sha256su1"]
        pub fn neon_sha256su1(a: ::simdty::i32x4, b: ::simdty::i32x4, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.arm.neon.vcvtfp2hf"]
        pub fn neon_vcvtfp2hf(a: ::simdty::f32x4) -> ::simdty::i16x4;
        #[link_name = "llvm.arm.neon.vcvthf2fp"]
        pub fn neon_vcvthf2fp(a: ::simdty::i16x4) -> ::simdty::f32x4;
        #[link_name = "llvm.arm.neon.vtbl1"]
        pub fn neon_vtbl1(a: ::simdty::i8x8, b: ::simdty::i8x8) -> ::simdty::i8x8;
        #[link_name = "llvm.arm.neon.vtbl2"]
        pub fn neon_vtbl2(a: ::simdty::i8x8, b: ::simdty::i8x8, c: ::simdty::i8x8) -> ::simdty::i8x8;
        #[link_name = "llvm.arm.neon.vtbl3"]
        pub fn neon_vtbl3(a: ::simdty::i8x8, b: ::simdty::i8x8, c: ::simdty::i8x8, d: ::simdty::i8x8) -> ::simdty::i8x8;
        #[link_name = "llvm.arm.neon.vtbl4"]
        pub fn neon_vtbl4(a: ::simdty::i8x8, b: ::simdty::i8x8, c: ::simdty::i8x8, d: ::simdty::i8x8, e: ::simdty::i8x8) -> ::simdty::i8x8;
        #[link_name = "llvm.arm.neon.vtbx1"]
        pub fn neon_vtbx1(a: ::simdty::i8x8, b: ::simdty::i8x8, c: ::simdty::i8x8) -> ::simdty::i8x8;
        #[link_name = "llvm.arm.neon.vtbx2"]
        pub fn neon_vtbx2(a: ::simdty::i8x8, b: ::simdty::i8x8, c: ::simdty::i8x8, d: ::simdty::i8x8) -> ::simdty::i8x8;
        #[link_name = "llvm.arm.neon.vtbx3"]
        pub fn neon_vtbx3(a: ::simdty::i8x8, b: ::simdty::i8x8, c: ::simdty::i8x8, d: ::simdty::i8x8, e: ::simdty::i8x8) -> ::simdty::i8x8;
        #[link_name = "llvm.arm.neon.vtbx4"]
        pub fn neon_vtbx4(a: ::simdty::i8x8, b: ::simdty::i8x8, c: ::simdty::i8x8, d: ::simdty::i8x8, e: ::simdty::i8x8, f: ::simdty::i8x8) -> ::simdty::i8x8;
        #[link_name = "llvm.arm.qadd"]
        pub fn qadd(a: i32, b: i32) -> i32;
        #[link_name = "llvm.arm.qsub"]
        pub fn qsub(a: i32, b: i32) -> i32;
        #[link_name = "llvm.arm.rbit"]
        pub fn rbit(a: i32) -> i32;
        #[link_name = "llvm.arm.set.fpscr"]
        pub fn set_fpscr(a: i32) -> ();
        #[link_name = "llvm.arm.ssat"]
        pub fn ssat(a: i32, b: i32) -> i32;
        #[link_name = "llvm.arm.stlex"]
        pub fn stlex(a: i32, b: *mut i8) -> i32;
        #[link_name = "llvm.arm.stlexd"]
        pub fn stlexd(a: i32, b: i32, c: *mut i8) -> i32;
        #[link_name = "llvm.arm.strex"]
        pub fn strex(a: i32, b: *mut i8) -> i32;
        #[link_name = "llvm.arm.strexd"]
        pub fn strexd(a: i32, b: i32, c: *mut i8) -> i32;
        #[link_name = "llvm.arm.thread.pointer"]
        pub fn thread_pointer() -> *mut i8;
        #[link_name = "llvm.arm.undefined"]
        pub fn undefined(a: i32) -> ();
        #[link_name = "llvm.arm.usat"]
        pub fn usat(a: i32, b: i32) -> i32;
    }
}
/// LLVM intrinsics for the cuda architecture.
pub mod cuda {
    extern {
        #[link_name = "llvm.cuda.syncthreads"]
        pub fn syncthreads() -> ();
    }
}
/// LLVM intrinsics for the hexagon architecture.
pub mod hexagon {
    extern {
        #[link_name = "llvm.hexagon.A2.abs"]
        pub fn A2_abs(a: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.absp"]
        pub fn A2_absp(a: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.abssat"]
        pub fn A2_abssat(a: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.add"]
        pub fn A2_add(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.addh.h16.hh"]
        pub fn A2_addh_h16_hh(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.addh.h16.hl"]
        pub fn A2_addh_h16_hl(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.addh.h16.lh"]
        pub fn A2_addh_h16_lh(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.addh.h16.ll"]
        pub fn A2_addh_h16_ll(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.addh.h16.sat.hh"]
        pub fn A2_addh_h16_sat_hh(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.addh.h16.sat.hl"]
        pub fn A2_addh_h16_sat_hl(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.addh.h16.sat.lh"]
        pub fn A2_addh_h16_sat_lh(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.addh.h16.sat.ll"]
        pub fn A2_addh_h16_sat_ll(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.addh.l16.hl"]
        pub fn A2_addh_l16_hl(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.addh.l16.ll"]
        pub fn A2_addh_l16_ll(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.addh.l16.sat.hl"]
        pub fn A2_addh_l16_sat_hl(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.addh.l16.sat.ll"]
        pub fn A2_addh_l16_sat_ll(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.addi"]
        pub fn A2_addi(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.addp"]
        pub fn A2_addp(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.addpsat"]
        pub fn A2_addpsat(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.addsat"]
        pub fn A2_addsat(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.addsp"]
        pub fn A2_addsp(a: i32, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.and"]
        pub fn A2_and(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.andir"]
        pub fn A2_andir(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.andp"]
        pub fn A2_andp(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.aslh"]
        pub fn A2_aslh(a: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.asrh"]
        pub fn A2_asrh(a: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.combine.hh"]
        pub fn A2_combine_hh(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.combine.hl"]
        pub fn A2_combine_hl(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.combine.lh"]
        pub fn A2_combine_lh(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.combine.ll"]
        pub fn A2_combine_ll(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.combineii"]
        pub fn A2_combineii(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.A2.combinew"]
        pub fn A2_combinew(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.A2.max"]
        pub fn A2_max(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.maxp"]
        pub fn A2_maxp(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.maxu"]
        pub fn A2_maxu(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.maxup"]
        pub fn A2_maxup(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.min"]
        pub fn A2_min(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.minp"]
        pub fn A2_minp(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.minu"]
        pub fn A2_minu(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.minup"]
        pub fn A2_minup(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.neg"]
        pub fn A2_neg(a: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.negp"]
        pub fn A2_negp(a: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.negsat"]
        pub fn A2_negsat(a: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.not"]
        pub fn A2_not(a: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.notp"]
        pub fn A2_notp(a: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.or"]
        pub fn A2_or(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.orir"]
        pub fn A2_orir(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.orp"]
        pub fn A2_orp(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.roundsat"]
        pub fn A2_roundsat(a: i64) -> i32;
        #[link_name = "llvm.hexagon.A2.sat"]
        pub fn A2_sat(a: i64) -> i32;
        #[link_name = "llvm.hexagon.A2.satb"]
        pub fn A2_satb(a: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.sath"]
        pub fn A2_sath(a: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.satub"]
        pub fn A2_satub(a: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.satuh"]
        pub fn A2_satuh(a: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.sub"]
        pub fn A2_sub(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.subh.h16.hh"]
        pub fn A2_subh_h16_hh(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.subh.h16.hl"]
        pub fn A2_subh_h16_hl(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.subh.h16.lh"]
        pub fn A2_subh_h16_lh(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.subh.h16.ll"]
        pub fn A2_subh_h16_ll(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.subh.h16.sat.hh"]
        pub fn A2_subh_h16_sat_hh(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.subh.h16.sat.hl"]
        pub fn A2_subh_h16_sat_hl(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.subh.h16.sat.lh"]
        pub fn A2_subh_h16_sat_lh(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.subh.h16.sat.ll"]
        pub fn A2_subh_h16_sat_ll(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.subh.l16.hl"]
        pub fn A2_subh_l16_hl(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.subh.l16.ll"]
        pub fn A2_subh_l16_ll(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.subh.l16.sat.hl"]
        pub fn A2_subh_l16_sat_hl(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.subh.l16.sat.ll"]
        pub fn A2_subh_l16_sat_ll(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.subp"]
        pub fn A2_subp(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.subri"]
        pub fn A2_subri(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.subsat"]
        pub fn A2_subsat(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.svaddh"]
        pub fn A2_svaddh(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.svaddhs"]
        pub fn A2_svaddhs(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.svadduhs"]
        pub fn A2_svadduhs(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.svavgh"]
        pub fn A2_svavgh(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.svavghs"]
        pub fn A2_svavghs(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.svnavgh"]
        pub fn A2_svnavgh(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.svsubh"]
        pub fn A2_svsubh(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.svsubhs"]
        pub fn A2_svsubhs(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.svsubuhs"]
        pub fn A2_svsubuhs(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.swiz"]
        pub fn A2_swiz(a: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.sxtb"]
        pub fn A2_sxtb(a: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.sxth"]
        pub fn A2_sxth(a: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.sxtw"]
        pub fn A2_sxtw(a: i32) -> i64;
        #[link_name = "llvm.hexagon.A2.tfr"]
        pub fn A2_tfr(a: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.tfrih"]
        pub fn A2_tfrih(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.tfril"]
        pub fn A2_tfril(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.tfrp"]
        pub fn A2_tfrp(a: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.tfrpi"]
        pub fn A2_tfrpi(a: i32) -> i64;
        #[link_name = "llvm.hexagon.A2.tfrsi"]
        pub fn A2_tfrsi(a: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.vabsh"]
        pub fn A2_vabsh(a: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vabshsat"]
        pub fn A2_vabshsat(a: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vabsw"]
        pub fn A2_vabsw(a: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vabswsat"]
        pub fn A2_vabswsat(a: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vaddb.map"]
        pub fn A2_vaddb_map(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vaddh"]
        pub fn A2_vaddh(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vaddhs"]
        pub fn A2_vaddhs(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vaddub"]
        pub fn A2_vaddub(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vaddubs"]
        pub fn A2_vaddubs(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vadduhs"]
        pub fn A2_vadduhs(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vaddw"]
        pub fn A2_vaddw(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vaddws"]
        pub fn A2_vaddws(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vavgh"]
        pub fn A2_vavgh(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vavghcr"]
        pub fn A2_vavghcr(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vavghr"]
        pub fn A2_vavghr(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vavgub"]
        pub fn A2_vavgub(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vavgubr"]
        pub fn A2_vavgubr(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vavguh"]
        pub fn A2_vavguh(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vavguhr"]
        pub fn A2_vavguhr(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vavguw"]
        pub fn A2_vavguw(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vavguwr"]
        pub fn A2_vavguwr(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vavgw"]
        pub fn A2_vavgw(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vavgwcr"]
        pub fn A2_vavgwcr(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vavgwr"]
        pub fn A2_vavgwr(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vcmpbeq"]
        pub fn A2_vcmpbeq(a: i64, b: i64) -> bool;
        #[link_name = "llvm.hexagon.A2.vcmpbgtu"]
        pub fn A2_vcmpbgtu(a: i64, b: i64) -> bool;
        #[link_name = "llvm.hexagon.A2.vcmpheq"]
        pub fn A2_vcmpheq(a: i64, b: i64) -> bool;
        #[link_name = "llvm.hexagon.A2.vcmphgt"]
        pub fn A2_vcmphgt(a: i64, b: i64) -> bool;
        #[link_name = "llvm.hexagon.A2.vcmphgtu"]
        pub fn A2_vcmphgtu(a: i64, b: i64) -> bool;
        #[link_name = "llvm.hexagon.A2.vcmpweq"]
        pub fn A2_vcmpweq(a: i64, b: i64) -> bool;
        #[link_name = "llvm.hexagon.A2.vcmpwgt"]
        pub fn A2_vcmpwgt(a: i64, b: i64) -> bool;
        #[link_name = "llvm.hexagon.A2.vcmpwgtu"]
        pub fn A2_vcmpwgtu(a: i64, b: i64) -> bool;
        #[link_name = "llvm.hexagon.A2.vconj"]
        pub fn A2_vconj(a: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vmaxb"]
        pub fn A2_vmaxb(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vmaxh"]
        pub fn A2_vmaxh(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vmaxub"]
        pub fn A2_vmaxub(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vmaxuh"]
        pub fn A2_vmaxuh(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vmaxuw"]
        pub fn A2_vmaxuw(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vmaxw"]
        pub fn A2_vmaxw(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vminb"]
        pub fn A2_vminb(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vminh"]
        pub fn A2_vminh(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vminub"]
        pub fn A2_vminub(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vminuh"]
        pub fn A2_vminuh(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vminuw"]
        pub fn A2_vminuw(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vminw"]
        pub fn A2_vminw(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vnavgh"]
        pub fn A2_vnavgh(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vnavghcr"]
        pub fn A2_vnavghcr(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vnavghr"]
        pub fn A2_vnavghr(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vnavgw"]
        pub fn A2_vnavgw(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vnavgwcr"]
        pub fn A2_vnavgwcr(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vnavgwr"]
        pub fn A2_vnavgwr(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vraddub"]
        pub fn A2_vraddub(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vraddub.acc"]
        pub fn A2_vraddub_acc(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vrsadub"]
        pub fn A2_vrsadub(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vrsadub.acc"]
        pub fn A2_vrsadub_acc(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vsubb.map"]
        pub fn A2_vsubb_map(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vsubh"]
        pub fn A2_vsubh(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vsubhs"]
        pub fn A2_vsubhs(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vsubub"]
        pub fn A2_vsubub(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vsububs"]
        pub fn A2_vsububs(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vsubuhs"]
        pub fn A2_vsubuhs(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vsubw"]
        pub fn A2_vsubw(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.vsubws"]
        pub fn A2_vsubws(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.xor"]
        pub fn A2_xor(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.xorp"]
        pub fn A2_xorp(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A2.zxtb"]
        pub fn A2_zxtb(a: i32) -> i32;
        #[link_name = "llvm.hexagon.A2.zxth"]
        pub fn A2_zxth(a: i32) -> i32;
        #[link_name = "llvm.hexagon.A4.andn"]
        pub fn A4_andn(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A4.andnp"]
        pub fn A4_andnp(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A4.bitsplit"]
        pub fn A4_bitsplit(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.A4.bitspliti"]
        pub fn A4_bitspliti(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.A4.boundscheck"]
        pub fn A4_boundscheck(a: i32, b: i64) -> bool;
        #[link_name = "llvm.hexagon.A4.cmpbeq"]
        pub fn A4_cmpbeq(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.A4.cmpbeqi"]
        pub fn A4_cmpbeqi(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.A4.cmpbgt"]
        pub fn A4_cmpbgt(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.A4.cmpbgti"]
        pub fn A4_cmpbgti(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.A4.cmpbgtu"]
        pub fn A4_cmpbgtu(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.A4.cmpbgtui"]
        pub fn A4_cmpbgtui(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.A4.cmpheq"]
        pub fn A4_cmpheq(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.A4.cmpheqi"]
        pub fn A4_cmpheqi(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.A4.cmphgt"]
        pub fn A4_cmphgt(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.A4.cmphgti"]
        pub fn A4_cmphgti(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.A4.cmphgtu"]
        pub fn A4_cmphgtu(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.A4.cmphgtui"]
        pub fn A4_cmphgtui(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.A4.combineir"]
        pub fn A4_combineir(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.A4.combineri"]
        pub fn A4_combineri(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.A4.cround.ri"]
        pub fn A4_cround_ri(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A4.cround.rr"]
        pub fn A4_cround_rr(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A4.modwrapu"]
        pub fn A4_modwrapu(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A4.orn"]
        pub fn A4_orn(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A4.ornp"]
        pub fn A4_ornp(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.A4.rcmpeq"]
        pub fn A4_rcmpeq(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A4.rcmpeqi"]
        pub fn A4_rcmpeqi(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A4.rcmpneq"]
        pub fn A4_rcmpneq(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A4.rcmpneqi"]
        pub fn A4_rcmpneqi(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A4.round.ri"]
        pub fn A4_round_ri(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A4.round.ri.sat"]
        pub fn A4_round_ri_sat(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A4.round.rr"]
        pub fn A4_round_rr(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A4.round.rr.sat"]
        pub fn A4_round_rr_sat(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.A4.tlbmatch"]
        pub fn A4_tlbmatch(a: i64, b: i32) -> bool;
        #[link_name = "llvm.hexagon.A4.vcmpbeq.any"]
        pub fn A4_vcmpbeq_any(a: i64, b: i64) -> bool;
        #[link_name = "llvm.hexagon.A4.vcmpbeqi"]
        pub fn A4_vcmpbeqi(a: i64, b: i32) -> bool;
        #[link_name = "llvm.hexagon.A4.vcmpbgt"]
        pub fn A4_vcmpbgt(a: i64, b: i64) -> bool;
        #[link_name = "llvm.hexagon.A4.vcmpbgti"]
        pub fn A4_vcmpbgti(a: i64, b: i32) -> bool;
        #[link_name = "llvm.hexagon.A4.vcmpbgtui"]
        pub fn A4_vcmpbgtui(a: i64, b: i32) -> bool;
        #[link_name = "llvm.hexagon.A4.vcmpheqi"]
        pub fn A4_vcmpheqi(a: i64, b: i32) -> bool;
        #[link_name = "llvm.hexagon.A4.vcmphgti"]
        pub fn A4_vcmphgti(a: i64, b: i32) -> bool;
        #[link_name = "llvm.hexagon.A4.vcmphgtui"]
        pub fn A4_vcmphgtui(a: i64, b: i32) -> bool;
        #[link_name = "llvm.hexagon.A4.vcmpweqi"]
        pub fn A4_vcmpweqi(a: i64, b: i32) -> bool;
        #[link_name = "llvm.hexagon.A4.vcmpwgti"]
        pub fn A4_vcmpwgti(a: i64, b: i32) -> bool;
        #[link_name = "llvm.hexagon.A4.vcmpwgtui"]
        pub fn A4_vcmpwgtui(a: i64, b: i32) -> bool;
        #[link_name = "llvm.hexagon.A4.vrmaxh"]
        pub fn A4_vrmaxh(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.A4.vrmaxuh"]
        pub fn A4_vrmaxuh(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.A4.vrmaxuw"]
        pub fn A4_vrmaxuw(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.A4.vrmaxw"]
        pub fn A4_vrmaxw(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.A4.vrminh"]
        pub fn A4_vrminh(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.A4.vrminuh"]
        pub fn A4_vrminuh(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.A4.vrminuw"]
        pub fn A4_vrminuw(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.A4.vrminw"]
        pub fn A4_vrminw(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.A5.vaddhubs"]
        pub fn A5_vaddhubs(a: i64, b: i64) -> i32;
        #[link_name = "llvm.hexagon.C2.all8"]
        pub fn C2_all8(a: i32) -> bool;
        #[link_name = "llvm.hexagon.C2.and"]
        pub fn C2_and(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.C2.andn"]
        pub fn C2_andn(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.C2.any8"]
        pub fn C2_any8(a: i32) -> bool;
        #[link_name = "llvm.hexagon.C2.bitsclr"]
        pub fn C2_bitsclr(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.C2.bitsclri"]
        pub fn C2_bitsclri(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.C2.bitsset"]
        pub fn C2_bitsset(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.C2.cmpeq"]
        pub fn C2_cmpeq(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.C2.cmpeqi"]
        pub fn C2_cmpeqi(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.C2.cmpeqp"]
        pub fn C2_cmpeqp(a: i64, b: i64) -> bool;
        #[link_name = "llvm.hexagon.C2.cmpgei"]
        pub fn C2_cmpgei(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.C2.cmpgeui"]
        pub fn C2_cmpgeui(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.C2.cmpgt"]
        pub fn C2_cmpgt(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.C2.cmpgti"]
        pub fn C2_cmpgti(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.C2.cmpgtp"]
        pub fn C2_cmpgtp(a: i64, b: i64) -> bool;
        #[link_name = "llvm.hexagon.C2.cmpgtu"]
        pub fn C2_cmpgtu(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.C2.cmpgtui"]
        pub fn C2_cmpgtui(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.C2.cmpgtup"]
        pub fn C2_cmpgtup(a: i64, b: i64) -> bool;
        #[link_name = "llvm.hexagon.C2.cmplt"]
        pub fn C2_cmplt(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.C2.cmpltu"]
        pub fn C2_cmpltu(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.C2.mask"]
        pub fn C2_mask(a: i32) -> i64;
        #[link_name = "llvm.hexagon.C2.mux"]
        pub fn C2_mux(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.C2.muxii"]
        pub fn C2_muxii(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.C2.muxir"]
        pub fn C2_muxir(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.C2.muxri"]
        pub fn C2_muxri(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.C2.not"]
        pub fn C2_not(a: i32) -> bool;
        #[link_name = "llvm.hexagon.C2.or"]
        pub fn C2_or(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.C2.orn"]
        pub fn C2_orn(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.C2.pxfer.map"]
        pub fn C2_pxfer_map(a: i32) -> bool;
        #[link_name = "llvm.hexagon.C2.tfrpr"]
        pub fn C2_tfrpr(a: i32) -> i32;
        #[link_name = "llvm.hexagon.C2.tfrrp"]
        pub fn C2_tfrrp(a: i32) -> bool;
        #[link_name = "llvm.hexagon.C2.vitpack"]
        pub fn C2_vitpack(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.C2.vmux"]
        pub fn C2_vmux(a: i32, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.C2.xor"]
        pub fn C2_xor(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.C4.and.and"]
        pub fn C4_and_and(a: i32, b: i32, c: i32) -> bool;
        #[link_name = "llvm.hexagon.C4.and.andn"]
        pub fn C4_and_andn(a: i32, b: i32, c: i32) -> bool;
        #[link_name = "llvm.hexagon.C4.and.or"]
        pub fn C4_and_or(a: i32, b: i32, c: i32) -> bool;
        #[link_name = "llvm.hexagon.C4.and.orn"]
        pub fn C4_and_orn(a: i32, b: i32, c: i32) -> bool;
        #[link_name = "llvm.hexagon.C4.cmplte"]
        pub fn C4_cmplte(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.C4.cmpltei"]
        pub fn C4_cmpltei(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.C4.cmplteu"]
        pub fn C4_cmplteu(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.C4.cmplteui"]
        pub fn C4_cmplteui(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.C4.cmpneq"]
        pub fn C4_cmpneq(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.C4.cmpneqi"]
        pub fn C4_cmpneqi(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.C4.fastcorner9"]
        pub fn C4_fastcorner9(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.C4.fastcorner9.not"]
        pub fn C4_fastcorner9_not(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.C4.nbitsclr"]
        pub fn C4_nbitsclr(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.C4.nbitsclri"]
        pub fn C4_nbitsclri(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.C4.nbitsset"]
        pub fn C4_nbitsset(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.C4.or.and"]
        pub fn C4_or_and(a: i32, b: i32, c: i32) -> bool;
        #[link_name = "llvm.hexagon.C4.or.andn"]
        pub fn C4_or_andn(a: i32, b: i32, c: i32) -> bool;
        #[link_name = "llvm.hexagon.C4.or.or"]
        pub fn C4_or_or(a: i32, b: i32, c: i32) -> bool;
        #[link_name = "llvm.hexagon.C4.or.orn"]
        pub fn C4_or_orn(a: i32, b: i32, c: i32) -> bool;
        #[link_name = "llvm.hexagon.F2.conv.d2df"]
        pub fn F2_conv_d2df(a: i64) -> f64;
        #[link_name = "llvm.hexagon.F2.conv.d2sf"]
        pub fn F2_conv_d2sf(a: i64) -> f32;
        #[link_name = "llvm.hexagon.F2.conv.df2d"]
        pub fn F2_conv_df2d(a: f64) -> i64;
        #[link_name = "llvm.hexagon.F2.conv.df2d.chop"]
        pub fn F2_conv_df2d_chop(a: f64) -> i64;
        #[link_name = "llvm.hexagon.F2.conv.df2sf"]
        pub fn F2_conv_df2sf(a: f64) -> f32;
        #[link_name = "llvm.hexagon.F2.conv.df2ud"]
        pub fn F2_conv_df2ud(a: f64) -> i64;
        #[link_name = "llvm.hexagon.F2.conv.df2ud.chop"]
        pub fn F2_conv_df2ud_chop(a: f64) -> i64;
        #[link_name = "llvm.hexagon.F2.conv.df2uw"]
        pub fn F2_conv_df2uw(a: f64) -> i32;
        #[link_name = "llvm.hexagon.F2.conv.df2uw.chop"]
        pub fn F2_conv_df2uw_chop(a: f64) -> i32;
        #[link_name = "llvm.hexagon.F2.conv.df2w"]
        pub fn F2_conv_df2w(a: f64) -> i32;
        #[link_name = "llvm.hexagon.F2.conv.df2w.chop"]
        pub fn F2_conv_df2w_chop(a: f64) -> i32;
        #[link_name = "llvm.hexagon.F2.conv.sf2d"]
        pub fn F2_conv_sf2d(a: f32) -> i64;
        #[link_name = "llvm.hexagon.F2.conv.sf2d.chop"]
        pub fn F2_conv_sf2d_chop(a: f32) -> i64;
        #[link_name = "llvm.hexagon.F2.conv.sf2df"]
        pub fn F2_conv_sf2df(a: f32) -> f64;
        #[link_name = "llvm.hexagon.F2.conv.sf2ud"]
        pub fn F2_conv_sf2ud(a: f32) -> i64;
        #[link_name = "llvm.hexagon.F2.conv.sf2ud.chop"]
        pub fn F2_conv_sf2ud_chop(a: f32) -> i64;
        #[link_name = "llvm.hexagon.F2.conv.sf2uw"]
        pub fn F2_conv_sf2uw(a: f32) -> i32;
        #[link_name = "llvm.hexagon.F2.conv.sf2uw.chop"]
        pub fn F2_conv_sf2uw_chop(a: f32) -> i32;
        #[link_name = "llvm.hexagon.F2.conv.sf2w"]
        pub fn F2_conv_sf2w(a: f32) -> i32;
        #[link_name = "llvm.hexagon.F2.conv.sf2w.chop"]
        pub fn F2_conv_sf2w_chop(a: f32) -> i32;
        #[link_name = "llvm.hexagon.F2.conv.ud2df"]
        pub fn F2_conv_ud2df(a: i64) -> f64;
        #[link_name = "llvm.hexagon.F2.conv.ud2sf"]
        pub fn F2_conv_ud2sf(a: i64) -> f32;
        #[link_name = "llvm.hexagon.F2.conv.uw2df"]
        pub fn F2_conv_uw2df(a: i32) -> f64;
        #[link_name = "llvm.hexagon.F2.conv.uw2sf"]
        pub fn F2_conv_uw2sf(a: i32) -> f32;
        #[link_name = "llvm.hexagon.F2.conv.w2df"]
        pub fn F2_conv_w2df(a: i32) -> f64;
        #[link_name = "llvm.hexagon.F2.conv.w2sf"]
        pub fn F2_conv_w2sf(a: i32) -> f32;
        #[link_name = "llvm.hexagon.F2.dfadd"]
        pub fn F2_dfadd(a: f64, b: f64) -> f64;
        #[link_name = "llvm.hexagon.F2.dfclass"]
        pub fn F2_dfclass(a: f64, b: i32) -> bool;
        #[link_name = "llvm.hexagon.F2.dfcmpeq"]
        pub fn F2_dfcmpeq(a: f64, b: f64) -> bool;
        #[link_name = "llvm.hexagon.F2.dfcmpge"]
        pub fn F2_dfcmpge(a: f64, b: f64) -> bool;
        #[link_name = "llvm.hexagon.F2.dfcmpgt"]
        pub fn F2_dfcmpgt(a: f64, b: f64) -> bool;
        #[link_name = "llvm.hexagon.F2.dfcmpuo"]
        pub fn F2_dfcmpuo(a: f64, b: f64) -> bool;
        #[link_name = "llvm.hexagon.F2.dffixupd"]
        pub fn F2_dffixupd(a: f64, b: f64) -> f64;
        #[link_name = "llvm.hexagon.F2.dffixupn"]
        pub fn F2_dffixupn(a: f64, b: f64) -> f64;
        #[link_name = "llvm.hexagon.F2.dffixupr"]
        pub fn F2_dffixupr(a: f64) -> f64;
        #[link_name = "llvm.hexagon.F2.dffma"]
        pub fn F2_dffma(a: f64, b: f64, c: f64) -> f64;
        #[link_name = "llvm.hexagon.F2.dffma.lib"]
        pub fn F2_dffma_lib(a: f64, b: f64, c: f64) -> f64;
        #[link_name = "llvm.hexagon.F2.dffma.sc"]
        pub fn F2_dffma_sc(a: f64, b: f64, c: f64, d: i32) -> f64;
        #[link_name = "llvm.hexagon.F2.dffms"]
        pub fn F2_dffms(a: f64, b: f64, c: f64) -> f64;
        #[link_name = "llvm.hexagon.F2.dffms.lib"]
        pub fn F2_dffms_lib(a: f64, b: f64, c: f64) -> f64;
        #[link_name = "llvm.hexagon.F2.dfimm.n"]
        pub fn F2_dfimm_n(a: i32) -> f64;
        #[link_name = "llvm.hexagon.F2.dfimm.p"]
        pub fn F2_dfimm_p(a: i32) -> f64;
        #[link_name = "llvm.hexagon.F2.dfmax"]
        pub fn F2_dfmax(a: f64, b: f64) -> f64;
        #[link_name = "llvm.hexagon.F2.dfmin"]
        pub fn F2_dfmin(a: f64, b: f64) -> f64;
        #[link_name = "llvm.hexagon.F2.dfmpy"]
        pub fn F2_dfmpy(a: f64, b: f64) -> f64;
        #[link_name = "llvm.hexagon.F2.dfsub"]
        pub fn F2_dfsub(a: f64, b: f64) -> f64;
        #[link_name = "llvm.hexagon.F2.sfadd"]
        pub fn F2_sfadd(a: f32, b: f32) -> f32;
        #[link_name = "llvm.hexagon.F2.sfclass"]
        pub fn F2_sfclass(a: f32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.F2.sfcmpeq"]
        pub fn F2_sfcmpeq(a: f32, b: f32) -> bool;
        #[link_name = "llvm.hexagon.F2.sfcmpge"]
        pub fn F2_sfcmpge(a: f32, b: f32) -> bool;
        #[link_name = "llvm.hexagon.F2.sfcmpgt"]
        pub fn F2_sfcmpgt(a: f32, b: f32) -> bool;
        #[link_name = "llvm.hexagon.F2.sfcmpuo"]
        pub fn F2_sfcmpuo(a: f32, b: f32) -> bool;
        #[link_name = "llvm.hexagon.F2.sffixupd"]
        pub fn F2_sffixupd(a: f32, b: f32) -> f32;
        #[link_name = "llvm.hexagon.F2.sffixupn"]
        pub fn F2_sffixupn(a: f32, b: f32) -> f32;
        #[link_name = "llvm.hexagon.F2.sffixupr"]
        pub fn F2_sffixupr(a: f32) -> f32;
        #[link_name = "llvm.hexagon.F2.sffma"]
        pub fn F2_sffma(a: f32, b: f32, c: f32) -> f32;
        #[link_name = "llvm.hexagon.F2.sffma.lib"]
        pub fn F2_sffma_lib(a: f32, b: f32, c: f32) -> f32;
        #[link_name = "llvm.hexagon.F2.sffma.sc"]
        pub fn F2_sffma_sc(a: f32, b: f32, c: f32, d: i32) -> f32;
        #[link_name = "llvm.hexagon.F2.sffms"]
        pub fn F2_sffms(a: f32, b: f32, c: f32) -> f32;
        #[link_name = "llvm.hexagon.F2.sffms.lib"]
        pub fn F2_sffms_lib(a: f32, b: f32, c: f32) -> f32;
        #[link_name = "llvm.hexagon.F2.sfimm.n"]
        pub fn F2_sfimm_n(a: i32) -> f32;
        #[link_name = "llvm.hexagon.F2.sfimm.p"]
        pub fn F2_sfimm_p(a: i32) -> f32;
        #[link_name = "llvm.hexagon.F2.sfmax"]
        pub fn F2_sfmax(a: f32, b: f32) -> f32;
        #[link_name = "llvm.hexagon.F2.sfmin"]
        pub fn F2_sfmin(a: f32, b: f32) -> f32;
        #[link_name = "llvm.hexagon.F2.sfmpy"]
        pub fn F2_sfmpy(a: f32, b: f32) -> f32;
        #[link_name = "llvm.hexagon.F2.sfsub"]
        pub fn F2_sfsub(a: f32, b: f32) -> f32;
        #[link_name = "llvm.hexagon.M2.acci"]
        pub fn M2_acci(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.accii"]
        pub fn M2_accii(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.cmaci.s0"]
        pub fn M2_cmaci_s0(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.cmacr.s0"]
        pub fn M2_cmacr_s0(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.cmacs.s0"]
        pub fn M2_cmacs_s0(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.cmacs.s1"]
        pub fn M2_cmacs_s1(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.cmacsc.s0"]
        pub fn M2_cmacsc_s0(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.cmacsc.s1"]
        pub fn M2_cmacsc_s1(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.cmpyi.s0"]
        pub fn M2_cmpyi_s0(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.cmpyr.s0"]
        pub fn M2_cmpyr_s0(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.cmpyrs.s0"]
        pub fn M2_cmpyrs_s0(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.cmpyrs.s1"]
        pub fn M2_cmpyrs_s1(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.cmpyrsc.s0"]
        pub fn M2_cmpyrsc_s0(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.cmpyrsc.s1"]
        pub fn M2_cmpyrsc_s1(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.cmpys.s0"]
        pub fn M2_cmpys_s0(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.cmpys.s1"]
        pub fn M2_cmpys_s1(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.cmpysc.s0"]
        pub fn M2_cmpysc_s0(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.cmpysc.s1"]
        pub fn M2_cmpysc_s1(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.cnacs.s0"]
        pub fn M2_cnacs_s0(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.cnacs.s1"]
        pub fn M2_cnacs_s1(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.cnacsc.s0"]
        pub fn M2_cnacsc_s0(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.cnacsc.s1"]
        pub fn M2_cnacsc_s1(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.dpmpyss.acc.s0"]
        pub fn M2_dpmpyss_acc_s0(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.dpmpyss.nac.s0"]
        pub fn M2_dpmpyss_nac_s0(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.dpmpyss.rnd.s0"]
        pub fn M2_dpmpyss_rnd_s0(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.dpmpyss.s0"]
        pub fn M2_dpmpyss_s0(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.dpmpyuu.acc.s0"]
        pub fn M2_dpmpyuu_acc_s0(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.dpmpyuu.nac.s0"]
        pub fn M2_dpmpyuu_nac_s0(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.dpmpyuu.s0"]
        pub fn M2_dpmpyuu_s0(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.hmmpyh.rs1"]
        pub fn M2_hmmpyh_rs1(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.hmmpyh.s1"]
        pub fn M2_hmmpyh_s1(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.hmmpyl.rs1"]
        pub fn M2_hmmpyl_rs1(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.hmmpyl.s1"]
        pub fn M2_hmmpyl_s1(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.maci"]
        pub fn M2_maci(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.macsin"]
        pub fn M2_macsin(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.macsip"]
        pub fn M2_macsip(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mmachs.rs0"]
        pub fn M2_mmachs_rs0(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.mmachs.rs1"]
        pub fn M2_mmachs_rs1(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.mmachs.s0"]
        pub fn M2_mmachs_s0(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.mmachs.s1"]
        pub fn M2_mmachs_s1(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.mmacls.rs0"]
        pub fn M2_mmacls_rs0(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.mmacls.rs1"]
        pub fn M2_mmacls_rs1(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.mmacls.s0"]
        pub fn M2_mmacls_s0(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.mmacls.s1"]
        pub fn M2_mmacls_s1(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.mmacuhs.rs0"]
        pub fn M2_mmacuhs_rs0(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.mmacuhs.rs1"]
        pub fn M2_mmacuhs_rs1(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.mmacuhs.s0"]
        pub fn M2_mmacuhs_s0(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.mmacuhs.s1"]
        pub fn M2_mmacuhs_s1(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.mmaculs.rs0"]
        pub fn M2_mmaculs_rs0(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.mmaculs.rs1"]
        pub fn M2_mmaculs_rs1(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.mmaculs.s0"]
        pub fn M2_mmaculs_s0(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.mmaculs.s1"]
        pub fn M2_mmaculs_s1(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.mmpyh.rs0"]
        pub fn M2_mmpyh_rs0(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.mmpyh.rs1"]
        pub fn M2_mmpyh_rs1(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.mmpyh.s0"]
        pub fn M2_mmpyh_s0(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.mmpyh.s1"]
        pub fn M2_mmpyh_s1(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.mmpyl.rs0"]
        pub fn M2_mmpyl_rs0(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.mmpyl.rs1"]
        pub fn M2_mmpyl_rs1(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.mmpyl.s0"]
        pub fn M2_mmpyl_s0(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.mmpyl.s1"]
        pub fn M2_mmpyl_s1(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.mmpyuh.rs0"]
        pub fn M2_mmpyuh_rs0(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.mmpyuh.rs1"]
        pub fn M2_mmpyuh_rs1(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.mmpyuh.s0"]
        pub fn M2_mmpyuh_s0(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.mmpyuh.s1"]
        pub fn M2_mmpyuh_s1(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.mmpyul.rs0"]
        pub fn M2_mmpyul_rs0(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.mmpyul.rs1"]
        pub fn M2_mmpyul_rs1(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.mmpyul.s0"]
        pub fn M2_mmpyul_s0(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.mmpyul.s1"]
        pub fn M2_mmpyul_s1(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.mpy.acc.hh.s0"]
        pub fn M2_mpy_acc_hh_s0(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.acc.hh.s1"]
        pub fn M2_mpy_acc_hh_s1(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.acc.hl.s0"]
        pub fn M2_mpy_acc_hl_s0(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.acc.hl.s1"]
        pub fn M2_mpy_acc_hl_s1(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.acc.lh.s0"]
        pub fn M2_mpy_acc_lh_s0(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.acc.lh.s1"]
        pub fn M2_mpy_acc_lh_s1(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.acc.ll.s0"]
        pub fn M2_mpy_acc_ll_s0(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.acc.ll.s1"]
        pub fn M2_mpy_acc_ll_s1(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.acc.sat.hh.s0"]
        pub fn M2_mpy_acc_sat_hh_s0(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.acc.sat.hh.s1"]
        pub fn M2_mpy_acc_sat_hh_s1(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.acc.sat.hl.s0"]
        pub fn M2_mpy_acc_sat_hl_s0(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.acc.sat.hl.s1"]
        pub fn M2_mpy_acc_sat_hl_s1(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.acc.sat.lh.s0"]
        pub fn M2_mpy_acc_sat_lh_s0(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.acc.sat.lh.s1"]
        pub fn M2_mpy_acc_sat_lh_s1(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.acc.sat.ll.s0"]
        pub fn M2_mpy_acc_sat_ll_s0(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.acc.sat.ll.s1"]
        pub fn M2_mpy_acc_sat_ll_s1(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.hh.s0"]
        pub fn M2_mpy_hh_s0(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.hh.s1"]
        pub fn M2_mpy_hh_s1(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.hl.s0"]
        pub fn M2_mpy_hl_s0(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.hl.s1"]
        pub fn M2_mpy_hl_s1(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.lh.s0"]
        pub fn M2_mpy_lh_s0(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.lh.s1"]
        pub fn M2_mpy_lh_s1(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.ll.s0"]
        pub fn M2_mpy_ll_s0(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.ll.s1"]
        pub fn M2_mpy_ll_s1(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.nac.hh.s0"]
        pub fn M2_mpy_nac_hh_s0(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.nac.hh.s1"]
        pub fn M2_mpy_nac_hh_s1(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.nac.hl.s0"]
        pub fn M2_mpy_nac_hl_s0(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.nac.hl.s1"]
        pub fn M2_mpy_nac_hl_s1(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.nac.lh.s0"]
        pub fn M2_mpy_nac_lh_s0(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.nac.lh.s1"]
        pub fn M2_mpy_nac_lh_s1(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.nac.ll.s0"]
        pub fn M2_mpy_nac_ll_s0(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.nac.ll.s1"]
        pub fn M2_mpy_nac_ll_s1(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.nac.sat.hh.s0"]
        pub fn M2_mpy_nac_sat_hh_s0(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.nac.sat.hh.s1"]
        pub fn M2_mpy_nac_sat_hh_s1(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.nac.sat.hl.s0"]
        pub fn M2_mpy_nac_sat_hl_s0(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.nac.sat.hl.s1"]
        pub fn M2_mpy_nac_sat_hl_s1(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.nac.sat.lh.s0"]
        pub fn M2_mpy_nac_sat_lh_s0(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.nac.sat.lh.s1"]
        pub fn M2_mpy_nac_sat_lh_s1(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.nac.sat.ll.s0"]
        pub fn M2_mpy_nac_sat_ll_s0(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.nac.sat.ll.s1"]
        pub fn M2_mpy_nac_sat_ll_s1(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.rnd.hh.s0"]
        pub fn M2_mpy_rnd_hh_s0(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.rnd.hh.s1"]
        pub fn M2_mpy_rnd_hh_s1(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.rnd.hl.s0"]
        pub fn M2_mpy_rnd_hl_s0(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.rnd.hl.s1"]
        pub fn M2_mpy_rnd_hl_s1(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.rnd.lh.s0"]
        pub fn M2_mpy_rnd_lh_s0(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.rnd.lh.s1"]
        pub fn M2_mpy_rnd_lh_s1(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.rnd.ll.s0"]
        pub fn M2_mpy_rnd_ll_s0(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.rnd.ll.s1"]
        pub fn M2_mpy_rnd_ll_s1(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.sat.hh.s0"]
        pub fn M2_mpy_sat_hh_s0(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.sat.hh.s1"]
        pub fn M2_mpy_sat_hh_s1(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.sat.hl.s0"]
        pub fn M2_mpy_sat_hl_s0(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.sat.hl.s1"]
        pub fn M2_mpy_sat_hl_s1(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.sat.lh.s0"]
        pub fn M2_mpy_sat_lh_s0(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.sat.lh.s1"]
        pub fn M2_mpy_sat_lh_s1(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.sat.ll.s0"]
        pub fn M2_mpy_sat_ll_s0(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.sat.ll.s1"]
        pub fn M2_mpy_sat_ll_s1(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.sat.rnd.hh.s0"]
        pub fn M2_mpy_sat_rnd_hh_s0(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.sat.rnd.hh.s1"]
        pub fn M2_mpy_sat_rnd_hh_s1(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.sat.rnd.hl.s0"]
        pub fn M2_mpy_sat_rnd_hl_s0(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.sat.rnd.hl.s1"]
        pub fn M2_mpy_sat_rnd_hl_s1(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.sat.rnd.lh.s0"]
        pub fn M2_mpy_sat_rnd_lh_s0(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.sat.rnd.lh.s1"]
        pub fn M2_mpy_sat_rnd_lh_s1(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.sat.rnd.ll.s0"]
        pub fn M2_mpy_sat_rnd_ll_s0(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.sat.rnd.ll.s1"]
        pub fn M2_mpy_sat_rnd_ll_s1(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.up"]
        pub fn M2_mpy_up(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.up.s1"]
        pub fn M2_mpy_up_s1(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpy.up.s1.sat"]
        pub fn M2_mpy_up_s1_sat(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpyd.acc.hh.s0"]
        pub fn M2_mpyd_acc_hh_s0(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyd.acc.hh.s1"]
        pub fn M2_mpyd_acc_hh_s1(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyd.acc.hl.s0"]
        pub fn M2_mpyd_acc_hl_s0(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyd.acc.hl.s1"]
        pub fn M2_mpyd_acc_hl_s1(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyd.acc.lh.s0"]
        pub fn M2_mpyd_acc_lh_s0(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyd.acc.lh.s1"]
        pub fn M2_mpyd_acc_lh_s1(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyd.acc.ll.s0"]
        pub fn M2_mpyd_acc_ll_s0(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyd.acc.ll.s1"]
        pub fn M2_mpyd_acc_ll_s1(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyd.hh.s0"]
        pub fn M2_mpyd_hh_s0(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyd.hh.s1"]
        pub fn M2_mpyd_hh_s1(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyd.hl.s0"]
        pub fn M2_mpyd_hl_s0(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyd.hl.s1"]
        pub fn M2_mpyd_hl_s1(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyd.lh.s0"]
        pub fn M2_mpyd_lh_s0(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyd.lh.s1"]
        pub fn M2_mpyd_lh_s1(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyd.ll.s0"]
        pub fn M2_mpyd_ll_s0(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyd.ll.s1"]
        pub fn M2_mpyd_ll_s1(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyd.nac.hh.s0"]
        pub fn M2_mpyd_nac_hh_s0(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyd.nac.hh.s1"]
        pub fn M2_mpyd_nac_hh_s1(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyd.nac.hl.s0"]
        pub fn M2_mpyd_nac_hl_s0(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyd.nac.hl.s1"]
        pub fn M2_mpyd_nac_hl_s1(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyd.nac.lh.s0"]
        pub fn M2_mpyd_nac_lh_s0(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyd.nac.lh.s1"]
        pub fn M2_mpyd_nac_lh_s1(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyd.nac.ll.s0"]
        pub fn M2_mpyd_nac_ll_s0(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyd.nac.ll.s1"]
        pub fn M2_mpyd_nac_ll_s1(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyd.rnd.hh.s0"]
        pub fn M2_mpyd_rnd_hh_s0(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyd.rnd.hh.s1"]
        pub fn M2_mpyd_rnd_hh_s1(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyd.rnd.hl.s0"]
        pub fn M2_mpyd_rnd_hl_s0(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyd.rnd.hl.s1"]
        pub fn M2_mpyd_rnd_hl_s1(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyd.rnd.lh.s0"]
        pub fn M2_mpyd_rnd_lh_s0(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyd.rnd.lh.s1"]
        pub fn M2_mpyd_rnd_lh_s1(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyd.rnd.ll.s0"]
        pub fn M2_mpyd_rnd_ll_s0(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyd.rnd.ll.s1"]
        pub fn M2_mpyd_rnd_ll_s1(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyi"]
        pub fn M2_mpyi(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpysmi"]
        pub fn M2_mpysmi(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpysu.up"]
        pub fn M2_mpysu_up(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpyu.acc.hh.s0"]
        pub fn M2_mpyu_acc_hh_s0(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpyu.acc.hh.s1"]
        pub fn M2_mpyu_acc_hh_s1(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpyu.acc.hl.s0"]
        pub fn M2_mpyu_acc_hl_s0(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpyu.acc.hl.s1"]
        pub fn M2_mpyu_acc_hl_s1(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpyu.acc.lh.s0"]
        pub fn M2_mpyu_acc_lh_s0(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpyu.acc.lh.s1"]
        pub fn M2_mpyu_acc_lh_s1(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpyu.acc.ll.s0"]
        pub fn M2_mpyu_acc_ll_s0(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpyu.acc.ll.s1"]
        pub fn M2_mpyu_acc_ll_s1(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpyu.hh.s0"]
        pub fn M2_mpyu_hh_s0(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpyu.hh.s1"]
        pub fn M2_mpyu_hh_s1(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpyu.hl.s0"]
        pub fn M2_mpyu_hl_s0(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpyu.hl.s1"]
        pub fn M2_mpyu_hl_s1(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpyu.lh.s0"]
        pub fn M2_mpyu_lh_s0(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpyu.lh.s1"]
        pub fn M2_mpyu_lh_s1(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpyu.ll.s0"]
        pub fn M2_mpyu_ll_s0(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpyu.ll.s1"]
        pub fn M2_mpyu_ll_s1(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpyu.nac.hh.s0"]
        pub fn M2_mpyu_nac_hh_s0(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpyu.nac.hh.s1"]
        pub fn M2_mpyu_nac_hh_s1(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpyu.nac.hl.s0"]
        pub fn M2_mpyu_nac_hl_s0(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpyu.nac.hl.s1"]
        pub fn M2_mpyu_nac_hl_s1(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpyu.nac.lh.s0"]
        pub fn M2_mpyu_nac_lh_s0(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpyu.nac.lh.s1"]
        pub fn M2_mpyu_nac_lh_s1(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpyu.nac.ll.s0"]
        pub fn M2_mpyu_nac_ll_s0(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpyu.nac.ll.s1"]
        pub fn M2_mpyu_nac_ll_s1(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpyu.up"]
        pub fn M2_mpyu_up(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.mpyud.acc.hh.s0"]
        pub fn M2_mpyud_acc_hh_s0(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyud.acc.hh.s1"]
        pub fn M2_mpyud_acc_hh_s1(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyud.acc.hl.s0"]
        pub fn M2_mpyud_acc_hl_s0(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyud.acc.hl.s1"]
        pub fn M2_mpyud_acc_hl_s1(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyud.acc.lh.s0"]
        pub fn M2_mpyud_acc_lh_s0(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyud.acc.lh.s1"]
        pub fn M2_mpyud_acc_lh_s1(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyud.acc.ll.s0"]
        pub fn M2_mpyud_acc_ll_s0(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyud.acc.ll.s1"]
        pub fn M2_mpyud_acc_ll_s1(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyud.hh.s0"]
        pub fn M2_mpyud_hh_s0(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyud.hh.s1"]
        pub fn M2_mpyud_hh_s1(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyud.hl.s0"]
        pub fn M2_mpyud_hl_s0(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyud.hl.s1"]
        pub fn M2_mpyud_hl_s1(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyud.lh.s0"]
        pub fn M2_mpyud_lh_s0(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyud.lh.s1"]
        pub fn M2_mpyud_lh_s1(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyud.ll.s0"]
        pub fn M2_mpyud_ll_s0(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyud.ll.s1"]
        pub fn M2_mpyud_ll_s1(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyud.nac.hh.s0"]
        pub fn M2_mpyud_nac_hh_s0(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyud.nac.hh.s1"]
        pub fn M2_mpyud_nac_hh_s1(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyud.nac.hl.s0"]
        pub fn M2_mpyud_nac_hl_s0(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyud.nac.hl.s1"]
        pub fn M2_mpyud_nac_hl_s1(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyud.nac.lh.s0"]
        pub fn M2_mpyud_nac_lh_s0(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyud.nac.lh.s1"]
        pub fn M2_mpyud_nac_lh_s1(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyud.nac.ll.s0"]
        pub fn M2_mpyud_nac_ll_s0(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyud.nac.ll.s1"]
        pub fn M2_mpyud_nac_ll_s1(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.mpyui"]
        pub fn M2_mpyui(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.nacci"]
        pub fn M2_nacci(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.naccii"]
        pub fn M2_naccii(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.subacc"]
        pub fn M2_subacc(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.vabsdiffh"]
        pub fn M2_vabsdiffh(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.vabsdiffw"]
        pub fn M2_vabsdiffw(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.vcmac.s0.sat.i"]
        pub fn M2_vcmac_s0_sat_i(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.vcmac.s0.sat.r"]
        pub fn M2_vcmac_s0_sat_r(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.vcmpy.s0.sat.i"]
        pub fn M2_vcmpy_s0_sat_i(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.vcmpy.s0.sat.r"]
        pub fn M2_vcmpy_s0_sat_r(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.vcmpy.s1.sat.i"]
        pub fn M2_vcmpy_s1_sat_i(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.vcmpy.s1.sat.r"]
        pub fn M2_vcmpy_s1_sat_r(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.vdmacs.s0"]
        pub fn M2_vdmacs_s0(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.vdmacs.s1"]
        pub fn M2_vdmacs_s1(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.vdmpyrs.s0"]
        pub fn M2_vdmpyrs_s0(a: i64, b: i64) -> i32;
        #[link_name = "llvm.hexagon.M2.vdmpyrs.s1"]
        pub fn M2_vdmpyrs_s1(a: i64, b: i64) -> i32;
        #[link_name = "llvm.hexagon.M2.vdmpys.s0"]
        pub fn M2_vdmpys_s0(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.vdmpys.s1"]
        pub fn M2_vdmpys_s1(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.vmac2"]
        pub fn M2_vmac2(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.vmac2es"]
        pub fn M2_vmac2es(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.vmac2es.s0"]
        pub fn M2_vmac2es_s0(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.vmac2es.s1"]
        pub fn M2_vmac2es_s1(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.vmac2s.s0"]
        pub fn M2_vmac2s_s0(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.vmac2s.s1"]
        pub fn M2_vmac2s_s1(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.vmac2su.s0"]
        pub fn M2_vmac2su_s0(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.vmac2su.s1"]
        pub fn M2_vmac2su_s1(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.vmpy2es.s0"]
        pub fn M2_vmpy2es_s0(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.vmpy2es.s1"]
        pub fn M2_vmpy2es_s1(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.vmpy2s.s0"]
        pub fn M2_vmpy2s_s0(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.vmpy2s.s0pack"]
        pub fn M2_vmpy2s_s0pack(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.vmpy2s.s1"]
        pub fn M2_vmpy2s_s1(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.vmpy2s.s1pack"]
        pub fn M2_vmpy2s_s1pack(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.vmpy2su.s0"]
        pub fn M2_vmpy2su_s0(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.vmpy2su.s1"]
        pub fn M2_vmpy2su_s1(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.vraddh"]
        pub fn M2_vraddh(a: i64, b: i64) -> i32;
        #[link_name = "llvm.hexagon.M2.vradduh"]
        pub fn M2_vradduh(a: i64, b: i64) -> i32;
        #[link_name = "llvm.hexagon.M2.vrcmaci.s0"]
        pub fn M2_vrcmaci_s0(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.vrcmaci.s0c"]
        pub fn M2_vrcmaci_s0c(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.vrcmacr.s0"]
        pub fn M2_vrcmacr_s0(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.vrcmacr.s0c"]
        pub fn M2_vrcmacr_s0c(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.vrcmpyi.s0"]
        pub fn M2_vrcmpyi_s0(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.vrcmpyi.s0c"]
        pub fn M2_vrcmpyi_s0c(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.vrcmpyr.s0"]
        pub fn M2_vrcmpyr_s0(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.vrcmpyr.s0c"]
        pub fn M2_vrcmpyr_s0c(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.vrcmpys.acc.s1"]
        pub fn M2_vrcmpys_acc_s1(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.vrcmpys.s1"]
        pub fn M2_vrcmpys_s1(a: i64, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M2.vrcmpys.s1rp"]
        pub fn M2_vrcmpys_s1rp(a: i64, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M2.vrmac.s0"]
        pub fn M2_vrmac_s0(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.vrmpy.s0"]
        pub fn M2_vrmpy_s0(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.M2.xor.xacc"]
        pub fn M2_xor_xacc(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M4.and.and"]
        pub fn M4_and_and(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M4.and.andn"]
        pub fn M4_and_andn(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M4.and.or"]
        pub fn M4_and_or(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M4.and.xor"]
        pub fn M4_and_xor(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M4.cmpyi.wh"]
        pub fn M4_cmpyi_wh(a: i64, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M4.cmpyi.whc"]
        pub fn M4_cmpyi_whc(a: i64, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M4.cmpyr.wh"]
        pub fn M4_cmpyr_wh(a: i64, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M4.cmpyr.whc"]
        pub fn M4_cmpyr_whc(a: i64, b: i32) -> i32;
        #[link_name = "llvm.hexagon.M4.mac.up.s1.sat"]
        pub fn M4_mac_up_s1_sat(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M4.mpyri.addi"]
        pub fn M4_mpyri_addi(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M4.mpyri.addr"]
        pub fn M4_mpyri_addr(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M4.mpyri.addr.u2"]
        pub fn M4_mpyri_addr_u2(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M4.mpyrr.addi"]
        pub fn M4_mpyrr_addi(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M4.mpyrr.addr"]
        pub fn M4_mpyrr_addr(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M4.nac.up.s1.sat"]
        pub fn M4_nac_up_s1_sat(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M4.or.and"]
        pub fn M4_or_and(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M4.or.andn"]
        pub fn M4_or_andn(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M4.or.or"]
        pub fn M4_or_or(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M4.or.xor"]
        pub fn M4_or_xor(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M4.pmpyw"]
        pub fn M4_pmpyw(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M4.pmpyw.acc"]
        pub fn M4_pmpyw_acc(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M4.vpmpyh"]
        pub fn M4_vpmpyh(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M4.vpmpyh.acc"]
        pub fn M4_vpmpyh_acc(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M4.vrmpyeh.acc.s0"]
        pub fn M4_vrmpyeh_acc_s0(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.M4.vrmpyeh.acc.s1"]
        pub fn M4_vrmpyeh_acc_s1(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.M4.vrmpyeh.s0"]
        pub fn M4_vrmpyeh_s0(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.M4.vrmpyeh.s1"]
        pub fn M4_vrmpyeh_s1(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.M4.vrmpyoh.acc.s0"]
        pub fn M4_vrmpyoh_acc_s0(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.M4.vrmpyoh.acc.s1"]
        pub fn M4_vrmpyoh_acc_s1(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.M4.vrmpyoh.s0"]
        pub fn M4_vrmpyoh_s0(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.M4.vrmpyoh.s1"]
        pub fn M4_vrmpyoh_s1(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.M4.xor.and"]
        pub fn M4_xor_and(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M4.xor.andn"]
        pub fn M4_xor_andn(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M4.xor.or"]
        pub fn M4_xor_or(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.M4.xor.xacc"]
        pub fn M4_xor_xacc(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.M5.vdmacbsu"]
        pub fn M5_vdmacbsu(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.M5.vdmpybsu"]
        pub fn M5_vdmpybsu(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.M5.vmacbsu"]
        pub fn M5_vmacbsu(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M5.vmacbuu"]
        pub fn M5_vmacbuu(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.M5.vmpybsu"]
        pub fn M5_vmpybsu(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M5.vmpybuu"]
        pub fn M5_vmpybuu(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.M5.vrmacbsu"]
        pub fn M5_vrmacbsu(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.M5.vrmacbuu"]
        pub fn M5_vrmacbuu(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.M5.vrmpybsu"]
        pub fn M5_vrmpybsu(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.M5.vrmpybuu"]
        pub fn M5_vrmpybuu(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.S2.addasl.rrri"]
        pub fn S2_addasl_rrri(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.asl.i.p"]
        pub fn S2_asl_i_p(a: i64, b: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.asl.i.p.acc"]
        pub fn S2_asl_i_p_acc(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.asl.i.p.and"]
        pub fn S2_asl_i_p_and(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.asl.i.p.nac"]
        pub fn S2_asl_i_p_nac(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.asl.i.p.or"]
        pub fn S2_asl_i_p_or(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.asl.i.p.xacc"]
        pub fn S2_asl_i_p_xacc(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.asl.i.r"]
        pub fn S2_asl_i_r(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.asl.i.r.acc"]
        pub fn S2_asl_i_r_acc(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.asl.i.r.and"]
        pub fn S2_asl_i_r_and(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.asl.i.r.nac"]
        pub fn S2_asl_i_r_nac(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.asl.i.r.or"]
        pub fn S2_asl_i_r_or(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.asl.i.r.sat"]
        pub fn S2_asl_i_r_sat(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.asl.i.r.xacc"]
        pub fn S2_asl_i_r_xacc(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.asl.i.vh"]
        pub fn S2_asl_i_vh(a: i64, b: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.asl.i.vw"]
        pub fn S2_asl_i_vw(a: i64, b: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.asl.r.p"]
        pub fn S2_asl_r_p(a: i64, b: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.asl.r.p.acc"]
        pub fn S2_asl_r_p_acc(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.asl.r.p.and"]
        pub fn S2_asl_r_p_and(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.asl.r.p.nac"]
        pub fn S2_asl_r_p_nac(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.asl.r.p.or"]
        pub fn S2_asl_r_p_or(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.asl.r.p.xor"]
        pub fn S2_asl_r_p_xor(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.asl.r.r"]
        pub fn S2_asl_r_r(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.asl.r.r.acc"]
        pub fn S2_asl_r_r_acc(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.asl.r.r.and"]
        pub fn S2_asl_r_r_and(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.asl.r.r.nac"]
        pub fn S2_asl_r_r_nac(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.asl.r.r.or"]
        pub fn S2_asl_r_r_or(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.asl.r.r.sat"]
        pub fn S2_asl_r_r_sat(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.asl.r.vh"]
        pub fn S2_asl_r_vh(a: i64, b: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.asl.r.vw"]
        pub fn S2_asl_r_vw(a: i64, b: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.asr.i.p"]
        pub fn S2_asr_i_p(a: i64, b: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.asr.i.p.acc"]
        pub fn S2_asr_i_p_acc(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.asr.i.p.and"]
        pub fn S2_asr_i_p_and(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.asr.i.p.nac"]
        pub fn S2_asr_i_p_nac(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.asr.i.p.or"]
        pub fn S2_asr_i_p_or(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.asr.i.p.rnd"]
        pub fn S2_asr_i_p_rnd(a: i64, b: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.asr.i.p.rnd.goodsyntax"]
        pub fn S2_asr_i_p_rnd_goodsyntax(a: i64, b: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.asr.i.r"]
        pub fn S2_asr_i_r(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.asr.i.r.acc"]
        pub fn S2_asr_i_r_acc(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.asr.i.r.and"]
        pub fn S2_asr_i_r_and(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.asr.i.r.nac"]
        pub fn S2_asr_i_r_nac(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.asr.i.r.or"]
        pub fn S2_asr_i_r_or(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.asr.i.r.rnd"]
        pub fn S2_asr_i_r_rnd(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.asr.i.r.rnd.goodsyntax"]
        pub fn S2_asr_i_r_rnd_goodsyntax(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.asr.i.svw.trun"]
        pub fn S2_asr_i_svw_trun(a: i64, b: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.asr.i.vh"]
        pub fn S2_asr_i_vh(a: i64, b: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.asr.i.vw"]
        pub fn S2_asr_i_vw(a: i64, b: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.asr.r.p"]
        pub fn S2_asr_r_p(a: i64, b: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.asr.r.p.acc"]
        pub fn S2_asr_r_p_acc(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.asr.r.p.and"]
        pub fn S2_asr_r_p_and(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.asr.r.p.nac"]
        pub fn S2_asr_r_p_nac(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.asr.r.p.or"]
        pub fn S2_asr_r_p_or(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.asr.r.p.xor"]
        pub fn S2_asr_r_p_xor(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.asr.r.r"]
        pub fn S2_asr_r_r(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.asr.r.r.acc"]
        pub fn S2_asr_r_r_acc(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.asr.r.r.and"]
        pub fn S2_asr_r_r_and(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.asr.r.r.nac"]
        pub fn S2_asr_r_r_nac(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.asr.r.r.or"]
        pub fn S2_asr_r_r_or(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.asr.r.r.sat"]
        pub fn S2_asr_r_r_sat(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.asr.r.svw.trun"]
        pub fn S2_asr_r_svw_trun(a: i64, b: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.asr.r.vh"]
        pub fn S2_asr_r_vh(a: i64, b: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.asr.r.vw"]
        pub fn S2_asr_r_vw(a: i64, b: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.brev"]
        pub fn S2_brev(a: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.brevp"]
        pub fn S2_brevp(a: i64) -> i64;
        #[link_name = "llvm.hexagon.S2.cl0"]
        pub fn S2_cl0(a: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.cl0p"]
        pub fn S2_cl0p(a: i64) -> i32;
        #[link_name = "llvm.hexagon.S2.cl1"]
        pub fn S2_cl1(a: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.cl1p"]
        pub fn S2_cl1p(a: i64) -> i32;
        #[link_name = "llvm.hexagon.S2.clb"]
        pub fn S2_clb(a: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.clbnorm"]
        pub fn S2_clbnorm(a: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.clbp"]
        pub fn S2_clbp(a: i64) -> i32;
        #[link_name = "llvm.hexagon.S2.clrbit.i"]
        pub fn S2_clrbit_i(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.clrbit.r"]
        pub fn S2_clrbit_r(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.ct0"]
        pub fn S2_ct0(a: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.ct0p"]
        pub fn S2_ct0p(a: i64) -> i32;
        #[link_name = "llvm.hexagon.S2.ct1"]
        pub fn S2_ct1(a: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.ct1p"]
        pub fn S2_ct1p(a: i64) -> i32;
        #[link_name = "llvm.hexagon.S2.deinterleave"]
        pub fn S2_deinterleave(a: i64) -> i64;
        #[link_name = "llvm.hexagon.S2.extractu"]
        pub fn S2_extractu(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.extractu.rp"]
        pub fn S2_extractu_rp(a: i32, b: i64) -> i32;
        #[link_name = "llvm.hexagon.S2.extractup"]
        pub fn S2_extractup(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.extractup.rp"]
        pub fn S2_extractup_rp(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.S2.insert"]
        pub fn S2_insert(a: i32, b: i32, c: i32, d: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.insert.rp"]
        pub fn S2_insert_rp(a: i32, b: i32, c: i64) -> i32;
        #[link_name = "llvm.hexagon.S2.insertp"]
        pub fn S2_insertp(a: i64, b: i64, c: i32, d: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.insertp.rp"]
        pub fn S2_insertp_rp(a: i64, b: i64, c: i64) -> i64;
        #[link_name = "llvm.hexagon.S2.interleave"]
        pub fn S2_interleave(a: i64) -> i64;
        #[link_name = "llvm.hexagon.S2.lfsp"]
        pub fn S2_lfsp(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.S2.lsl.r.p"]
        pub fn S2_lsl_r_p(a: i64, b: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.lsl.r.p.acc"]
        pub fn S2_lsl_r_p_acc(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.lsl.r.p.and"]
        pub fn S2_lsl_r_p_and(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.lsl.r.p.nac"]
        pub fn S2_lsl_r_p_nac(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.lsl.r.p.or"]
        pub fn S2_lsl_r_p_or(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.lsl.r.p.xor"]
        pub fn S2_lsl_r_p_xor(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.lsl.r.r"]
        pub fn S2_lsl_r_r(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.lsl.r.r.acc"]
        pub fn S2_lsl_r_r_acc(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.lsl.r.r.and"]
        pub fn S2_lsl_r_r_and(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.lsl.r.r.nac"]
        pub fn S2_lsl_r_r_nac(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.lsl.r.r.or"]
        pub fn S2_lsl_r_r_or(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.lsl.r.vh"]
        pub fn S2_lsl_r_vh(a: i64, b: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.lsl.r.vw"]
        pub fn S2_lsl_r_vw(a: i64, b: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.lsr.i.p"]
        pub fn S2_lsr_i_p(a: i64, b: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.lsr.i.p.acc"]
        pub fn S2_lsr_i_p_acc(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.lsr.i.p.and"]
        pub fn S2_lsr_i_p_and(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.lsr.i.p.nac"]
        pub fn S2_lsr_i_p_nac(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.lsr.i.p.or"]
        pub fn S2_lsr_i_p_or(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.lsr.i.p.xacc"]
        pub fn S2_lsr_i_p_xacc(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.lsr.i.r"]
        pub fn S2_lsr_i_r(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.lsr.i.r.acc"]
        pub fn S2_lsr_i_r_acc(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.lsr.i.r.and"]
        pub fn S2_lsr_i_r_and(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.lsr.i.r.nac"]
        pub fn S2_lsr_i_r_nac(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.lsr.i.r.or"]
        pub fn S2_lsr_i_r_or(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.lsr.i.r.xacc"]
        pub fn S2_lsr_i_r_xacc(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.lsr.i.vh"]
        pub fn S2_lsr_i_vh(a: i64, b: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.lsr.i.vw"]
        pub fn S2_lsr_i_vw(a: i64, b: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.lsr.r.p"]
        pub fn S2_lsr_r_p(a: i64, b: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.lsr.r.p.acc"]
        pub fn S2_lsr_r_p_acc(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.lsr.r.p.and"]
        pub fn S2_lsr_r_p_and(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.lsr.r.p.nac"]
        pub fn S2_lsr_r_p_nac(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.lsr.r.p.or"]
        pub fn S2_lsr_r_p_or(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.lsr.r.p.xor"]
        pub fn S2_lsr_r_p_xor(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.lsr.r.r"]
        pub fn S2_lsr_r_r(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.lsr.r.r.acc"]
        pub fn S2_lsr_r_r_acc(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.lsr.r.r.and"]
        pub fn S2_lsr_r_r_and(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.lsr.r.r.nac"]
        pub fn S2_lsr_r_r_nac(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.lsr.r.r.or"]
        pub fn S2_lsr_r_r_or(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.lsr.r.vh"]
        pub fn S2_lsr_r_vh(a: i64, b: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.lsr.r.vw"]
        pub fn S2_lsr_r_vw(a: i64, b: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.packhl"]
        pub fn S2_packhl(a: i32, b: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.parityp"]
        pub fn S2_parityp(a: i64, b: i64) -> i32;
        #[link_name = "llvm.hexagon.S2.setbit.i"]
        pub fn S2_setbit_i(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.setbit.r"]
        pub fn S2_setbit_r(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.shuffeb"]
        pub fn S2_shuffeb(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.S2.shuffeh"]
        pub fn S2_shuffeh(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.S2.shuffob"]
        pub fn S2_shuffob(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.S2.shuffoh"]
        pub fn S2_shuffoh(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.S2.svsathb"]
        pub fn S2_svsathb(a: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.svsathub"]
        pub fn S2_svsathub(a: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.tableidxb.goodsyntax"]
        pub fn S2_tableidxb_goodsyntax(a: i32, b: i32, c: i32, d: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.tableidxd.goodsyntax"]
        pub fn S2_tableidxd_goodsyntax(a: i32, b: i32, c: i32, d: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.tableidxh.goodsyntax"]
        pub fn S2_tableidxh_goodsyntax(a: i32, b: i32, c: i32, d: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.tableidxw.goodsyntax"]
        pub fn S2_tableidxw_goodsyntax(a: i32, b: i32, c: i32, d: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.togglebit.i"]
        pub fn S2_togglebit_i(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.togglebit.r"]
        pub fn S2_togglebit_r(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.tstbit.i"]
        pub fn S2_tstbit_i(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.S2.tstbit.r"]
        pub fn S2_tstbit_r(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.S2.valignib"]
        pub fn S2_valignib(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.valignrb"]
        pub fn S2_valignrb(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.vcnegh"]
        pub fn S2_vcnegh(a: i64, b: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.vcrotate"]
        pub fn S2_vcrotate(a: i64, b: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.vrcnegh"]
        pub fn S2_vrcnegh(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.vrndpackwh"]
        pub fn S2_vrndpackwh(a: i64) -> i32;
        #[link_name = "llvm.hexagon.S2.vrndpackwhs"]
        pub fn S2_vrndpackwhs(a: i64) -> i32;
        #[link_name = "llvm.hexagon.S2.vsathb"]
        pub fn S2_vsathb(a: i64) -> i32;
        #[link_name = "llvm.hexagon.S2.vsathb.nopack"]
        pub fn S2_vsathb_nopack(a: i64) -> i64;
        #[link_name = "llvm.hexagon.S2.vsathub"]
        pub fn S2_vsathub(a: i64) -> i32;
        #[link_name = "llvm.hexagon.S2.vsathub.nopack"]
        pub fn S2_vsathub_nopack(a: i64) -> i64;
        #[link_name = "llvm.hexagon.S2.vsatwh"]
        pub fn S2_vsatwh(a: i64) -> i32;
        #[link_name = "llvm.hexagon.S2.vsatwh.nopack"]
        pub fn S2_vsatwh_nopack(a: i64) -> i64;
        #[link_name = "llvm.hexagon.S2.vsatwuh"]
        pub fn S2_vsatwuh(a: i64) -> i32;
        #[link_name = "llvm.hexagon.S2.vsatwuh.nopack"]
        pub fn S2_vsatwuh_nopack(a: i64) -> i64;
        #[link_name = "llvm.hexagon.S2.vsplatrb"]
        pub fn S2_vsplatrb(a: i32) -> i32;
        #[link_name = "llvm.hexagon.S2.vsplatrh"]
        pub fn S2_vsplatrh(a: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.vspliceib"]
        pub fn S2_vspliceib(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.vsplicerb"]
        pub fn S2_vsplicerb(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.vsxtbh"]
        pub fn S2_vsxtbh(a: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.vsxthw"]
        pub fn S2_vsxthw(a: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.vtrunehb"]
        pub fn S2_vtrunehb(a: i64) -> i32;
        #[link_name = "llvm.hexagon.S2.vtrunewh"]
        pub fn S2_vtrunewh(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.S2.vtrunohb"]
        pub fn S2_vtrunohb(a: i64) -> i32;
        #[link_name = "llvm.hexagon.S2.vtrunowh"]
        pub fn S2_vtrunowh(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.S2.vzxtbh"]
        pub fn S2_vzxtbh(a: i32) -> i64;
        #[link_name = "llvm.hexagon.S2.vzxthw"]
        pub fn S2_vzxthw(a: i32) -> i64;
        #[link_name = "llvm.hexagon.S4.addaddi"]
        pub fn S4_addaddi(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S4.addi.asl.ri"]
        pub fn S4_addi_asl_ri(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S4.addi.lsr.ri"]
        pub fn S4_addi_lsr_ri(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S4.andi.asl.ri"]
        pub fn S4_andi_asl_ri(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S4.andi.lsr.ri"]
        pub fn S4_andi_lsr_ri(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S4.clbaddi"]
        pub fn S4_clbaddi(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.S4.clbpaddi"]
        pub fn S4_clbpaddi(a: i64, b: i32) -> i32;
        #[link_name = "llvm.hexagon.S4.clbpnorm"]
        pub fn S4_clbpnorm(a: i64) -> i32;
        #[link_name = "llvm.hexagon.S4.extract"]
        pub fn S4_extract(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S4.extract.rp"]
        pub fn S4_extract_rp(a: i32, b: i64) -> i32;
        #[link_name = "llvm.hexagon.S4.extractp"]
        pub fn S4_extractp(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S4.extractp.rp"]
        pub fn S4_extractp_rp(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.S4.lsli"]
        pub fn S4_lsli(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.S4.ntstbit.i"]
        pub fn S4_ntstbit_i(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.S4.ntstbit.r"]
        pub fn S4_ntstbit_r(a: i32, b: i32) -> bool;
        #[link_name = "llvm.hexagon.S4.or.andi"]
        pub fn S4_or_andi(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S4.or.andix"]
        pub fn S4_or_andix(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S4.or.ori"]
        pub fn S4_or_ori(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S4.ori.asl.ri"]
        pub fn S4_ori_asl_ri(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S4.ori.lsr.ri"]
        pub fn S4_ori_lsr_ri(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S4.parity"]
        pub fn S4_parity(a: i32, b: i32) -> i32;
        #[link_name = "llvm.hexagon.S4.subaddi"]
        pub fn S4_subaddi(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S4.subi.asl.ri"]
        pub fn S4_subi_asl_ri(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S4.subi.lsr.ri"]
        pub fn S4_subi_lsr_ri(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.hexagon.S4.vrcrotate"]
        pub fn S4_vrcrotate(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.hexagon.S4.vrcrotate.acc"]
        pub fn S4_vrcrotate_acc(a: i64, b: i64, c: i32, d: i32) -> i64;
        #[link_name = "llvm.hexagon.S4.vxaddsubh"]
        pub fn S4_vxaddsubh(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.S4.vxaddsubhr"]
        pub fn S4_vxaddsubhr(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.S4.vxaddsubw"]
        pub fn S4_vxaddsubw(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.S4.vxsubaddh"]
        pub fn S4_vxsubaddh(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.S4.vxsubaddhr"]
        pub fn S4_vxsubaddhr(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.S4.vxsubaddw"]
        pub fn S4_vxsubaddw(a: i64, b: i64) -> i64;
        #[link_name = "llvm.hexagon.S5.asrhub.rnd.sat.goodsyntax"]
        pub fn S5_asrhub_rnd_sat_goodsyntax(a: i64, b: i32) -> i32;
        #[link_name = "llvm.hexagon.S5.asrhub.sat"]
        pub fn S5_asrhub_sat(a: i64, b: i32) -> i32;
        #[link_name = "llvm.hexagon.S5.popcountp"]
        pub fn S5_popcountp(a: i64) -> i32;
        #[link_name = "llvm.hexagon.S5.vasrhrnd.goodsyntax"]
        pub fn S5_vasrhrnd_goodsyntax(a: i64, b: i32) -> i64;
        #[link_name = "llvm.hexagon.SI.to.SXTHI.asrh"]
        pub fn SI_to_SXTHI_asrh(a: i32) -> i32;
        #[link_name = "llvm.hexagon.circ.ldd"]
        pub fn circ_ldd(a: *mut i8, b: *mut i8, c: i32, d: i32) -> *mut i8;
    }
}
/// LLVM intrinsics for the mips architecture.
pub mod mips {
    extern {
        #[link_name = "llvm.mips.add.a.b"]
        pub fn add_a_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.add.a.d"]
        pub fn add_a_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.add.a.h"]
        pub fn add_a_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.add.a.w"]
        pub fn add_a_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.adds.a.b"]
        pub fn adds_a_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.adds.a.d"]
        pub fn adds_a_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.adds.a.h"]
        pub fn adds_a_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.adds.a.w"]
        pub fn adds_a_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.adds.s.b"]
        pub fn adds_s_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.adds.s.d"]
        pub fn adds_s_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.adds.s.h"]
        pub fn adds_s_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.adds.s.w"]
        pub fn adds_s_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.adds.u.b"]
        pub fn adds_u_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.adds.u.d"]
        pub fn adds_u_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.adds.u.h"]
        pub fn adds_u_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.adds.u.w"]
        pub fn adds_u_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.addsc"]
        pub fn addsc(a: i32, b: i32) -> i32;
        #[link_name = "llvm.mips.addu.ph"]
        pub fn addu_ph(a: ::simdty::i16x2, b: ::simdty::i16x2) -> ::simdty::i16x2;
        #[link_name = "llvm.mips.addu.qb"]
        pub fn addu_qb(a: ::simdty::i8x4, b: ::simdty::i8x4) -> ::simdty::i8x4;
        #[link_name = "llvm.mips.addu.s.ph"]
        pub fn addu_s_ph(a: ::simdty::i16x2, b: ::simdty::i16x2) -> ::simdty::i16x2;
        #[link_name = "llvm.mips.addu.s.qb"]
        pub fn addu_s_qb(a: ::simdty::i8x4, b: ::simdty::i8x4) -> ::simdty::i8x4;
        #[link_name = "llvm.mips.adduh.qb"]
        pub fn adduh_qb(a: ::simdty::i8x4, b: ::simdty::i8x4) -> ::simdty::i8x4;
        #[link_name = "llvm.mips.adduh.r.qb"]
        pub fn adduh_r_qb(a: ::simdty::i8x4, b: ::simdty::i8x4) -> ::simdty::i8x4;
        #[link_name = "llvm.mips.addv.b"]
        pub fn addv_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.addv.d"]
        pub fn addv_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.addv.h"]
        pub fn addv_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.addv.w"]
        pub fn addv_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.addvi.b"]
        pub fn addvi_b(a: ::simdty::i8x16, b: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.addvi.d"]
        pub fn addvi_d(a: ::simdty::i64x2, b: i32) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.addvi.h"]
        pub fn addvi_h(a: ::simdty::i16x8, b: i32) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.addvi.w"]
        pub fn addvi_w(a: ::simdty::i32x4, b: i32) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.addwc"]
        pub fn addwc(a: i32, b: i32) -> i32;
        #[link_name = "llvm.mips.and.v"]
        pub fn and_v(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.andi.b"]
        pub fn andi_b(a: ::simdty::i8x16, b: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.append"]
        pub fn append(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.mips.asub.s.b"]
        pub fn asub_s_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.asub.s.d"]
        pub fn asub_s_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.asub.s.h"]
        pub fn asub_s_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.asub.s.w"]
        pub fn asub_s_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.asub.u.b"]
        pub fn asub_u_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.asub.u.d"]
        pub fn asub_u_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.asub.u.h"]
        pub fn asub_u_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.asub.u.w"]
        pub fn asub_u_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.ave.s.b"]
        pub fn ave_s_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.ave.s.d"]
        pub fn ave_s_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.ave.s.h"]
        pub fn ave_s_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.ave.s.w"]
        pub fn ave_s_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.ave.u.b"]
        pub fn ave_u_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.ave.u.d"]
        pub fn ave_u_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.ave.u.h"]
        pub fn ave_u_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.ave.u.w"]
        pub fn ave_u_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.aver.s.b"]
        pub fn aver_s_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.aver.s.d"]
        pub fn aver_s_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.aver.s.h"]
        pub fn aver_s_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.aver.s.w"]
        pub fn aver_s_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.aver.u.b"]
        pub fn aver_u_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.aver.u.d"]
        pub fn aver_u_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.aver.u.h"]
        pub fn aver_u_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.aver.u.w"]
        pub fn aver_u_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.balign"]
        pub fn balign(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.mips.bclr.b"]
        pub fn bclr_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.bclr.d"]
        pub fn bclr_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.bclr.h"]
        pub fn bclr_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.bclr.w"]
        pub fn bclr_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.bclri.b"]
        pub fn bclri_b(a: ::simdty::i8x16, b: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.bclri.d"]
        pub fn bclri_d(a: ::simdty::i64x2, b: i32) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.bclri.h"]
        pub fn bclri_h(a: ::simdty::i16x8, b: i32) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.bclri.w"]
        pub fn bclri_w(a: ::simdty::i32x4, b: i32) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.binsl.b"]
        pub fn binsl_b(a: ::simdty::i8x16, b: ::simdty::i8x16, c: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.binsl.d"]
        pub fn binsl_d(a: ::simdty::i64x2, b: ::simdty::i64x2, c: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.binsl.h"]
        pub fn binsl_h(a: ::simdty::i16x8, b: ::simdty::i16x8, c: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.binsl.w"]
        pub fn binsl_w(a: ::simdty::i32x4, b: ::simdty::i32x4, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.binsli.b"]
        pub fn binsli_b(a: ::simdty::i8x16, b: ::simdty::i8x16, c: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.binsli.d"]
        pub fn binsli_d(a: ::simdty::i64x2, b: ::simdty::i64x2, c: i32) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.binsli.h"]
        pub fn binsli_h(a: ::simdty::i16x8, b: ::simdty::i16x8, c: i32) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.binsli.w"]
        pub fn binsli_w(a: ::simdty::i32x4, b: ::simdty::i32x4, c: i32) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.binsr.b"]
        pub fn binsr_b(a: ::simdty::i8x16, b: ::simdty::i8x16, c: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.binsr.d"]
        pub fn binsr_d(a: ::simdty::i64x2, b: ::simdty::i64x2, c: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.binsr.h"]
        pub fn binsr_h(a: ::simdty::i16x8, b: ::simdty::i16x8, c: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.binsr.w"]
        pub fn binsr_w(a: ::simdty::i32x4, b: ::simdty::i32x4, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.binsri.b"]
        pub fn binsri_b(a: ::simdty::i8x16, b: ::simdty::i8x16, c: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.binsri.d"]
        pub fn binsri_d(a: ::simdty::i64x2, b: ::simdty::i64x2, c: i32) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.binsri.h"]
        pub fn binsri_h(a: ::simdty::i16x8, b: ::simdty::i16x8, c: i32) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.binsri.w"]
        pub fn binsri_w(a: ::simdty::i32x4, b: ::simdty::i32x4, c: i32) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.bitrev"]
        pub fn bitrev(a: i32) -> i32;
        #[link_name = "llvm.mips.bmnz.v"]
        pub fn bmnz_v(a: ::simdty::i8x16, b: ::simdty::i8x16, c: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.bmnzi.b"]
        pub fn bmnzi_b(a: ::simdty::i8x16, b: ::simdty::i8x16, c: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.bmz.v"]
        pub fn bmz_v(a: ::simdty::i8x16, b: ::simdty::i8x16, c: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.bmzi.b"]
        pub fn bmzi_b(a: ::simdty::i8x16, b: ::simdty::i8x16, c: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.bneg.b"]
        pub fn bneg_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.bneg.d"]
        pub fn bneg_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.bneg.h"]
        pub fn bneg_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.bneg.w"]
        pub fn bneg_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.bnegi.b"]
        pub fn bnegi_b(a: ::simdty::i8x16, b: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.bnegi.d"]
        pub fn bnegi_d(a: ::simdty::i64x2, b: i32) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.bnegi.h"]
        pub fn bnegi_h(a: ::simdty::i16x8, b: i32) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.bnegi.w"]
        pub fn bnegi_w(a: ::simdty::i32x4, b: i32) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.bnz.b"]
        pub fn bnz_b(a: ::simdty::i8x16) -> i32;
        #[link_name = "llvm.mips.bnz.d"]
        pub fn bnz_d(a: ::simdty::i64x2) -> i32;
        #[link_name = "llvm.mips.bnz.h"]
        pub fn bnz_h(a: ::simdty::i16x8) -> i32;
        #[link_name = "llvm.mips.bnz.v"]
        pub fn bnz_v(a: ::simdty::i8x16) -> i32;
        #[link_name = "llvm.mips.bnz.w"]
        pub fn bnz_w(a: ::simdty::i32x4) -> i32;
        #[link_name = "llvm.mips.bposge32"]
        pub fn bposge32() -> i32;
        #[link_name = "llvm.mips.bsel.v"]
        pub fn bsel_v(a: ::simdty::i8x16, b: ::simdty::i8x16, c: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.bseli.b"]
        pub fn bseli_b(a: ::simdty::i8x16, b: ::simdty::i8x16, c: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.bset.b"]
        pub fn bset_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.bset.d"]
        pub fn bset_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.bset.h"]
        pub fn bset_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.bset.w"]
        pub fn bset_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.bseti.b"]
        pub fn bseti_b(a: ::simdty::i8x16, b: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.bseti.d"]
        pub fn bseti_d(a: ::simdty::i64x2, b: i32) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.bseti.h"]
        pub fn bseti_h(a: ::simdty::i16x8, b: i32) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.bseti.w"]
        pub fn bseti_w(a: ::simdty::i32x4, b: i32) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.bz.b"]
        pub fn bz_b(a: ::simdty::i8x16) -> i32;
        #[link_name = "llvm.mips.bz.d"]
        pub fn bz_d(a: ::simdty::i64x2) -> i32;
        #[link_name = "llvm.mips.bz.h"]
        pub fn bz_h(a: ::simdty::i16x8) -> i32;
        #[link_name = "llvm.mips.bz.v"]
        pub fn bz_v(a: ::simdty::i8x16) -> i32;
        #[link_name = "llvm.mips.bz.w"]
        pub fn bz_w(a: ::simdty::i32x4) -> i32;
        #[link_name = "llvm.mips.ceq.b"]
        pub fn ceq_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.ceq.d"]
        pub fn ceq_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.ceq.h"]
        pub fn ceq_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.ceq.w"]
        pub fn ceq_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.ceqi.b"]
        pub fn ceqi_b(a: ::simdty::i8x16, b: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.ceqi.d"]
        pub fn ceqi_d(a: ::simdty::i64x2, b: i32) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.ceqi.h"]
        pub fn ceqi_h(a: ::simdty::i16x8, b: i32) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.ceqi.w"]
        pub fn ceqi_w(a: ::simdty::i32x4, b: i32) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.cfcmsa"]
        pub fn cfcmsa(a: i32) -> i32;
        #[link_name = "llvm.mips.cle.s.b"]
        pub fn cle_s_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.cle.s.d"]
        pub fn cle_s_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.cle.s.h"]
        pub fn cle_s_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.cle.s.w"]
        pub fn cle_s_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.cle.u.b"]
        pub fn cle_u_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.cle.u.d"]
        pub fn cle_u_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.cle.u.h"]
        pub fn cle_u_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.cle.u.w"]
        pub fn cle_u_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.clei.s.b"]
        pub fn clei_s_b(a: ::simdty::i8x16, b: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.clei.s.d"]
        pub fn clei_s_d(a: ::simdty::i64x2, b: i32) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.clei.s.h"]
        pub fn clei_s_h(a: ::simdty::i16x8, b: i32) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.clei.s.w"]
        pub fn clei_s_w(a: ::simdty::i32x4, b: i32) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.clei.u.b"]
        pub fn clei_u_b(a: ::simdty::i8x16, b: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.clei.u.d"]
        pub fn clei_u_d(a: ::simdty::i64x2, b: i32) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.clei.u.h"]
        pub fn clei_u_h(a: ::simdty::i16x8, b: i32) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.clei.u.w"]
        pub fn clei_u_w(a: ::simdty::i32x4, b: i32) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.clt.s.b"]
        pub fn clt_s_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.clt.s.d"]
        pub fn clt_s_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.clt.s.h"]
        pub fn clt_s_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.clt.s.w"]
        pub fn clt_s_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.clt.u.b"]
        pub fn clt_u_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.clt.u.d"]
        pub fn clt_u_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.clt.u.h"]
        pub fn clt_u_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.clt.u.w"]
        pub fn clt_u_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.clti.s.b"]
        pub fn clti_s_b(a: ::simdty::i8x16, b: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.clti.s.d"]
        pub fn clti_s_d(a: ::simdty::i64x2, b: i32) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.clti.s.h"]
        pub fn clti_s_h(a: ::simdty::i16x8, b: i32) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.clti.s.w"]
        pub fn clti_s_w(a: ::simdty::i32x4, b: i32) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.clti.u.b"]
        pub fn clti_u_b(a: ::simdty::i8x16, b: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.clti.u.d"]
        pub fn clti_u_d(a: ::simdty::i64x2, b: i32) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.clti.u.h"]
        pub fn clti_u_h(a: ::simdty::i16x8, b: i32) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.clti.u.w"]
        pub fn clti_u_w(a: ::simdty::i32x4, b: i32) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.cmpgdu.eq.qb"]
        pub fn cmpgdu_eq_qb(a: ::simdty::i8x4, b: ::simdty::i8x4) -> i32;
        #[link_name = "llvm.mips.cmpgdu.le.qb"]
        pub fn cmpgdu_le_qb(a: ::simdty::i8x4, b: ::simdty::i8x4) -> i32;
        #[link_name = "llvm.mips.cmpgdu.lt.qb"]
        pub fn cmpgdu_lt_qb(a: ::simdty::i8x4, b: ::simdty::i8x4) -> i32;
        #[link_name = "llvm.mips.cmpgu.eq.qb"]
        pub fn cmpgu_eq_qb(a: ::simdty::i8x4, b: ::simdty::i8x4) -> i32;
        #[link_name = "llvm.mips.cmpgu.le.qb"]
        pub fn cmpgu_le_qb(a: ::simdty::i8x4, b: ::simdty::i8x4) -> i32;
        #[link_name = "llvm.mips.cmpgu.lt.qb"]
        pub fn cmpgu_lt_qb(a: ::simdty::i8x4, b: ::simdty::i8x4) -> i32;
        #[link_name = "llvm.mips.cmpu.eq.qb"]
        pub fn cmpu_eq_qb(a: ::simdty::i8x4, b: ::simdty::i8x4) -> ();
        #[link_name = "llvm.mips.cmpu.le.qb"]
        pub fn cmpu_le_qb(a: ::simdty::i8x4, b: ::simdty::i8x4) -> ();
        #[link_name = "llvm.mips.cmpu.lt.qb"]
        pub fn cmpu_lt_qb(a: ::simdty::i8x4, b: ::simdty::i8x4) -> ();
        #[link_name = "llvm.mips.copy.s.b"]
        pub fn copy_s_b(a: ::simdty::i8x16, b: i32) -> i32;
        #[link_name = "llvm.mips.copy.s.d"]
        pub fn copy_s_d(a: ::simdty::i64x2, b: i32) -> i64;
        #[link_name = "llvm.mips.copy.s.h"]
        pub fn copy_s_h(a: ::simdty::i16x8, b: i32) -> i32;
        #[link_name = "llvm.mips.copy.s.w"]
        pub fn copy_s_w(a: ::simdty::i32x4, b: i32) -> i32;
        #[link_name = "llvm.mips.copy.u.b"]
        pub fn copy_u_b(a: ::simdty::i8x16, b: i32) -> i32;
        #[link_name = "llvm.mips.copy.u.d"]
        pub fn copy_u_d(a: ::simdty::i64x2, b: i32) -> i64;
        #[link_name = "llvm.mips.copy.u.h"]
        pub fn copy_u_h(a: ::simdty::i16x8, b: i32) -> i32;
        #[link_name = "llvm.mips.copy.u.w"]
        pub fn copy_u_w(a: ::simdty::i32x4, b: i32) -> i32;
        #[link_name = "llvm.mips.ctcmsa"]
        pub fn ctcmsa(a: i32, b: i32) -> ();
        #[link_name = "llvm.mips.div.s.b"]
        pub fn div_s_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.div.s.d"]
        pub fn div_s_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.div.s.h"]
        pub fn div_s_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.div.s.w"]
        pub fn div_s_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.div.u.b"]
        pub fn div_u_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.div.u.d"]
        pub fn div_u_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.div.u.h"]
        pub fn div_u_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.div.u.w"]
        pub fn div_u_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.dlsa"]
        pub fn dlsa(a: i64, b: i64, c: i32) -> i64;
        #[link_name = "llvm.mips.dotp.s.d"]
        pub fn dotp_s_d(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.dotp.s.h"]
        pub fn dotp_s_h(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.dotp.s.w"]
        pub fn dotp_s_w(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.dotp.u.d"]
        pub fn dotp_u_d(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.dotp.u.h"]
        pub fn dotp_u_h(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.dotp.u.w"]
        pub fn dotp_u_w(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.dpa.w.ph"]
        pub fn dpa_w_ph(a: i64, b: ::simdty::i16x2, c: ::simdty::i16x2) -> i64;
        #[link_name = "llvm.mips.dpadd.s.d"]
        pub fn dpadd_s_d(a: ::simdty::i64x2, b: ::simdty::i32x4, c: ::simdty::i32x4) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.dpadd.s.h"]
        pub fn dpadd_s_h(a: ::simdty::i16x8, b: ::simdty::i8x16, c: ::simdty::i8x16) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.dpadd.s.w"]
        pub fn dpadd_s_w(a: ::simdty::i32x4, b: ::simdty::i16x8, c: ::simdty::i16x8) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.dpadd.u.d"]
        pub fn dpadd_u_d(a: ::simdty::i64x2, b: ::simdty::i32x4, c: ::simdty::i32x4) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.dpadd.u.h"]
        pub fn dpadd_u_h(a: ::simdty::i16x8, b: ::simdty::i8x16, c: ::simdty::i8x16) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.dpadd.u.w"]
        pub fn dpadd_u_w(a: ::simdty::i32x4, b: ::simdty::i16x8, c: ::simdty::i16x8) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.dpau.h.qbl"]
        pub fn dpau_h_qbl(a: i64, b: ::simdty::i8x4, c: ::simdty::i8x4) -> i64;
        #[link_name = "llvm.mips.dpau.h.qbr"]
        pub fn dpau_h_qbr(a: i64, b: ::simdty::i8x4, c: ::simdty::i8x4) -> i64;
        #[link_name = "llvm.mips.dpax.w.ph"]
        pub fn dpax_w_ph(a: i64, b: ::simdty::i16x2, c: ::simdty::i16x2) -> i64;
        #[link_name = "llvm.mips.dps.w.ph"]
        pub fn dps_w_ph(a: i64, b: ::simdty::i16x2, c: ::simdty::i16x2) -> i64;
        #[link_name = "llvm.mips.dpsu.h.qbl"]
        pub fn dpsu_h_qbl(a: i64, b: ::simdty::i8x4, c: ::simdty::i8x4) -> i64;
        #[link_name = "llvm.mips.dpsu.h.qbr"]
        pub fn dpsu_h_qbr(a: i64, b: ::simdty::i8x4, c: ::simdty::i8x4) -> i64;
        #[link_name = "llvm.mips.dpsub.s.d"]
        pub fn dpsub_s_d(a: ::simdty::i64x2, b: ::simdty::i32x4, c: ::simdty::i32x4) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.dpsub.s.h"]
        pub fn dpsub_s_h(a: ::simdty::i16x8, b: ::simdty::i8x16, c: ::simdty::i8x16) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.dpsub.s.w"]
        pub fn dpsub_s_w(a: ::simdty::i32x4, b: ::simdty::i16x8, c: ::simdty::i16x8) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.dpsub.u.d"]
        pub fn dpsub_u_d(a: ::simdty::i64x2, b: ::simdty::i32x4, c: ::simdty::i32x4) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.dpsub.u.h"]
        pub fn dpsub_u_h(a: ::simdty::i16x8, b: ::simdty::i8x16, c: ::simdty::i8x16) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.dpsub.u.w"]
        pub fn dpsub_u_w(a: ::simdty::i32x4, b: ::simdty::i16x8, c: ::simdty::i16x8) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.dpsx.w.ph"]
        pub fn dpsx_w_ph(a: i64, b: ::simdty::i16x2, c: ::simdty::i16x2) -> i64;
        #[link_name = "llvm.mips.extp"]
        pub fn extp(a: i64, b: i32) -> i32;
        #[link_name = "llvm.mips.extpdp"]
        pub fn extpdp(a: i64, b: i32) -> i32;
        #[link_name = "llvm.mips.extr.r.w"]
        pub fn extr_r_w(a: i64, b: i32) -> i32;
        #[link_name = "llvm.mips.extr.rs.w"]
        pub fn extr_rs_w(a: i64, b: i32) -> i32;
        #[link_name = "llvm.mips.extr.s.h"]
        pub fn extr_s_h(a: i64, b: i32) -> i32;
        #[link_name = "llvm.mips.extr.w"]
        pub fn extr_w(a: i64, b: i32) -> i32;
        #[link_name = "llvm.mips.fadd.d"]
        pub fn fadd_d(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.mips.fadd.w"]
        pub fn fadd_w(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.mips.fcaf.d"]
        pub fn fcaf_d(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.fcaf.w"]
        pub fn fcaf_w(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.fceq.d"]
        pub fn fceq_d(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.fceq.w"]
        pub fn fceq_w(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.fclass.d"]
        pub fn fclass_d(a: ::simdty::f64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.fclass.w"]
        pub fn fclass_w(a: ::simdty::f32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.fcle.d"]
        pub fn fcle_d(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.fcle.w"]
        pub fn fcle_w(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.fclt.d"]
        pub fn fclt_d(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.fclt.w"]
        pub fn fclt_w(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.fcne.d"]
        pub fn fcne_d(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.fcne.w"]
        pub fn fcne_w(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.fcor.d"]
        pub fn fcor_d(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.fcor.w"]
        pub fn fcor_w(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.fcueq.d"]
        pub fn fcueq_d(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.fcueq.w"]
        pub fn fcueq_w(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.fcule.d"]
        pub fn fcule_d(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.fcule.w"]
        pub fn fcule_w(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.fcult.d"]
        pub fn fcult_d(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.fcult.w"]
        pub fn fcult_w(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.fcun.d"]
        pub fn fcun_d(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.fcun.w"]
        pub fn fcun_w(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.fcune.d"]
        pub fn fcune_d(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.fcune.w"]
        pub fn fcune_w(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.fdiv.d"]
        pub fn fdiv_d(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.mips.fdiv.w"]
        pub fn fdiv_w(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.mips.fexdo.w"]
        pub fn fexdo_w(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::f32x4;
        #[link_name = "llvm.mips.fexp2.d"]
        pub fn fexp2_d(a: ::simdty::f64x2, b: ::simdty::i64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.mips.fexp2.w"]
        pub fn fexp2_w(a: ::simdty::f32x4, b: ::simdty::i32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.mips.fexupl.d"]
        pub fn fexupl_d(a: ::simdty::f32x4) -> ::simdty::f64x2;
        #[link_name = "llvm.mips.fexupr.d"]
        pub fn fexupr_d(a: ::simdty::f32x4) -> ::simdty::f64x2;
        #[link_name = "llvm.mips.ffint.s.d"]
        pub fn ffint_s_d(a: ::simdty::i64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.mips.ffint.s.w"]
        pub fn ffint_s_w(a: ::simdty::i32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.mips.ffint.u.d"]
        pub fn ffint_u_d(a: ::simdty::i64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.mips.ffint.u.w"]
        pub fn ffint_u_w(a: ::simdty::i32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.mips.ffql.d"]
        pub fn ffql_d(a: ::simdty::i32x4) -> ::simdty::f64x2;
        #[link_name = "llvm.mips.ffql.w"]
        pub fn ffql_w(a: ::simdty::i16x8) -> ::simdty::f32x4;
        #[link_name = "llvm.mips.ffqr.d"]
        pub fn ffqr_d(a: ::simdty::i32x4) -> ::simdty::f64x2;
        #[link_name = "llvm.mips.ffqr.w"]
        pub fn ffqr_w(a: ::simdty::i16x8) -> ::simdty::f32x4;
        #[link_name = "llvm.mips.fill.b"]
        pub fn fill_b(a: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.fill.d"]
        pub fn fill_d(a: i64) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.fill.h"]
        pub fn fill_h(a: i32) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.fill.w"]
        pub fn fill_w(a: i32) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.flog2.d"]
        pub fn flog2_d(a: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.mips.flog2.w"]
        pub fn flog2_w(a: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.mips.fmadd.d"]
        pub fn fmadd_d(a: ::simdty::f64x2, b: ::simdty::f64x2, c: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.mips.fmadd.w"]
        pub fn fmadd_w(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.mips.fmax.a.d"]
        pub fn fmax_a_d(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.mips.fmax.a.w"]
        pub fn fmax_a_w(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.mips.fmax.d"]
        pub fn fmax_d(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.mips.fmax.w"]
        pub fn fmax_w(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.mips.fmin.a.d"]
        pub fn fmin_a_d(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.mips.fmin.a.w"]
        pub fn fmin_a_w(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.mips.fmin.d"]
        pub fn fmin_d(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.mips.fmin.w"]
        pub fn fmin_w(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.mips.fmsub.d"]
        pub fn fmsub_d(a: ::simdty::f64x2, b: ::simdty::f64x2, c: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.mips.fmsub.w"]
        pub fn fmsub_w(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.mips.fmul.d"]
        pub fn fmul_d(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.mips.fmul.w"]
        pub fn fmul_w(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.mips.frcp.d"]
        pub fn frcp_d(a: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.mips.frcp.w"]
        pub fn frcp_w(a: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.mips.frint.d"]
        pub fn frint_d(a: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.mips.frint.w"]
        pub fn frint_w(a: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.mips.frsqrt.d"]
        pub fn frsqrt_d(a: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.mips.frsqrt.w"]
        pub fn frsqrt_w(a: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.mips.fsaf.d"]
        pub fn fsaf_d(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.fsaf.w"]
        pub fn fsaf_w(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.fseq.d"]
        pub fn fseq_d(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.fseq.w"]
        pub fn fseq_w(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.fsle.d"]
        pub fn fsle_d(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.fsle.w"]
        pub fn fsle_w(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.fslt.d"]
        pub fn fslt_d(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.fslt.w"]
        pub fn fslt_w(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.fsne.d"]
        pub fn fsne_d(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.fsne.w"]
        pub fn fsne_w(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.fsor.d"]
        pub fn fsor_d(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.fsor.w"]
        pub fn fsor_w(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.fsqrt.d"]
        pub fn fsqrt_d(a: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.mips.fsqrt.w"]
        pub fn fsqrt_w(a: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.mips.fsub.d"]
        pub fn fsub_d(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.mips.fsub.w"]
        pub fn fsub_w(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.mips.fsueq.d"]
        pub fn fsueq_d(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.fsueq.w"]
        pub fn fsueq_w(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.fsule.d"]
        pub fn fsule_d(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.fsule.w"]
        pub fn fsule_w(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.fsult.d"]
        pub fn fsult_d(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.fsult.w"]
        pub fn fsult_w(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.fsun.d"]
        pub fn fsun_d(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.fsun.w"]
        pub fn fsun_w(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.fsune.d"]
        pub fn fsune_d(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.fsune.w"]
        pub fn fsune_w(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.ftint.s.d"]
        pub fn ftint_s_d(a: ::simdty::f64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.ftint.s.w"]
        pub fn ftint_s_w(a: ::simdty::f32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.ftint.u.d"]
        pub fn ftint_u_d(a: ::simdty::f64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.ftint.u.w"]
        pub fn ftint_u_w(a: ::simdty::f32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.ftq.h"]
        pub fn ftq_h(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.ftq.w"]
        pub fn ftq_w(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.ftrunc.s.d"]
        pub fn ftrunc_s_d(a: ::simdty::f64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.ftrunc.s.w"]
        pub fn ftrunc_s_w(a: ::simdty::f32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.ftrunc.u.d"]
        pub fn ftrunc_u_d(a: ::simdty::f64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.ftrunc.u.w"]
        pub fn ftrunc_u_w(a: ::simdty::f32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.hadd.s.d"]
        pub fn hadd_s_d(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.hadd.s.h"]
        pub fn hadd_s_h(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.hadd.s.w"]
        pub fn hadd_s_w(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.hadd.u.d"]
        pub fn hadd_u_d(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.hadd.u.h"]
        pub fn hadd_u_h(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.hadd.u.w"]
        pub fn hadd_u_w(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.hsub.s.d"]
        pub fn hsub_s_d(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.hsub.s.h"]
        pub fn hsub_s_h(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.hsub.s.w"]
        pub fn hsub_s_w(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.hsub.u.d"]
        pub fn hsub_u_d(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.hsub.u.h"]
        pub fn hsub_u_h(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.hsub.u.w"]
        pub fn hsub_u_w(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.ilvev.b"]
        pub fn ilvev_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.ilvev.d"]
        pub fn ilvev_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.ilvev.h"]
        pub fn ilvev_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.ilvev.w"]
        pub fn ilvev_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.ilvl.b"]
        pub fn ilvl_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.ilvl.d"]
        pub fn ilvl_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.ilvl.h"]
        pub fn ilvl_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.ilvl.w"]
        pub fn ilvl_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.ilvod.b"]
        pub fn ilvod_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.ilvod.d"]
        pub fn ilvod_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.ilvod.h"]
        pub fn ilvod_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.ilvod.w"]
        pub fn ilvod_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.ilvr.b"]
        pub fn ilvr_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.ilvr.d"]
        pub fn ilvr_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.ilvr.h"]
        pub fn ilvr_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.ilvr.w"]
        pub fn ilvr_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.insert.b"]
        pub fn insert_b(a: ::simdty::i8x16, b: i32, c: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.insert.d"]
        pub fn insert_d(a: ::simdty::i64x2, b: i32, c: i64) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.insert.h"]
        pub fn insert_h(a: ::simdty::i16x8, b: i32, c: i32) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.insert.w"]
        pub fn insert_w(a: ::simdty::i32x4, b: i32, c: i32) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.insv"]
        pub fn insv(a: i32, b: i32) -> i32;
        #[link_name = "llvm.mips.insve.b"]
        pub fn insve_b(a: ::simdty::i8x16, b: i32, c: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.insve.d"]
        pub fn insve_d(a: ::simdty::i64x2, b: i32, c: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.insve.h"]
        pub fn insve_h(a: ::simdty::i16x8, b: i32, c: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.insve.w"]
        pub fn insve_w(a: ::simdty::i32x4, b: i32, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.lbux"]
        pub fn lbux(a: *mut i8, b: i32) -> i32;
        #[link_name = "llvm.mips.ld.b"]
        pub fn ld_b(a: *mut i8, b: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.ld.d"]
        pub fn ld_d(a: *mut i8, b: i32) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.ld.h"]
        pub fn ld_h(a: *mut i8, b: i32) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.ld.w"]
        pub fn ld_w(a: *mut i8, b: i32) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.ldi.b"]
        pub fn ldi_b(a: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.ldi.d"]
        pub fn ldi_d(a: i32) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.ldi.h"]
        pub fn ldi_h(a: i32) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.ldi.w"]
        pub fn ldi_w(a: i32) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.lhx"]
        pub fn lhx(a: *mut i8, b: i32) -> i32;
        #[link_name = "llvm.mips.lsa"]
        pub fn lsa(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.mips.lwx"]
        pub fn lwx(a: *mut i8, b: i32) -> i32;
        #[link_name = "llvm.mips.madd"]
        pub fn madd(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.mips.madd.q.h"]
        pub fn madd_q_h(a: ::simdty::i16x8, b: ::simdty::i16x8, c: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.madd.q.w"]
        pub fn madd_q_w(a: ::simdty::i32x4, b: ::simdty::i32x4, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.maddr.q.h"]
        pub fn maddr_q_h(a: ::simdty::i16x8, b: ::simdty::i16x8, c: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.maddr.q.w"]
        pub fn maddr_q_w(a: ::simdty::i32x4, b: ::simdty::i32x4, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.maddu"]
        pub fn maddu(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.mips.maddv.b"]
        pub fn maddv_b(a: ::simdty::i8x16, b: ::simdty::i8x16, c: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.maddv.d"]
        pub fn maddv_d(a: ::simdty::i64x2, b: ::simdty::i64x2, c: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.maddv.h"]
        pub fn maddv_h(a: ::simdty::i16x8, b: ::simdty::i16x8, c: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.maddv.w"]
        pub fn maddv_w(a: ::simdty::i32x4, b: ::simdty::i32x4, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.max.a.b"]
        pub fn max_a_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.max.a.d"]
        pub fn max_a_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.max.a.h"]
        pub fn max_a_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.max.a.w"]
        pub fn max_a_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.max.s.b"]
        pub fn max_s_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.max.s.d"]
        pub fn max_s_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.max.s.h"]
        pub fn max_s_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.max.s.w"]
        pub fn max_s_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.max.u.b"]
        pub fn max_u_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.max.u.d"]
        pub fn max_u_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.max.u.h"]
        pub fn max_u_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.max.u.w"]
        pub fn max_u_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.maxi.s.b"]
        pub fn maxi_s_b(a: ::simdty::i8x16, b: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.maxi.s.d"]
        pub fn maxi_s_d(a: ::simdty::i64x2, b: i32) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.maxi.s.h"]
        pub fn maxi_s_h(a: ::simdty::i16x8, b: i32) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.maxi.s.w"]
        pub fn maxi_s_w(a: ::simdty::i32x4, b: i32) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.maxi.u.b"]
        pub fn maxi_u_b(a: ::simdty::i8x16, b: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.maxi.u.d"]
        pub fn maxi_u_d(a: ::simdty::i64x2, b: i32) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.maxi.u.h"]
        pub fn maxi_u_h(a: ::simdty::i16x8, b: i32) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.maxi.u.w"]
        pub fn maxi_u_w(a: ::simdty::i32x4, b: i32) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.min.a.b"]
        pub fn min_a_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.min.a.d"]
        pub fn min_a_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.min.a.h"]
        pub fn min_a_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.min.a.w"]
        pub fn min_a_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.min.s.b"]
        pub fn min_s_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.min.s.d"]
        pub fn min_s_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.min.s.h"]
        pub fn min_s_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.min.s.w"]
        pub fn min_s_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.min.u.b"]
        pub fn min_u_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.min.u.d"]
        pub fn min_u_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.min.u.h"]
        pub fn min_u_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.min.u.w"]
        pub fn min_u_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.mini.s.b"]
        pub fn mini_s_b(a: ::simdty::i8x16, b: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.mini.s.d"]
        pub fn mini_s_d(a: ::simdty::i64x2, b: i32) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.mini.s.h"]
        pub fn mini_s_h(a: ::simdty::i16x8, b: i32) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.mini.s.w"]
        pub fn mini_s_w(a: ::simdty::i32x4, b: i32) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.mini.u.b"]
        pub fn mini_u_b(a: ::simdty::i8x16, b: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.mini.u.d"]
        pub fn mini_u_d(a: ::simdty::i64x2, b: i32) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.mini.u.h"]
        pub fn mini_u_h(a: ::simdty::i16x8, b: i32) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.mini.u.w"]
        pub fn mini_u_w(a: ::simdty::i32x4, b: i32) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.mod.s.b"]
        pub fn mod_s_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.mod.s.d"]
        pub fn mod_s_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.mod.s.h"]
        pub fn mod_s_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.mod.s.w"]
        pub fn mod_s_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.mod.u.b"]
        pub fn mod_u_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.mod.u.d"]
        pub fn mod_u_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.mod.u.h"]
        pub fn mod_u_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.mod.u.w"]
        pub fn mod_u_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.modsub"]
        pub fn modsub(a: i32, b: i32) -> i32;
        #[link_name = "llvm.mips.move.v"]
        pub fn move_v(a: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.msub"]
        pub fn msub(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.mips.msub.q.h"]
        pub fn msub_q_h(a: ::simdty::i16x8, b: ::simdty::i16x8, c: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.msub.q.w"]
        pub fn msub_q_w(a: ::simdty::i32x4, b: ::simdty::i32x4, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.msubr.q.h"]
        pub fn msubr_q_h(a: ::simdty::i16x8, b: ::simdty::i16x8, c: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.msubr.q.w"]
        pub fn msubr_q_w(a: ::simdty::i32x4, b: ::simdty::i32x4, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.msubu"]
        pub fn msubu(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.mips.msubv.b"]
        pub fn msubv_b(a: ::simdty::i8x16, b: ::simdty::i8x16, c: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.msubv.d"]
        pub fn msubv_d(a: ::simdty::i64x2, b: ::simdty::i64x2, c: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.msubv.h"]
        pub fn msubv_h(a: ::simdty::i16x8, b: ::simdty::i16x8, c: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.msubv.w"]
        pub fn msubv_w(a: ::simdty::i32x4, b: ::simdty::i32x4, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.mthlip"]
        pub fn mthlip(a: i64, b: i32) -> i64;
        #[link_name = "llvm.mips.mul.ph"]
        pub fn mul_ph(a: ::simdty::i16x2, b: ::simdty::i16x2) -> ::simdty::i16x2;
        #[link_name = "llvm.mips.mul.q.h"]
        pub fn mul_q_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.mul.q.w"]
        pub fn mul_q_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.mul.s.ph"]
        pub fn mul_s_ph(a: ::simdty::i16x2, b: ::simdty::i16x2) -> ::simdty::i16x2;
        #[link_name = "llvm.mips.mulr.q.h"]
        pub fn mulr_q_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.mulr.q.w"]
        pub fn mulr_q_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.mulsa.w.ph"]
        pub fn mulsa_w_ph(a: i64, b: ::simdty::i16x2, c: ::simdty::i16x2) -> i64;
        #[link_name = "llvm.mips.mult"]
        pub fn mult(a: i32, b: i32) -> i64;
        #[link_name = "llvm.mips.multu"]
        pub fn multu(a: i32, b: i32) -> i64;
        #[link_name = "llvm.mips.mulv.b"]
        pub fn mulv_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.mulv.d"]
        pub fn mulv_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.mulv.h"]
        pub fn mulv_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.mulv.w"]
        pub fn mulv_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.nloc.b"]
        pub fn nloc_b(a: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.nloc.d"]
        pub fn nloc_d(a: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.nloc.h"]
        pub fn nloc_h(a: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.nloc.w"]
        pub fn nloc_w(a: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.nlzc.b"]
        pub fn nlzc_b(a: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.nlzc.d"]
        pub fn nlzc_d(a: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.nlzc.h"]
        pub fn nlzc_h(a: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.nlzc.w"]
        pub fn nlzc_w(a: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.nor.v"]
        pub fn nor_v(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.nori.b"]
        pub fn nori_b(a: ::simdty::i8x16, b: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.or.v"]
        pub fn or_v(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.ori.b"]
        pub fn ori_b(a: ::simdty::i8x16, b: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.pckev.b"]
        pub fn pckev_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.pckev.d"]
        pub fn pckev_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.pckev.h"]
        pub fn pckev_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.pckev.w"]
        pub fn pckev_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.pckod.b"]
        pub fn pckod_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.pckod.d"]
        pub fn pckod_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.pckod.h"]
        pub fn pckod_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.pckod.w"]
        pub fn pckod_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.pcnt.b"]
        pub fn pcnt_b(a: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.pcnt.d"]
        pub fn pcnt_d(a: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.pcnt.h"]
        pub fn pcnt_h(a: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.pcnt.w"]
        pub fn pcnt_w(a: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.pick.qb"]
        pub fn pick_qb(a: ::simdty::i8x4, b: ::simdty::i8x4) -> ::simdty::i8x4;
        #[link_name = "llvm.mips.precr.qb.ph"]
        pub fn precr_qb_ph(a: ::simdty::i16x2, b: ::simdty::i16x2) -> ::simdty::i8x4;
        #[link_name = "llvm.mips.precr.sra.ph.w"]
        pub fn precr_sra_ph_w(a: i32, b: i32, c: i32) -> ::simdty::i16x2;
        #[link_name = "llvm.mips.precr.sra.r.ph.w"]
        pub fn precr_sra_r_ph_w(a: i32, b: i32, c: i32) -> ::simdty::i16x2;
        #[link_name = "llvm.mips.prepend"]
        pub fn prepend(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.mips.raddu.w.qb"]
        pub fn raddu_w_qb(a: ::simdty::i8x4) -> i32;
        #[link_name = "llvm.mips.rddsp"]
        pub fn rddsp(a: i32) -> i32;
        #[link_name = "llvm.mips.repl.qb"]
        pub fn repl_qb(a: i32) -> ::simdty::i8x4;
        #[link_name = "llvm.mips.sat.s.b"]
        pub fn sat_s_b(a: ::simdty::i8x16, b: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.sat.s.d"]
        pub fn sat_s_d(a: ::simdty::i64x2, b: i32) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.sat.s.h"]
        pub fn sat_s_h(a: ::simdty::i16x8, b: i32) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.sat.s.w"]
        pub fn sat_s_w(a: ::simdty::i32x4, b: i32) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.sat.u.b"]
        pub fn sat_u_b(a: ::simdty::i8x16, b: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.sat.u.d"]
        pub fn sat_u_d(a: ::simdty::i64x2, b: i32) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.sat.u.h"]
        pub fn sat_u_h(a: ::simdty::i16x8, b: i32) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.sat.u.w"]
        pub fn sat_u_w(a: ::simdty::i32x4, b: i32) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.shf.b"]
        pub fn shf_b(a: ::simdty::i8x16, b: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.shf.h"]
        pub fn shf_h(a: ::simdty::i16x8, b: i32) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.shf.w"]
        pub fn shf_w(a: ::simdty::i32x4, b: i32) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.shilo"]
        pub fn shilo(a: i64, b: i32) -> i64;
        #[link_name = "llvm.mips.shll.qb"]
        pub fn shll_qb(a: ::simdty::i8x4, b: i32) -> ::simdty::i8x4;
        #[link_name = "llvm.mips.shra.qb"]
        pub fn shra_qb(a: ::simdty::i8x4, b: i32) -> ::simdty::i8x4;
        #[link_name = "llvm.mips.shra.r.qb"]
        pub fn shra_r_qb(a: ::simdty::i8x4, b: i32) -> ::simdty::i8x4;
        #[link_name = "llvm.mips.shrl.ph"]
        pub fn shrl_ph(a: ::simdty::i16x2, b: i32) -> ::simdty::i16x2;
        #[link_name = "llvm.mips.shrl.qb"]
        pub fn shrl_qb(a: ::simdty::i8x4, b: i32) -> ::simdty::i8x4;
        #[link_name = "llvm.mips.sld.b"]
        pub fn sld_b(a: ::simdty::i8x16, b: ::simdty::i8x16, c: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.sld.d"]
        pub fn sld_d(a: ::simdty::i64x2, b: ::simdty::i64x2, c: i32) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.sld.h"]
        pub fn sld_h(a: ::simdty::i16x8, b: ::simdty::i16x8, c: i32) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.sld.w"]
        pub fn sld_w(a: ::simdty::i32x4, b: ::simdty::i32x4, c: i32) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.sldi.b"]
        pub fn sldi_b(a: ::simdty::i8x16, b: ::simdty::i8x16, c: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.sldi.d"]
        pub fn sldi_d(a: ::simdty::i64x2, b: ::simdty::i64x2, c: i32) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.sldi.h"]
        pub fn sldi_h(a: ::simdty::i16x8, b: ::simdty::i16x8, c: i32) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.sldi.w"]
        pub fn sldi_w(a: ::simdty::i32x4, b: ::simdty::i32x4, c: i32) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.sll.b"]
        pub fn sll_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.sll.d"]
        pub fn sll_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.sll.h"]
        pub fn sll_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.sll.w"]
        pub fn sll_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.slli.b"]
        pub fn slli_b(a: ::simdty::i8x16, b: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.slli.d"]
        pub fn slli_d(a: ::simdty::i64x2, b: i32) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.slli.h"]
        pub fn slli_h(a: ::simdty::i16x8, b: i32) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.slli.w"]
        pub fn slli_w(a: ::simdty::i32x4, b: i32) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.splat.b"]
        pub fn splat_b(a: ::simdty::i8x16, b: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.splat.d"]
        pub fn splat_d(a: ::simdty::i64x2, b: i32) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.splat.h"]
        pub fn splat_h(a: ::simdty::i16x8, b: i32) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.splat.w"]
        pub fn splat_w(a: ::simdty::i32x4, b: i32) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.splati.b"]
        pub fn splati_b(a: ::simdty::i8x16, b: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.splati.d"]
        pub fn splati_d(a: ::simdty::i64x2, b: i32) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.splati.h"]
        pub fn splati_h(a: ::simdty::i16x8, b: i32) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.splati.w"]
        pub fn splati_w(a: ::simdty::i32x4, b: i32) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.sra.b"]
        pub fn sra_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.sra.d"]
        pub fn sra_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.sra.h"]
        pub fn sra_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.sra.w"]
        pub fn sra_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.srai.b"]
        pub fn srai_b(a: ::simdty::i8x16, b: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.srai.d"]
        pub fn srai_d(a: ::simdty::i64x2, b: i32) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.srai.h"]
        pub fn srai_h(a: ::simdty::i16x8, b: i32) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.srai.w"]
        pub fn srai_w(a: ::simdty::i32x4, b: i32) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.srar.b"]
        pub fn srar_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.srar.d"]
        pub fn srar_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.srar.h"]
        pub fn srar_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.srar.w"]
        pub fn srar_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.srari.b"]
        pub fn srari_b(a: ::simdty::i8x16, b: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.srari.d"]
        pub fn srari_d(a: ::simdty::i64x2, b: i32) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.srari.h"]
        pub fn srari_h(a: ::simdty::i16x8, b: i32) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.srari.w"]
        pub fn srari_w(a: ::simdty::i32x4, b: i32) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.srl.b"]
        pub fn srl_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.srl.d"]
        pub fn srl_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.srl.h"]
        pub fn srl_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.srl.w"]
        pub fn srl_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.srli.b"]
        pub fn srli_b(a: ::simdty::i8x16, b: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.srli.d"]
        pub fn srli_d(a: ::simdty::i64x2, b: i32) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.srli.h"]
        pub fn srli_h(a: ::simdty::i16x8, b: i32) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.srli.w"]
        pub fn srli_w(a: ::simdty::i32x4, b: i32) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.srlr.b"]
        pub fn srlr_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.srlr.d"]
        pub fn srlr_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.srlr.h"]
        pub fn srlr_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.srlr.w"]
        pub fn srlr_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.srlri.b"]
        pub fn srlri_b(a: ::simdty::i8x16, b: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.srlri.d"]
        pub fn srlri_d(a: ::simdty::i64x2, b: i32) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.srlri.h"]
        pub fn srlri_h(a: ::simdty::i16x8, b: i32) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.srlri.w"]
        pub fn srlri_w(a: ::simdty::i32x4, b: i32) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.st.b"]
        pub fn st_b(a: ::simdty::i8x16, b: *mut i8, c: i32) -> ();
        #[link_name = "llvm.mips.st.d"]
        pub fn st_d(a: ::simdty::i64x2, b: *mut i8, c: i32) -> ();
        #[link_name = "llvm.mips.st.h"]
        pub fn st_h(a: ::simdty::i16x8, b: *mut i8, c: i32) -> ();
        #[link_name = "llvm.mips.st.w"]
        pub fn st_w(a: ::simdty::i32x4, b: *mut i8, c: i32) -> ();
        #[link_name = "llvm.mips.subs.s.b"]
        pub fn subs_s_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.subs.s.d"]
        pub fn subs_s_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.subs.s.h"]
        pub fn subs_s_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.subs.s.w"]
        pub fn subs_s_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.subs.u.b"]
        pub fn subs_u_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.subs.u.d"]
        pub fn subs_u_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.subs.u.h"]
        pub fn subs_u_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.subs.u.w"]
        pub fn subs_u_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.subsus.u.b"]
        pub fn subsus_u_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.subsus.u.d"]
        pub fn subsus_u_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.subsus.u.h"]
        pub fn subsus_u_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.subsus.u.w"]
        pub fn subsus_u_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.subsuu.s.b"]
        pub fn subsuu_s_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.subsuu.s.d"]
        pub fn subsuu_s_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.subsuu.s.h"]
        pub fn subsuu_s_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.subsuu.s.w"]
        pub fn subsuu_s_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.subu.ph"]
        pub fn subu_ph(a: ::simdty::i16x2, b: ::simdty::i16x2) -> ::simdty::i16x2;
        #[link_name = "llvm.mips.subu.qb"]
        pub fn subu_qb(a: ::simdty::i8x4, b: ::simdty::i8x4) -> ::simdty::i8x4;
        #[link_name = "llvm.mips.subu.s.ph"]
        pub fn subu_s_ph(a: ::simdty::i16x2, b: ::simdty::i16x2) -> ::simdty::i16x2;
        #[link_name = "llvm.mips.subu.s.qb"]
        pub fn subu_s_qb(a: ::simdty::i8x4, b: ::simdty::i8x4) -> ::simdty::i8x4;
        #[link_name = "llvm.mips.subuh.qb"]
        pub fn subuh_qb(a: ::simdty::i8x4, b: ::simdty::i8x4) -> ::simdty::i8x4;
        #[link_name = "llvm.mips.subuh.r.qb"]
        pub fn subuh_r_qb(a: ::simdty::i8x4, b: ::simdty::i8x4) -> ::simdty::i8x4;
        #[link_name = "llvm.mips.subv.b"]
        pub fn subv_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.subv.d"]
        pub fn subv_d(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.subv.h"]
        pub fn subv_h(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.subv.w"]
        pub fn subv_w(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.subvi.b"]
        pub fn subvi_b(a: ::simdty::i8x16, b: i32) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.subvi.d"]
        pub fn subvi_d(a: ::simdty::i64x2, b: i32) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.subvi.h"]
        pub fn subvi_h(a: ::simdty::i16x8, b: i32) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.subvi.w"]
        pub fn subvi_w(a: ::simdty::i32x4, b: i32) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.vshf.b"]
        pub fn vshf_b(a: ::simdty::i8x16, b: ::simdty::i8x16, c: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.vshf.d"]
        pub fn vshf_d(a: ::simdty::i64x2, b: ::simdty::i64x2, c: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.mips.vshf.h"]
        pub fn vshf_h(a: ::simdty::i16x8, b: ::simdty::i16x8, c: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.mips.vshf.w"]
        pub fn vshf_w(a: ::simdty::i32x4, b: ::simdty::i32x4, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.mips.wrdsp"]
        pub fn wrdsp(a: i32, b: i32) -> ();
        #[link_name = "llvm.mips.xor.v"]
        pub fn xor_v(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.mips.xori.b"]
        pub fn xori_b(a: ::simdty::i8x16, b: i32) -> ::simdty::i8x16;
    }
}
/// LLVM intrinsics for the nvvm architecture.
pub mod nvvm {
    extern {
        #[link_name = "llvm.nvvm.abs.i"]
        pub fn abs_i(a: i32) -> i32;
        #[link_name = "llvm.nvvm.abs.ll"]
        pub fn abs_ll(a: i64) -> i64;
        #[link_name = "llvm.nvvm.add.rm.d"]
        pub fn add_rm_d(a: f64, b: f64) -> f64;
        #[link_name = "llvm.nvvm.add.rm.f"]
        pub fn add_rm_f(a: f32, b: f32) -> f32;
        #[link_name = "llvm.nvvm.add.rm.ftz.f"]
        pub fn add_rm_ftz_f(a: f32, b: f32) -> f32;
        #[link_name = "llvm.nvvm.add.rn.d"]
        pub fn add_rn_d(a: f64, b: f64) -> f64;
        #[link_name = "llvm.nvvm.add.rn.f"]
        pub fn add_rn_f(a: f32, b: f32) -> f32;
        #[link_name = "llvm.nvvm.add.rn.ftz.f"]
        pub fn add_rn_ftz_f(a: f32, b: f32) -> f32;
        #[link_name = "llvm.nvvm.add.rp.d"]
        pub fn add_rp_d(a: f64, b: f64) -> f64;
        #[link_name = "llvm.nvvm.add.rp.f"]
        pub fn add_rp_f(a: f32, b: f32) -> f32;
        #[link_name = "llvm.nvvm.add.rp.ftz.f"]
        pub fn add_rp_ftz_f(a: f32, b: f32) -> f32;
        #[link_name = "llvm.nvvm.add.rz.d"]
        pub fn add_rz_d(a: f64, b: f64) -> f64;
        #[link_name = "llvm.nvvm.add.rz.f"]
        pub fn add_rz_f(a: f32, b: f32) -> f32;
        #[link_name = "llvm.nvvm.add.rz.ftz.f"]
        pub fn add_rz_ftz_f(a: f32, b: f32) -> f32;
        #[link_name = "llvm.nvvm.barrier0"]
        pub fn barrier0() -> ();
        #[link_name = "llvm.nvvm.barrier0.and"]
        pub fn barrier0_and(a: i32) -> i32;
        #[link_name = "llvm.nvvm.barrier0.or"]
        pub fn barrier0_or(a: i32) -> i32;
        #[link_name = "llvm.nvvm.barrier0.popc"]
        pub fn barrier0_popc(a: i32) -> i32;
        #[link_name = "llvm.nvvm.bitcast.d2ll"]
        pub fn bitcast_d2ll(a: f64) -> i64;
        #[link_name = "llvm.nvvm.bitcast.f2i"]
        pub fn bitcast_f2i(a: f32) -> i32;
        #[link_name = "llvm.nvvm.bitcast.i2f"]
        pub fn bitcast_i2f(a: i32) -> f32;
        #[link_name = "llvm.nvvm.bitcast.ll2d"]
        pub fn bitcast_ll2d(a: i64) -> f64;
        #[link_name = "llvm.nvvm.brev32"]
        pub fn brev32(a: i32) -> i32;
        #[link_name = "llvm.nvvm.brev64"]
        pub fn brev64(a: i64) -> i64;
        #[link_name = "llvm.nvvm.ceil.d"]
        pub fn ceil_d(a: f64) -> f64;
        #[link_name = "llvm.nvvm.ceil.f"]
        pub fn ceil_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.ceil.ftz.f"]
        pub fn ceil_ftz_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.clz.i"]
        pub fn clz_i(a: i32) -> i32;
        #[link_name = "llvm.nvvm.clz.ll"]
        pub fn clz_ll(a: i64) -> i32;
        #[link_name = "llvm.nvvm.compiler.error"]
        pub fn compiler_error(a: *mut i8) -> ();
        #[link_name = "llvm.nvvm.compiler.warn"]
        pub fn compiler_warn(a: *mut i8) -> ();
        #[link_name = "llvm.nvvm.cos.approx.f"]
        pub fn cos_approx_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.cos.approx.ftz.f"]
        pub fn cos_approx_ftz_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.d2f.rm"]
        pub fn d2f_rm(a: f64) -> f32;
        #[link_name = "llvm.nvvm.d2f.rm.ftz"]
        pub fn d2f_rm_ftz(a: f64) -> f32;
        #[link_name = "llvm.nvvm.d2f.rn"]
        pub fn d2f_rn(a: f64) -> f32;
        #[link_name = "llvm.nvvm.d2f.rn.ftz"]
        pub fn d2f_rn_ftz(a: f64) -> f32;
        #[link_name = "llvm.nvvm.d2f.rp"]
        pub fn d2f_rp(a: f64) -> f32;
        #[link_name = "llvm.nvvm.d2f.rp.ftz"]
        pub fn d2f_rp_ftz(a: f64) -> f32;
        #[link_name = "llvm.nvvm.d2f.rz"]
        pub fn d2f_rz(a: f64) -> f32;
        #[link_name = "llvm.nvvm.d2f.rz.ftz"]
        pub fn d2f_rz_ftz(a: f64) -> f32;
        #[link_name = "llvm.nvvm.d2i.hi"]
        pub fn d2i_hi(a: f64) -> i32;
        #[link_name = "llvm.nvvm.d2i.lo"]
        pub fn d2i_lo(a: f64) -> i32;
        #[link_name = "llvm.nvvm.d2i.rm"]
        pub fn d2i_rm(a: f64) -> i32;
        #[link_name = "llvm.nvvm.d2i.rn"]
        pub fn d2i_rn(a: f64) -> i32;
        #[link_name = "llvm.nvvm.d2i.rp"]
        pub fn d2i_rp(a: f64) -> i32;
        #[link_name = "llvm.nvvm.d2i.rz"]
        pub fn d2i_rz(a: f64) -> i32;
        #[link_name = "llvm.nvvm.d2ll.rm"]
        pub fn d2ll_rm(a: f64) -> i64;
        #[link_name = "llvm.nvvm.d2ll.rn"]
        pub fn d2ll_rn(a: f64) -> i64;
        #[link_name = "llvm.nvvm.d2ll.rp"]
        pub fn d2ll_rp(a: f64) -> i64;
        #[link_name = "llvm.nvvm.d2ll.rz"]
        pub fn d2ll_rz(a: f64) -> i64;
        #[link_name = "llvm.nvvm.d2ui.rm"]
        pub fn d2ui_rm(a: f64) -> i32;
        #[link_name = "llvm.nvvm.d2ui.rn"]
        pub fn d2ui_rn(a: f64) -> i32;
        #[link_name = "llvm.nvvm.d2ui.rp"]
        pub fn d2ui_rp(a: f64) -> i32;
        #[link_name = "llvm.nvvm.d2ui.rz"]
        pub fn d2ui_rz(a: f64) -> i32;
        #[link_name = "llvm.nvvm.d2ull.rm"]
        pub fn d2ull_rm(a: f64) -> i64;
        #[link_name = "llvm.nvvm.d2ull.rn"]
        pub fn d2ull_rn(a: f64) -> i64;
        #[link_name = "llvm.nvvm.d2ull.rp"]
        pub fn d2ull_rp(a: f64) -> i64;
        #[link_name = "llvm.nvvm.d2ull.rz"]
        pub fn d2ull_rz(a: f64) -> i64;
        #[link_name = "llvm.nvvm.div.approx.f"]
        pub fn div_approx_f(a: f32, b: f32) -> f32;
        #[link_name = "llvm.nvvm.div.approx.ftz.f"]
        pub fn div_approx_ftz_f(a: f32, b: f32) -> f32;
        #[link_name = "llvm.nvvm.div.rm.d"]
        pub fn div_rm_d(a: f64, b: f64) -> f64;
        #[link_name = "llvm.nvvm.div.rm.f"]
        pub fn div_rm_f(a: f32, b: f32) -> f32;
        #[link_name = "llvm.nvvm.div.rm.ftz.f"]
        pub fn div_rm_ftz_f(a: f32, b: f32) -> f32;
        #[link_name = "llvm.nvvm.div.rn.d"]
        pub fn div_rn_d(a: f64, b: f64) -> f64;
        #[link_name = "llvm.nvvm.div.rn.f"]
        pub fn div_rn_f(a: f32, b: f32) -> f32;
        #[link_name = "llvm.nvvm.div.rn.ftz.f"]
        pub fn div_rn_ftz_f(a: f32, b: f32) -> f32;
        #[link_name = "llvm.nvvm.div.rp.d"]
        pub fn div_rp_d(a: f64, b: f64) -> f64;
        #[link_name = "llvm.nvvm.div.rp.f"]
        pub fn div_rp_f(a: f32, b: f32) -> f32;
        #[link_name = "llvm.nvvm.div.rp.ftz.f"]
        pub fn div_rp_ftz_f(a: f32, b: f32) -> f32;
        #[link_name = "llvm.nvvm.div.rz.d"]
        pub fn div_rz_d(a: f64, b: f64) -> f64;
        #[link_name = "llvm.nvvm.div.rz.f"]
        pub fn div_rz_f(a: f32, b: f32) -> f32;
        #[link_name = "llvm.nvvm.div.rz.ftz.f"]
        pub fn div_rz_ftz_f(a: f32, b: f32) -> f32;
        #[link_name = "llvm.nvvm.ex2.approx.d"]
        pub fn ex2_approx_d(a: f64) -> f64;
        #[link_name = "llvm.nvvm.ex2.approx.f"]
        pub fn ex2_approx_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.ex2.approx.ftz.f"]
        pub fn ex2_approx_ftz_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.f2h.rn"]
        pub fn f2h_rn(a: f32) -> i16;
        #[link_name = "llvm.nvvm.f2h.rn.ftz"]
        pub fn f2h_rn_ftz(a: f32) -> i16;
        #[link_name = "llvm.nvvm.f2i.rm"]
        pub fn f2i_rm(a: f32) -> i32;
        #[link_name = "llvm.nvvm.f2i.rm.ftz"]
        pub fn f2i_rm_ftz(a: f32) -> i32;
        #[link_name = "llvm.nvvm.f2i.rn"]
        pub fn f2i_rn(a: f32) -> i32;
        #[link_name = "llvm.nvvm.f2i.rn.ftz"]
        pub fn f2i_rn_ftz(a: f32) -> i32;
        #[link_name = "llvm.nvvm.f2i.rp"]
        pub fn f2i_rp(a: f32) -> i32;
        #[link_name = "llvm.nvvm.f2i.rp.ftz"]
        pub fn f2i_rp_ftz(a: f32) -> i32;
        #[link_name = "llvm.nvvm.f2i.rz"]
        pub fn f2i_rz(a: f32) -> i32;
        #[link_name = "llvm.nvvm.f2i.rz.ftz"]
        pub fn f2i_rz_ftz(a: f32) -> i32;
        #[link_name = "llvm.nvvm.f2ll.rm"]
        pub fn f2ll_rm(a: f32) -> i64;
        #[link_name = "llvm.nvvm.f2ll.rm.ftz"]
        pub fn f2ll_rm_ftz(a: f32) -> i64;
        #[link_name = "llvm.nvvm.f2ll.rn"]
        pub fn f2ll_rn(a: f32) -> i64;
        #[link_name = "llvm.nvvm.f2ll.rn.ftz"]
        pub fn f2ll_rn_ftz(a: f32) -> i64;
        #[link_name = "llvm.nvvm.f2ll.rp"]
        pub fn f2ll_rp(a: f32) -> i64;
        #[link_name = "llvm.nvvm.f2ll.rp.ftz"]
        pub fn f2ll_rp_ftz(a: f32) -> i64;
        #[link_name = "llvm.nvvm.f2ll.rz"]
        pub fn f2ll_rz(a: f32) -> i64;
        #[link_name = "llvm.nvvm.f2ll.rz.ftz"]
        pub fn f2ll_rz_ftz(a: f32) -> i64;
        #[link_name = "llvm.nvvm.f2ui.rm"]
        pub fn f2ui_rm(a: f32) -> i32;
        #[link_name = "llvm.nvvm.f2ui.rm.ftz"]
        pub fn f2ui_rm_ftz(a: f32) -> i32;
        #[link_name = "llvm.nvvm.f2ui.rn"]
        pub fn f2ui_rn(a: f32) -> i32;
        #[link_name = "llvm.nvvm.f2ui.rn.ftz"]
        pub fn f2ui_rn_ftz(a: f32) -> i32;
        #[link_name = "llvm.nvvm.f2ui.rp"]
        pub fn f2ui_rp(a: f32) -> i32;
        #[link_name = "llvm.nvvm.f2ui.rp.ftz"]
        pub fn f2ui_rp_ftz(a: f32) -> i32;
        #[link_name = "llvm.nvvm.f2ui.rz"]
        pub fn f2ui_rz(a: f32) -> i32;
        #[link_name = "llvm.nvvm.f2ui.rz.ftz"]
        pub fn f2ui_rz_ftz(a: f32) -> i32;
        #[link_name = "llvm.nvvm.f2ull.rm"]
        pub fn f2ull_rm(a: f32) -> i64;
        #[link_name = "llvm.nvvm.f2ull.rm.ftz"]
        pub fn f2ull_rm_ftz(a: f32) -> i64;
        #[link_name = "llvm.nvvm.f2ull.rn"]
        pub fn f2ull_rn(a: f32) -> i64;
        #[link_name = "llvm.nvvm.f2ull.rn.ftz"]
        pub fn f2ull_rn_ftz(a: f32) -> i64;
        #[link_name = "llvm.nvvm.f2ull.rp"]
        pub fn f2ull_rp(a: f32) -> i64;
        #[link_name = "llvm.nvvm.f2ull.rp.ftz"]
        pub fn f2ull_rp_ftz(a: f32) -> i64;
        #[link_name = "llvm.nvvm.f2ull.rz"]
        pub fn f2ull_rz(a: f32) -> i64;
        #[link_name = "llvm.nvvm.f2ull.rz.ftz"]
        pub fn f2ull_rz_ftz(a: f32) -> i64;
        #[link_name = "llvm.nvvm.fabs.d"]
        pub fn fabs_d(a: f64) -> f64;
        #[link_name = "llvm.nvvm.fabs.f"]
        pub fn fabs_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.fabs.ftz.f"]
        pub fn fabs_ftz_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.floor.d"]
        pub fn floor_d(a: f64) -> f64;
        #[link_name = "llvm.nvvm.floor.f"]
        pub fn floor_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.floor.ftz.f"]
        pub fn floor_ftz_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.fma.rm.d"]
        pub fn fma_rm_d(a: f64, b: f64, c: f64) -> f64;
        #[link_name = "llvm.nvvm.fma.rm.f"]
        pub fn fma_rm_f(a: f32, b: f32, c: f32) -> f32;
        #[link_name = "llvm.nvvm.fma.rm.ftz.f"]
        pub fn fma_rm_ftz_f(a: f32, b: f32, c: f32) -> f32;
        #[link_name = "llvm.nvvm.fma.rn.d"]
        pub fn fma_rn_d(a: f64, b: f64, c: f64) -> f64;
        #[link_name = "llvm.nvvm.fma.rn.f"]
        pub fn fma_rn_f(a: f32, b: f32, c: f32) -> f32;
        #[link_name = "llvm.nvvm.fma.rn.ftz.f"]
        pub fn fma_rn_ftz_f(a: f32, b: f32, c: f32) -> f32;
        #[link_name = "llvm.nvvm.fma.rp.d"]
        pub fn fma_rp_d(a: f64, b: f64, c: f64) -> f64;
        #[link_name = "llvm.nvvm.fma.rp.f"]
        pub fn fma_rp_f(a: f32, b: f32, c: f32) -> f32;
        #[link_name = "llvm.nvvm.fma.rp.ftz.f"]
        pub fn fma_rp_ftz_f(a: f32, b: f32, c: f32) -> f32;
        #[link_name = "llvm.nvvm.fma.rz.d"]
        pub fn fma_rz_d(a: f64, b: f64, c: f64) -> f64;
        #[link_name = "llvm.nvvm.fma.rz.f"]
        pub fn fma_rz_f(a: f32, b: f32, c: f32) -> f32;
        #[link_name = "llvm.nvvm.fma.rz.ftz.f"]
        pub fn fma_rz_ftz_f(a: f32, b: f32, c: f32) -> f32;
        #[link_name = "llvm.nvvm.fmax.d"]
        pub fn fmax_d(a: f64, b: f64) -> f64;
        #[link_name = "llvm.nvvm.fmax.f"]
        pub fn fmax_f(a: f32, b: f32) -> f32;
        #[link_name = "llvm.nvvm.fmax.ftz.f"]
        pub fn fmax_ftz_f(a: f32, b: f32) -> f32;
        #[link_name = "llvm.nvvm.fmin.d"]
        pub fn fmin_d(a: f64, b: f64) -> f64;
        #[link_name = "llvm.nvvm.fmin.f"]
        pub fn fmin_f(a: f32, b: f32) -> f32;
        #[link_name = "llvm.nvvm.fmin.ftz.f"]
        pub fn fmin_ftz_f(a: f32, b: f32) -> f32;
        #[link_name = "llvm.nvvm.h2f"]
        pub fn h2f(a: i16) -> f32;
        #[link_name = "llvm.nvvm.i2d.rm"]
        pub fn i2d_rm(a: i32) -> f64;
        #[link_name = "llvm.nvvm.i2d.rn"]
        pub fn i2d_rn(a: i32) -> f64;
        #[link_name = "llvm.nvvm.i2d.rp"]
        pub fn i2d_rp(a: i32) -> f64;
        #[link_name = "llvm.nvvm.i2d.rz"]
        pub fn i2d_rz(a: i32) -> f64;
        #[link_name = "llvm.nvvm.i2f.rm"]
        pub fn i2f_rm(a: i32) -> f32;
        #[link_name = "llvm.nvvm.i2f.rn"]
        pub fn i2f_rn(a: i32) -> f32;
        #[link_name = "llvm.nvvm.i2f.rp"]
        pub fn i2f_rp(a: i32) -> f32;
        #[link_name = "llvm.nvvm.i2f.rz"]
        pub fn i2f_rz(a: i32) -> f32;
        #[link_name = "llvm.nvvm.isspacep.const"]
        pub fn isspacep_const(a: *mut i8) -> bool;
        #[link_name = "llvm.nvvm.isspacep.global"]
        pub fn isspacep_global(a: *mut i8) -> bool;
        #[link_name = "llvm.nvvm.isspacep.local"]
        pub fn isspacep_local(a: *mut i8) -> bool;
        #[link_name = "llvm.nvvm.isspacep.shared"]
        pub fn isspacep_shared(a: *mut i8) -> bool;
        #[link_name = "llvm.nvvm.istypep.sampler"]
        pub fn istypep_sampler(a: i64) -> bool;
        #[link_name = "llvm.nvvm.istypep.surface"]
        pub fn istypep_surface(a: i64) -> bool;
        #[link_name = "llvm.nvvm.istypep.texture"]
        pub fn istypep_texture(a: i64) -> bool;
        #[link_name = "llvm.nvvm.lg2.approx.d"]
        pub fn lg2_approx_d(a: f64) -> f64;
        #[link_name = "llvm.nvvm.lg2.approx.f"]
        pub fn lg2_approx_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.lg2.approx.ftz.f"]
        pub fn lg2_approx_ftz_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.ll2d.rm"]
        pub fn ll2d_rm(a: i64) -> f64;
        #[link_name = "llvm.nvvm.ll2d.rn"]
        pub fn ll2d_rn(a: i64) -> f64;
        #[link_name = "llvm.nvvm.ll2d.rp"]
        pub fn ll2d_rp(a: i64) -> f64;
        #[link_name = "llvm.nvvm.ll2d.rz"]
        pub fn ll2d_rz(a: i64) -> f64;
        #[link_name = "llvm.nvvm.ll2f.rm"]
        pub fn ll2f_rm(a: i64) -> f32;
        #[link_name = "llvm.nvvm.ll2f.rn"]
        pub fn ll2f_rn(a: i64) -> f32;
        #[link_name = "llvm.nvvm.ll2f.rp"]
        pub fn ll2f_rp(a: i64) -> f32;
        #[link_name = "llvm.nvvm.ll2f.rz"]
        pub fn ll2f_rz(a: i64) -> f32;
        #[link_name = "llvm.nvvm.lohi.i2d"]
        pub fn lohi_i2d(a: i32, b: i32) -> f64;
        #[link_name = "llvm.nvvm.max.i"]
        pub fn max_i(a: i32, b: i32) -> i32;
        #[link_name = "llvm.nvvm.max.ll"]
        pub fn max_ll(a: i64, b: i64) -> i64;
        #[link_name = "llvm.nvvm.max.ui"]
        pub fn max_ui(a: i32, b: i32) -> i32;
        #[link_name = "llvm.nvvm.max.ull"]
        pub fn max_ull(a: i64, b: i64) -> i64;
        #[link_name = "llvm.nvvm.membar.cta"]
        pub fn membar_cta() -> ();
        #[link_name = "llvm.nvvm.membar.gl"]
        pub fn membar_gl() -> ();
        #[link_name = "llvm.nvvm.membar.sys"]
        pub fn membar_sys() -> ();
        #[link_name = "llvm.nvvm.min.i"]
        pub fn min_i(a: i32, b: i32) -> i32;
        #[link_name = "llvm.nvvm.min.ll"]
        pub fn min_ll(a: i64, b: i64) -> i64;
        #[link_name = "llvm.nvvm.min.ui"]
        pub fn min_ui(a: i32, b: i32) -> i32;
        #[link_name = "llvm.nvvm.min.ull"]
        pub fn min_ull(a: i64, b: i64) -> i64;
        #[link_name = "llvm.nvvm.move.double"]
        pub fn move_double(a: f64) -> f64;
        #[link_name = "llvm.nvvm.move.float"]
        pub fn move_float(a: f32) -> f32;
        #[link_name = "llvm.nvvm.move.i16"]
        pub fn move_i16(a: i16) -> i16;
        #[link_name = "llvm.nvvm.move.i32"]
        pub fn move_i32(a: i32) -> i32;
        #[link_name = "llvm.nvvm.move.i64"]
        pub fn move_i64(a: i64) -> i64;
        #[link_name = "llvm.nvvm.move.ptr"]
        pub fn move_ptr(a: *mut i8) -> *mut i8;
        #[link_name = "llvm.nvvm.mul24.i"]
        pub fn mul24_i(a: i32, b: i32) -> i32;
        #[link_name = "llvm.nvvm.mul24.ui"]
        pub fn mul24_ui(a: i32, b: i32) -> i32;
        #[link_name = "llvm.nvvm.mul.rm.d"]
        pub fn mul_rm_d(a: f64, b: f64) -> f64;
        #[link_name = "llvm.nvvm.mul.rm.f"]
        pub fn mul_rm_f(a: f32, b: f32) -> f32;
        #[link_name = "llvm.nvvm.mul.rm.ftz.f"]
        pub fn mul_rm_ftz_f(a: f32, b: f32) -> f32;
        #[link_name = "llvm.nvvm.mul.rn.d"]
        pub fn mul_rn_d(a: f64, b: f64) -> f64;
        #[link_name = "llvm.nvvm.mul.rn.f"]
        pub fn mul_rn_f(a: f32, b: f32) -> f32;
        #[link_name = "llvm.nvvm.mul.rn.ftz.f"]
        pub fn mul_rn_ftz_f(a: f32, b: f32) -> f32;
        #[link_name = "llvm.nvvm.mul.rp.d"]
        pub fn mul_rp_d(a: f64, b: f64) -> f64;
        #[link_name = "llvm.nvvm.mul.rp.f"]
        pub fn mul_rp_f(a: f32, b: f32) -> f32;
        #[link_name = "llvm.nvvm.mul.rp.ftz.f"]
        pub fn mul_rp_ftz_f(a: f32, b: f32) -> f32;
        #[link_name = "llvm.nvvm.mul.rz.d"]
        pub fn mul_rz_d(a: f64, b: f64) -> f64;
        #[link_name = "llvm.nvvm.mul.rz.f"]
        pub fn mul_rz_f(a: f32, b: f32) -> f32;
        #[link_name = "llvm.nvvm.mul.rz.ftz.f"]
        pub fn mul_rz_ftz_f(a: f32, b: f32) -> f32;
        #[link_name = "llvm.nvvm.mulhi.i"]
        pub fn mulhi_i(a: i32, b: i32) -> i32;
        #[link_name = "llvm.nvvm.mulhi.ll"]
        pub fn mulhi_ll(a: i64, b: i64) -> i64;
        #[link_name = "llvm.nvvm.mulhi.ui"]
        pub fn mulhi_ui(a: i32, b: i32) -> i32;
        #[link_name = "llvm.nvvm.mulhi.ull"]
        pub fn mulhi_ull(a: i64, b: i64) -> i64;
        #[link_name = "llvm.nvvm.popc.i"]
        pub fn popc_i(a: i32) -> i32;
        #[link_name = "llvm.nvvm.popc.ll"]
        pub fn popc_ll(a: i64) -> i32;
        #[link_name = "llvm.nvvm.prmt"]
        pub fn prmt(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.nvvm.ptr.constant.to.gen"]
        pub fn ptr_constant_to_gen(a: *mut i8) -> *mut i8;
        #[link_name = "llvm.nvvm.ptr.gen.to.constant"]
        pub fn ptr_gen_to_constant(a: *mut i8) -> *mut i8;
        #[link_name = "llvm.nvvm.ptr.gen.to.global"]
        pub fn ptr_gen_to_global(a: *mut i8) -> *mut i8;
        #[link_name = "llvm.nvvm.ptr.gen.to.local"]
        pub fn ptr_gen_to_local(a: *mut i8) -> *mut i8;
        #[link_name = "llvm.nvvm.ptr.gen.to.param"]
        pub fn ptr_gen_to_param(a: *mut i8) -> *mut i8;
        #[link_name = "llvm.nvvm.ptr.gen.to.shared"]
        pub fn ptr_gen_to_shared(a: *mut i8) -> *mut i8;
        #[link_name = "llvm.nvvm.ptr.global.to.gen"]
        pub fn ptr_global_to_gen(a: *mut i8) -> *mut i8;
        #[link_name = "llvm.nvvm.ptr.local.to.gen"]
        pub fn ptr_local_to_gen(a: *mut i8) -> *mut i8;
        #[link_name = "llvm.nvvm.ptr.shared.to.gen"]
        pub fn ptr_shared_to_gen(a: *mut i8) -> *mut i8;
        #[link_name = "llvm.nvvm.rcp.approx.ftz.d"]
        pub fn rcp_approx_ftz_d(a: f64) -> f64;
        #[link_name = "llvm.nvvm.rcp.rm.d"]
        pub fn rcp_rm_d(a: f64) -> f64;
        #[link_name = "llvm.nvvm.rcp.rm.f"]
        pub fn rcp_rm_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.rcp.rm.ftz.f"]
        pub fn rcp_rm_ftz_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.rcp.rn.d"]
        pub fn rcp_rn_d(a: f64) -> f64;
        #[link_name = "llvm.nvvm.rcp.rn.f"]
        pub fn rcp_rn_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.rcp.rn.ftz.f"]
        pub fn rcp_rn_ftz_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.rcp.rp.d"]
        pub fn rcp_rp_d(a: f64) -> f64;
        #[link_name = "llvm.nvvm.rcp.rp.f"]
        pub fn rcp_rp_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.rcp.rp.ftz.f"]
        pub fn rcp_rp_ftz_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.rcp.rz.d"]
        pub fn rcp_rz_d(a: f64) -> f64;
        #[link_name = "llvm.nvvm.rcp.rz.f"]
        pub fn rcp_rz_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.rcp.rz.ftz.f"]
        pub fn rcp_rz_ftz_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.ctaid.x"]
        pub fn read_ptx_sreg_ctaid_x() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.ctaid.y"]
        pub fn read_ptx_sreg_ctaid_y() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.ctaid.z"]
        pub fn read_ptx_sreg_ctaid_z() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.envreg0"]
        pub fn read_ptx_sreg_envreg0() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.envreg1"]
        pub fn read_ptx_sreg_envreg1() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.envreg10"]
        pub fn read_ptx_sreg_envreg10() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.envreg11"]
        pub fn read_ptx_sreg_envreg11() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.envreg12"]
        pub fn read_ptx_sreg_envreg12() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.envreg13"]
        pub fn read_ptx_sreg_envreg13() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.envreg14"]
        pub fn read_ptx_sreg_envreg14() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.envreg15"]
        pub fn read_ptx_sreg_envreg15() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.envreg16"]
        pub fn read_ptx_sreg_envreg16() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.envreg17"]
        pub fn read_ptx_sreg_envreg17() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.envreg18"]
        pub fn read_ptx_sreg_envreg18() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.envreg19"]
        pub fn read_ptx_sreg_envreg19() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.envreg2"]
        pub fn read_ptx_sreg_envreg2() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.envreg20"]
        pub fn read_ptx_sreg_envreg20() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.envreg21"]
        pub fn read_ptx_sreg_envreg21() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.envreg22"]
        pub fn read_ptx_sreg_envreg22() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.envreg23"]
        pub fn read_ptx_sreg_envreg23() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.envreg24"]
        pub fn read_ptx_sreg_envreg24() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.envreg25"]
        pub fn read_ptx_sreg_envreg25() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.envreg26"]
        pub fn read_ptx_sreg_envreg26() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.envreg27"]
        pub fn read_ptx_sreg_envreg27() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.envreg28"]
        pub fn read_ptx_sreg_envreg28() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.envreg29"]
        pub fn read_ptx_sreg_envreg29() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.envreg3"]
        pub fn read_ptx_sreg_envreg3() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.envreg30"]
        pub fn read_ptx_sreg_envreg30() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.envreg31"]
        pub fn read_ptx_sreg_envreg31() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.envreg4"]
        pub fn read_ptx_sreg_envreg4() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.envreg5"]
        pub fn read_ptx_sreg_envreg5() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.envreg6"]
        pub fn read_ptx_sreg_envreg6() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.envreg7"]
        pub fn read_ptx_sreg_envreg7() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.envreg8"]
        pub fn read_ptx_sreg_envreg8() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.envreg9"]
        pub fn read_ptx_sreg_envreg9() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.nctaid.x"]
        pub fn read_ptx_sreg_nctaid_x() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.nctaid.y"]
        pub fn read_ptx_sreg_nctaid_y() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.nctaid.z"]
        pub fn read_ptx_sreg_nctaid_z() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.ntid.x"]
        pub fn read_ptx_sreg_ntid_x() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.ntid.y"]
        pub fn read_ptx_sreg_ntid_y() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.ntid.z"]
        pub fn read_ptx_sreg_ntid_z() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.tid.x"]
        pub fn read_ptx_sreg_tid_x() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.tid.y"]
        pub fn read_ptx_sreg_tid_y() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.tid.z"]
        pub fn read_ptx_sreg_tid_z() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.warpsize"]
        pub fn read_ptx_sreg_warpsize() -> i32;
        #[link_name = "llvm.nvvm.reflect"]
        pub fn reflect(a: *mut i8) -> i32;
        #[link_name = "llvm.nvvm.rotate.b32"]
        pub fn rotate_b32(a: i32, b: i32) -> i32;
        #[link_name = "llvm.nvvm.rotate.b64"]
        pub fn rotate_b64(a: i64, b: i32) -> i64;
        #[link_name = "llvm.nvvm.rotate.right.b64"]
        pub fn rotate_right_b64(a: i64, b: i32) -> i64;
        #[link_name = "llvm.nvvm.round.d"]
        pub fn round_d(a: f64) -> f64;
        #[link_name = "llvm.nvvm.round.f"]
        pub fn round_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.round.ftz.f"]
        pub fn round_ftz_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.rsqrt.approx.d"]
        pub fn rsqrt_approx_d(a: f64) -> f64;
        #[link_name = "llvm.nvvm.rsqrt.approx.f"]
        pub fn rsqrt_approx_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.rsqrt.approx.ftz.f"]
        pub fn rsqrt_approx_ftz_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.sad.i"]
        pub fn sad_i(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.nvvm.sad.ui"]
        pub fn sad_ui(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.nvvm.saturate.d"]
        pub fn saturate_d(a: f64) -> f64;
        #[link_name = "llvm.nvvm.saturate.f"]
        pub fn saturate_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.saturate.ftz.f"]
        pub fn saturate_ftz_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.sin.approx.f"]
        pub fn sin_approx_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.sin.approx.ftz.f"]
        pub fn sin_approx_ftz_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.sqrt.approx.f"]
        pub fn sqrt_approx_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.sqrt.approx.ftz.f"]
        pub fn sqrt_approx_ftz_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.sqrt.f"]
        pub fn sqrt_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.sqrt.rm.d"]
        pub fn sqrt_rm_d(a: f64) -> f64;
        #[link_name = "llvm.nvvm.sqrt.rm.f"]
        pub fn sqrt_rm_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.sqrt.rm.ftz.f"]
        pub fn sqrt_rm_ftz_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.sqrt.rn.d"]
        pub fn sqrt_rn_d(a: f64) -> f64;
        #[link_name = "llvm.nvvm.sqrt.rn.f"]
        pub fn sqrt_rn_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.sqrt.rn.ftz.f"]
        pub fn sqrt_rn_ftz_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.sqrt.rp.d"]
        pub fn sqrt_rp_d(a: f64) -> f64;
        #[link_name = "llvm.nvvm.sqrt.rp.f"]
        pub fn sqrt_rp_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.sqrt.rp.ftz.f"]
        pub fn sqrt_rp_ftz_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.sqrt.rz.d"]
        pub fn sqrt_rz_d(a: f64) -> f64;
        #[link_name = "llvm.nvvm.sqrt.rz.f"]
        pub fn sqrt_rz_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.sqrt.rz.ftz.f"]
        pub fn sqrt_rz_ftz_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.suld.1d.array.i16.clamp"]
        pub fn suld_1d_array_i16_clamp(a: i64, b: i32, c: i32) -> i16;
        #[link_name = "llvm.nvvm.suld.1d.array.i16.trap"]
        pub fn suld_1d_array_i16_trap(a: i64, b: i32, c: i32) -> i16;
        #[link_name = "llvm.nvvm.suld.1d.array.i16.zero"]
        pub fn suld_1d_array_i16_zero(a: i64, b: i32, c: i32) -> i16;
        #[link_name = "llvm.nvvm.suld.1d.array.i32.clamp"]
        pub fn suld_1d_array_i32_clamp(a: i64, b: i32, c: i32) -> i32;
        #[link_name = "llvm.nvvm.suld.1d.array.i32.trap"]
        pub fn suld_1d_array_i32_trap(a: i64, b: i32, c: i32) -> i32;
        #[link_name = "llvm.nvvm.suld.1d.array.i32.zero"]
        pub fn suld_1d_array_i32_zero(a: i64, b: i32, c: i32) -> i32;
        #[link_name = "llvm.nvvm.suld.1d.array.i64.clamp"]
        pub fn suld_1d_array_i64_clamp(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.nvvm.suld.1d.array.i64.trap"]
        pub fn suld_1d_array_i64_trap(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.nvvm.suld.1d.array.i64.zero"]
        pub fn suld_1d_array_i64_zero(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.nvvm.suld.1d.array.i8.clamp"]
        pub fn suld_1d_array_i8_clamp(a: i64, b: i32, c: i32) -> i16;
        #[link_name = "llvm.nvvm.suld.1d.array.i8.trap"]
        pub fn suld_1d_array_i8_trap(a: i64, b: i32, c: i32) -> i16;
        #[link_name = "llvm.nvvm.suld.1d.array.i8.zero"]
        pub fn suld_1d_array_i8_zero(a: i64, b: i32, c: i32) -> i16;
        #[link_name = "llvm.nvvm.suld.1d.i16.clamp"]
        pub fn suld_1d_i16_clamp(a: i64, b: i32) -> i16;
        #[link_name = "llvm.nvvm.suld.1d.i16.trap"]
        pub fn suld_1d_i16_trap(a: i64, b: i32) -> i16;
        #[link_name = "llvm.nvvm.suld.1d.i16.zero"]
        pub fn suld_1d_i16_zero(a: i64, b: i32) -> i16;
        #[link_name = "llvm.nvvm.suld.1d.i32.clamp"]
        pub fn suld_1d_i32_clamp(a: i64, b: i32) -> i32;
        #[link_name = "llvm.nvvm.suld.1d.i32.trap"]
        pub fn suld_1d_i32_trap(a: i64, b: i32) -> i32;
        #[link_name = "llvm.nvvm.suld.1d.i32.zero"]
        pub fn suld_1d_i32_zero(a: i64, b: i32) -> i32;
        #[link_name = "llvm.nvvm.suld.1d.i64.clamp"]
        pub fn suld_1d_i64_clamp(a: i64, b: i32) -> i64;
        #[link_name = "llvm.nvvm.suld.1d.i64.trap"]
        pub fn suld_1d_i64_trap(a: i64, b: i32) -> i64;
        #[link_name = "llvm.nvvm.suld.1d.i64.zero"]
        pub fn suld_1d_i64_zero(a: i64, b: i32) -> i64;
        #[link_name = "llvm.nvvm.suld.1d.i8.clamp"]
        pub fn suld_1d_i8_clamp(a: i64, b: i32) -> i16;
        #[link_name = "llvm.nvvm.suld.1d.i8.trap"]
        pub fn suld_1d_i8_trap(a: i64, b: i32) -> i16;
        #[link_name = "llvm.nvvm.suld.1d.i8.zero"]
        pub fn suld_1d_i8_zero(a: i64, b: i32) -> i16;
        #[link_name = "llvm.nvvm.suld.2d.array.i16.clamp"]
        pub fn suld_2d_array_i16_clamp(a: i64, b: i32, c: i32, d: i32) -> i16;
        #[link_name = "llvm.nvvm.suld.2d.array.i16.trap"]
        pub fn suld_2d_array_i16_trap(a: i64, b: i32, c: i32, d: i32) -> i16;
        #[link_name = "llvm.nvvm.suld.2d.array.i16.zero"]
        pub fn suld_2d_array_i16_zero(a: i64, b: i32, c: i32, d: i32) -> i16;
        #[link_name = "llvm.nvvm.suld.2d.array.i32.clamp"]
        pub fn suld_2d_array_i32_clamp(a: i64, b: i32, c: i32, d: i32) -> i32;
        #[link_name = "llvm.nvvm.suld.2d.array.i32.trap"]
        pub fn suld_2d_array_i32_trap(a: i64, b: i32, c: i32, d: i32) -> i32;
        #[link_name = "llvm.nvvm.suld.2d.array.i32.zero"]
        pub fn suld_2d_array_i32_zero(a: i64, b: i32, c: i32, d: i32) -> i32;
        #[link_name = "llvm.nvvm.suld.2d.array.i64.clamp"]
        pub fn suld_2d_array_i64_clamp(a: i64, b: i32, c: i32, d: i32) -> i64;
        #[link_name = "llvm.nvvm.suld.2d.array.i64.trap"]
        pub fn suld_2d_array_i64_trap(a: i64, b: i32, c: i32, d: i32) -> i64;
        #[link_name = "llvm.nvvm.suld.2d.array.i64.zero"]
        pub fn suld_2d_array_i64_zero(a: i64, b: i32, c: i32, d: i32) -> i64;
        #[link_name = "llvm.nvvm.suld.2d.array.i8.clamp"]
        pub fn suld_2d_array_i8_clamp(a: i64, b: i32, c: i32, d: i32) -> i16;
        #[link_name = "llvm.nvvm.suld.2d.array.i8.trap"]
        pub fn suld_2d_array_i8_trap(a: i64, b: i32, c: i32, d: i32) -> i16;
        #[link_name = "llvm.nvvm.suld.2d.array.i8.zero"]
        pub fn suld_2d_array_i8_zero(a: i64, b: i32, c: i32, d: i32) -> i16;
        #[link_name = "llvm.nvvm.suld.2d.i16.clamp"]
        pub fn suld_2d_i16_clamp(a: i64, b: i32, c: i32) -> i16;
        #[link_name = "llvm.nvvm.suld.2d.i16.trap"]
        pub fn suld_2d_i16_trap(a: i64, b: i32, c: i32) -> i16;
        #[link_name = "llvm.nvvm.suld.2d.i16.zero"]
        pub fn suld_2d_i16_zero(a: i64, b: i32, c: i32) -> i16;
        #[link_name = "llvm.nvvm.suld.2d.i32.clamp"]
        pub fn suld_2d_i32_clamp(a: i64, b: i32, c: i32) -> i32;
        #[link_name = "llvm.nvvm.suld.2d.i32.trap"]
        pub fn suld_2d_i32_trap(a: i64, b: i32, c: i32) -> i32;
        #[link_name = "llvm.nvvm.suld.2d.i32.zero"]
        pub fn suld_2d_i32_zero(a: i64, b: i32, c: i32) -> i32;
        #[link_name = "llvm.nvvm.suld.2d.i64.clamp"]
        pub fn suld_2d_i64_clamp(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.nvvm.suld.2d.i64.trap"]
        pub fn suld_2d_i64_trap(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.nvvm.suld.2d.i64.zero"]
        pub fn suld_2d_i64_zero(a: i64, b: i32, c: i32) -> i64;
        #[link_name = "llvm.nvvm.suld.2d.i8.clamp"]
        pub fn suld_2d_i8_clamp(a: i64, b: i32, c: i32) -> i16;
        #[link_name = "llvm.nvvm.suld.2d.i8.trap"]
        pub fn suld_2d_i8_trap(a: i64, b: i32, c: i32) -> i16;
        #[link_name = "llvm.nvvm.suld.2d.i8.zero"]
        pub fn suld_2d_i8_zero(a: i64, b: i32, c: i32) -> i16;
        #[link_name = "llvm.nvvm.suld.3d.i16.clamp"]
        pub fn suld_3d_i16_clamp(a: i64, b: i32, c: i32, d: i32) -> i16;
        #[link_name = "llvm.nvvm.suld.3d.i16.trap"]
        pub fn suld_3d_i16_trap(a: i64, b: i32, c: i32, d: i32) -> i16;
        #[link_name = "llvm.nvvm.suld.3d.i16.zero"]
        pub fn suld_3d_i16_zero(a: i64, b: i32, c: i32, d: i32) -> i16;
        #[link_name = "llvm.nvvm.suld.3d.i32.clamp"]
        pub fn suld_3d_i32_clamp(a: i64, b: i32, c: i32, d: i32) -> i32;
        #[link_name = "llvm.nvvm.suld.3d.i32.trap"]
        pub fn suld_3d_i32_trap(a: i64, b: i32, c: i32, d: i32) -> i32;
        #[link_name = "llvm.nvvm.suld.3d.i32.zero"]
        pub fn suld_3d_i32_zero(a: i64, b: i32, c: i32, d: i32) -> i32;
        #[link_name = "llvm.nvvm.suld.3d.i64.clamp"]
        pub fn suld_3d_i64_clamp(a: i64, b: i32, c: i32, d: i32) -> i64;
        #[link_name = "llvm.nvvm.suld.3d.i64.trap"]
        pub fn suld_3d_i64_trap(a: i64, b: i32, c: i32, d: i32) -> i64;
        #[link_name = "llvm.nvvm.suld.3d.i64.zero"]
        pub fn suld_3d_i64_zero(a: i64, b: i32, c: i32, d: i32) -> i64;
        #[link_name = "llvm.nvvm.suld.3d.i8.clamp"]
        pub fn suld_3d_i8_clamp(a: i64, b: i32, c: i32, d: i32) -> i16;
        #[link_name = "llvm.nvvm.suld.3d.i8.trap"]
        pub fn suld_3d_i8_trap(a: i64, b: i32, c: i32, d: i32) -> i16;
        #[link_name = "llvm.nvvm.suld.3d.i8.zero"]
        pub fn suld_3d_i8_zero(a: i64, b: i32, c: i32, d: i32) -> i16;
        #[link_name = "llvm.nvvm.suq.array.size"]
        pub fn suq_array_size(a: i64) -> i32;
        #[link_name = "llvm.nvvm.suq.channel.data.type"]
        pub fn suq_channel_data_type(a: i64) -> i32;
        #[link_name = "llvm.nvvm.suq.channel.order"]
        pub fn suq_channel_order(a: i64) -> i32;
        #[link_name = "llvm.nvvm.suq.depth"]
        pub fn suq_depth(a: i64) -> i32;
        #[link_name = "llvm.nvvm.suq.height"]
        pub fn suq_height(a: i64) -> i32;
        #[link_name = "llvm.nvvm.suq.width"]
        pub fn suq_width(a: i64) -> i32;
        #[link_name = "llvm.nvvm.sust.b.1d.array.i16.clamp"]
        pub fn sust_b_1d_array_i16_clamp(a: i64, b: i32, c: i32, d: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.array.i16.trap"]
        pub fn sust_b_1d_array_i16_trap(a: i64, b: i32, c: i32, d: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.array.i16.zero"]
        pub fn sust_b_1d_array_i16_zero(a: i64, b: i32, c: i32, d: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.array.i32.clamp"]
        pub fn sust_b_1d_array_i32_clamp(a: i64, b: i32, c: i32, d: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.array.i32.trap"]
        pub fn sust_b_1d_array_i32_trap(a: i64, b: i32, c: i32, d: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.array.i32.zero"]
        pub fn sust_b_1d_array_i32_zero(a: i64, b: i32, c: i32, d: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.array.i64.clamp"]
        pub fn sust_b_1d_array_i64_clamp(a: i64, b: i32, c: i32, d: i64) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.array.i64.trap"]
        pub fn sust_b_1d_array_i64_trap(a: i64, b: i32, c: i32, d: i64) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.array.i64.zero"]
        pub fn sust_b_1d_array_i64_zero(a: i64, b: i32, c: i32, d: i64) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.array.i8.clamp"]
        pub fn sust_b_1d_array_i8_clamp(a: i64, b: i32, c: i32, d: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.array.i8.trap"]
        pub fn sust_b_1d_array_i8_trap(a: i64, b: i32, c: i32, d: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.array.i8.zero"]
        pub fn sust_b_1d_array_i8_zero(a: i64, b: i32, c: i32, d: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.array.v2i16.clamp"]
        pub fn sust_b_1d_array_v2i16_clamp(a: i64, b: i32, c: i32, d: i16, e: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.array.v2i16.trap"]
        pub fn sust_b_1d_array_v2i16_trap(a: i64, b: i32, c: i32, d: i16, e: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.array.v2i16.zero"]
        pub fn sust_b_1d_array_v2i16_zero(a: i64, b: i32, c: i32, d: i16, e: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.array.v2i32.clamp"]
        pub fn sust_b_1d_array_v2i32_clamp(a: i64, b: i32, c: i32, d: i32, e: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.array.v2i32.trap"]
        pub fn sust_b_1d_array_v2i32_trap(a: i64, b: i32, c: i32, d: i32, e: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.array.v2i32.zero"]
        pub fn sust_b_1d_array_v2i32_zero(a: i64, b: i32, c: i32, d: i32, e: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.array.v2i64.clamp"]
        pub fn sust_b_1d_array_v2i64_clamp(a: i64, b: i32, c: i32, d: i64, e: i64) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.array.v2i64.trap"]
        pub fn sust_b_1d_array_v2i64_trap(a: i64, b: i32, c: i32, d: i64, e: i64) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.array.v2i64.zero"]
        pub fn sust_b_1d_array_v2i64_zero(a: i64, b: i32, c: i32, d: i64, e: i64) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.array.v2i8.clamp"]
        pub fn sust_b_1d_array_v2i8_clamp(a: i64, b: i32, c: i32, d: i16, e: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.array.v2i8.trap"]
        pub fn sust_b_1d_array_v2i8_trap(a: i64, b: i32, c: i32, d: i16, e: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.array.v2i8.zero"]
        pub fn sust_b_1d_array_v2i8_zero(a: i64, b: i32, c: i32, d: i16, e: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.array.v4i16.clamp"]
        pub fn sust_b_1d_array_v4i16_clamp(a: i64, b: i32, c: i32, d: i16, e: i16, f: i16, g: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.array.v4i16.trap"]
        pub fn sust_b_1d_array_v4i16_trap(a: i64, b: i32, c: i32, d: i16, e: i16, f: i16, g: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.array.v4i16.zero"]
        pub fn sust_b_1d_array_v4i16_zero(a: i64, b: i32, c: i32, d: i16, e: i16, f: i16, g: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.array.v4i32.clamp"]
        pub fn sust_b_1d_array_v4i32_clamp(a: i64, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.array.v4i32.trap"]
        pub fn sust_b_1d_array_v4i32_trap(a: i64, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.array.v4i32.zero"]
        pub fn sust_b_1d_array_v4i32_zero(a: i64, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.array.v4i8.clamp"]
        pub fn sust_b_1d_array_v4i8_clamp(a: i64, b: i32, c: i32, d: i16, e: i16, f: i16, g: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.array.v4i8.trap"]
        pub fn sust_b_1d_array_v4i8_trap(a: i64, b: i32, c: i32, d: i16, e: i16, f: i16, g: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.array.v4i8.zero"]
        pub fn sust_b_1d_array_v4i8_zero(a: i64, b: i32, c: i32, d: i16, e: i16, f: i16, g: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.i16.clamp"]
        pub fn sust_b_1d_i16_clamp(a: i64, b: i32, c: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.i16.trap"]
        pub fn sust_b_1d_i16_trap(a: i64, b: i32, c: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.i16.zero"]
        pub fn sust_b_1d_i16_zero(a: i64, b: i32, c: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.i32.clamp"]
        pub fn sust_b_1d_i32_clamp(a: i64, b: i32, c: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.i32.trap"]
        pub fn sust_b_1d_i32_trap(a: i64, b: i32, c: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.i32.zero"]
        pub fn sust_b_1d_i32_zero(a: i64, b: i32, c: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.i64.clamp"]
        pub fn sust_b_1d_i64_clamp(a: i64, b: i32, c: i64) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.i64.trap"]
        pub fn sust_b_1d_i64_trap(a: i64, b: i32, c: i64) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.i64.zero"]
        pub fn sust_b_1d_i64_zero(a: i64, b: i32, c: i64) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.i8.clamp"]
        pub fn sust_b_1d_i8_clamp(a: i64, b: i32, c: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.i8.trap"]
        pub fn sust_b_1d_i8_trap(a: i64, b: i32, c: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.i8.zero"]
        pub fn sust_b_1d_i8_zero(a: i64, b: i32, c: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.v2i16.clamp"]
        pub fn sust_b_1d_v2i16_clamp(a: i64, b: i32, c: i16, d: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.v2i16.trap"]
        pub fn sust_b_1d_v2i16_trap(a: i64, b: i32, c: i16, d: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.v2i16.zero"]
        pub fn sust_b_1d_v2i16_zero(a: i64, b: i32, c: i16, d: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.v2i32.clamp"]
        pub fn sust_b_1d_v2i32_clamp(a: i64, b: i32, c: i32, d: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.v2i32.trap"]
        pub fn sust_b_1d_v2i32_trap(a: i64, b: i32, c: i32, d: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.v2i32.zero"]
        pub fn sust_b_1d_v2i32_zero(a: i64, b: i32, c: i32, d: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.v2i64.clamp"]
        pub fn sust_b_1d_v2i64_clamp(a: i64, b: i32, c: i64, d: i64) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.v2i64.trap"]
        pub fn sust_b_1d_v2i64_trap(a: i64, b: i32, c: i64, d: i64) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.v2i64.zero"]
        pub fn sust_b_1d_v2i64_zero(a: i64, b: i32, c: i64, d: i64) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.v2i8.clamp"]
        pub fn sust_b_1d_v2i8_clamp(a: i64, b: i32, c: i16, d: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.v2i8.trap"]
        pub fn sust_b_1d_v2i8_trap(a: i64, b: i32, c: i16, d: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.v2i8.zero"]
        pub fn sust_b_1d_v2i8_zero(a: i64, b: i32, c: i16, d: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.v4i16.clamp"]
        pub fn sust_b_1d_v4i16_clamp(a: i64, b: i32, c: i16, d: i16, e: i16, f: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.v4i16.trap"]
        pub fn sust_b_1d_v4i16_trap(a: i64, b: i32, c: i16, d: i16, e: i16, f: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.v4i16.zero"]
        pub fn sust_b_1d_v4i16_zero(a: i64, b: i32, c: i16, d: i16, e: i16, f: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.v4i32.clamp"]
        pub fn sust_b_1d_v4i32_clamp(a: i64, b: i32, c: i32, d: i32, e: i32, f: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.v4i32.trap"]
        pub fn sust_b_1d_v4i32_trap(a: i64, b: i32, c: i32, d: i32, e: i32, f: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.v4i32.zero"]
        pub fn sust_b_1d_v4i32_zero(a: i64, b: i32, c: i32, d: i32, e: i32, f: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.v4i8.clamp"]
        pub fn sust_b_1d_v4i8_clamp(a: i64, b: i32, c: i16, d: i16, e: i16, f: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.v4i8.trap"]
        pub fn sust_b_1d_v4i8_trap(a: i64, b: i32, c: i16, d: i16, e: i16, f: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.1d.v4i8.zero"]
        pub fn sust_b_1d_v4i8_zero(a: i64, b: i32, c: i16, d: i16, e: i16, f: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.array.i16.clamp"]
        pub fn sust_b_2d_array_i16_clamp(a: i64, b: i32, c: i32, d: i32, e: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.array.i16.trap"]
        pub fn sust_b_2d_array_i16_trap(a: i64, b: i32, c: i32, d: i32, e: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.array.i16.zero"]
        pub fn sust_b_2d_array_i16_zero(a: i64, b: i32, c: i32, d: i32, e: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.array.i32.clamp"]
        pub fn sust_b_2d_array_i32_clamp(a: i64, b: i32, c: i32, d: i32, e: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.array.i32.trap"]
        pub fn sust_b_2d_array_i32_trap(a: i64, b: i32, c: i32, d: i32, e: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.array.i32.zero"]
        pub fn sust_b_2d_array_i32_zero(a: i64, b: i32, c: i32, d: i32, e: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.array.i64.clamp"]
        pub fn sust_b_2d_array_i64_clamp(a: i64, b: i32, c: i32, d: i32, e: i64) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.array.i64.trap"]
        pub fn sust_b_2d_array_i64_trap(a: i64, b: i32, c: i32, d: i32, e: i64) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.array.i64.zero"]
        pub fn sust_b_2d_array_i64_zero(a: i64, b: i32, c: i32, d: i32, e: i64) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.array.i8.clamp"]
        pub fn sust_b_2d_array_i8_clamp(a: i64, b: i32, c: i32, d: i32, e: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.array.i8.trap"]
        pub fn sust_b_2d_array_i8_trap(a: i64, b: i32, c: i32, d: i32, e: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.array.i8.zero"]
        pub fn sust_b_2d_array_i8_zero(a: i64, b: i32, c: i32, d: i32, e: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.array.v2i16.clamp"]
        pub fn sust_b_2d_array_v2i16_clamp(a: i64, b: i32, c: i32, d: i32, e: i16, f: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.array.v2i16.trap"]
        pub fn sust_b_2d_array_v2i16_trap(a: i64, b: i32, c: i32, d: i32, e: i16, f: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.array.v2i16.zero"]
        pub fn sust_b_2d_array_v2i16_zero(a: i64, b: i32, c: i32, d: i32, e: i16, f: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.array.v2i32.clamp"]
        pub fn sust_b_2d_array_v2i32_clamp(a: i64, b: i32, c: i32, d: i32, e: i32, f: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.array.v2i32.trap"]
        pub fn sust_b_2d_array_v2i32_trap(a: i64, b: i32, c: i32, d: i32, e: i32, f: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.array.v2i32.zero"]
        pub fn sust_b_2d_array_v2i32_zero(a: i64, b: i32, c: i32, d: i32, e: i32, f: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.array.v2i64.clamp"]
        pub fn sust_b_2d_array_v2i64_clamp(a: i64, b: i32, c: i32, d: i32, e: i64, f: i64) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.array.v2i64.trap"]
        pub fn sust_b_2d_array_v2i64_trap(a: i64, b: i32, c: i32, d: i32, e: i64, f: i64) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.array.v2i64.zero"]
        pub fn sust_b_2d_array_v2i64_zero(a: i64, b: i32, c: i32, d: i32, e: i64, f: i64) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.array.v2i8.clamp"]
        pub fn sust_b_2d_array_v2i8_clamp(a: i64, b: i32, c: i32, d: i32, e: i16, f: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.array.v2i8.trap"]
        pub fn sust_b_2d_array_v2i8_trap(a: i64, b: i32, c: i32, d: i32, e: i16, f: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.array.v2i8.zero"]
        pub fn sust_b_2d_array_v2i8_zero(a: i64, b: i32, c: i32, d: i32, e: i16, f: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.array.v4i16.clamp"]
        pub fn sust_b_2d_array_v4i16_clamp(a: i64, b: i32, c: i32, d: i32, e: i16, f: i16, g: i16, h: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.array.v4i16.trap"]
        pub fn sust_b_2d_array_v4i16_trap(a: i64, b: i32, c: i32, d: i32, e: i16, f: i16, g: i16, h: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.array.v4i16.zero"]
        pub fn sust_b_2d_array_v4i16_zero(a: i64, b: i32, c: i32, d: i32, e: i16, f: i16, g: i16, h: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.array.v4i32.clamp"]
        pub fn sust_b_2d_array_v4i32_clamp(a: i64, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.array.v4i32.trap"]
        pub fn sust_b_2d_array_v4i32_trap(a: i64, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.array.v4i32.zero"]
        pub fn sust_b_2d_array_v4i32_zero(a: i64, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.array.v4i8.clamp"]
        pub fn sust_b_2d_array_v4i8_clamp(a: i64, b: i32, c: i32, d: i32, e: i16, f: i16, g: i16, h: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.array.v4i8.trap"]
        pub fn sust_b_2d_array_v4i8_trap(a: i64, b: i32, c: i32, d: i32, e: i16, f: i16, g: i16, h: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.array.v4i8.zero"]
        pub fn sust_b_2d_array_v4i8_zero(a: i64, b: i32, c: i32, d: i32, e: i16, f: i16, g: i16, h: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.i16.clamp"]
        pub fn sust_b_2d_i16_clamp(a: i64, b: i32, c: i32, d: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.i16.trap"]
        pub fn sust_b_2d_i16_trap(a: i64, b: i32, c: i32, d: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.i16.zero"]
        pub fn sust_b_2d_i16_zero(a: i64, b: i32, c: i32, d: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.i32.clamp"]
        pub fn sust_b_2d_i32_clamp(a: i64, b: i32, c: i32, d: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.i32.trap"]
        pub fn sust_b_2d_i32_trap(a: i64, b: i32, c: i32, d: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.i32.zero"]
        pub fn sust_b_2d_i32_zero(a: i64, b: i32, c: i32, d: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.i64.clamp"]
        pub fn sust_b_2d_i64_clamp(a: i64, b: i32, c: i32, d: i64) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.i64.trap"]
        pub fn sust_b_2d_i64_trap(a: i64, b: i32, c: i32, d: i64) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.i64.zero"]
        pub fn sust_b_2d_i64_zero(a: i64, b: i32, c: i32, d: i64) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.i8.clamp"]
        pub fn sust_b_2d_i8_clamp(a: i64, b: i32, c: i32, d: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.i8.trap"]
        pub fn sust_b_2d_i8_trap(a: i64, b: i32, c: i32, d: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.i8.zero"]
        pub fn sust_b_2d_i8_zero(a: i64, b: i32, c: i32, d: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.v2i16.clamp"]
        pub fn sust_b_2d_v2i16_clamp(a: i64, b: i32, c: i32, d: i16, e: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.v2i16.trap"]
        pub fn sust_b_2d_v2i16_trap(a: i64, b: i32, c: i32, d: i16, e: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.v2i16.zero"]
        pub fn sust_b_2d_v2i16_zero(a: i64, b: i32, c: i32, d: i16, e: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.v2i32.clamp"]
        pub fn sust_b_2d_v2i32_clamp(a: i64, b: i32, c: i32, d: i32, e: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.v2i32.trap"]
        pub fn sust_b_2d_v2i32_trap(a: i64, b: i32, c: i32, d: i32, e: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.v2i32.zero"]
        pub fn sust_b_2d_v2i32_zero(a: i64, b: i32, c: i32, d: i32, e: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.v2i64.clamp"]
        pub fn sust_b_2d_v2i64_clamp(a: i64, b: i32, c: i32, d: i64, e: i64) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.v2i64.trap"]
        pub fn sust_b_2d_v2i64_trap(a: i64, b: i32, c: i32, d: i64, e: i64) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.v2i64.zero"]
        pub fn sust_b_2d_v2i64_zero(a: i64, b: i32, c: i32, d: i64, e: i64) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.v2i8.clamp"]
        pub fn sust_b_2d_v2i8_clamp(a: i64, b: i32, c: i32, d: i16, e: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.v2i8.trap"]
        pub fn sust_b_2d_v2i8_trap(a: i64, b: i32, c: i32, d: i16, e: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.v2i8.zero"]
        pub fn sust_b_2d_v2i8_zero(a: i64, b: i32, c: i32, d: i16, e: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.v4i16.clamp"]
        pub fn sust_b_2d_v4i16_clamp(a: i64, b: i32, c: i32, d: i16, e: i16, f: i16, g: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.v4i16.trap"]
        pub fn sust_b_2d_v4i16_trap(a: i64, b: i32, c: i32, d: i16, e: i16, f: i16, g: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.v4i16.zero"]
        pub fn sust_b_2d_v4i16_zero(a: i64, b: i32, c: i32, d: i16, e: i16, f: i16, g: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.v4i32.clamp"]
        pub fn sust_b_2d_v4i32_clamp(a: i64, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.v4i32.trap"]
        pub fn sust_b_2d_v4i32_trap(a: i64, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.v4i32.zero"]
        pub fn sust_b_2d_v4i32_zero(a: i64, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.v4i8.clamp"]
        pub fn sust_b_2d_v4i8_clamp(a: i64, b: i32, c: i32, d: i16, e: i16, f: i16, g: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.v4i8.trap"]
        pub fn sust_b_2d_v4i8_trap(a: i64, b: i32, c: i32, d: i16, e: i16, f: i16, g: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.2d.v4i8.zero"]
        pub fn sust_b_2d_v4i8_zero(a: i64, b: i32, c: i32, d: i16, e: i16, f: i16, g: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.3d.i16.clamp"]
        pub fn sust_b_3d_i16_clamp(a: i64, b: i32, c: i32, d: i32, e: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.3d.i16.trap"]
        pub fn sust_b_3d_i16_trap(a: i64, b: i32, c: i32, d: i32, e: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.3d.i16.zero"]
        pub fn sust_b_3d_i16_zero(a: i64, b: i32, c: i32, d: i32, e: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.3d.i32.clamp"]
        pub fn sust_b_3d_i32_clamp(a: i64, b: i32, c: i32, d: i32, e: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.3d.i32.trap"]
        pub fn sust_b_3d_i32_trap(a: i64, b: i32, c: i32, d: i32, e: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.3d.i32.zero"]
        pub fn sust_b_3d_i32_zero(a: i64, b: i32, c: i32, d: i32, e: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.3d.i64.clamp"]
        pub fn sust_b_3d_i64_clamp(a: i64, b: i32, c: i32, d: i32, e: i64) -> ();
        #[link_name = "llvm.nvvm.sust.b.3d.i64.trap"]
        pub fn sust_b_3d_i64_trap(a: i64, b: i32, c: i32, d: i32, e: i64) -> ();
        #[link_name = "llvm.nvvm.sust.b.3d.i64.zero"]
        pub fn sust_b_3d_i64_zero(a: i64, b: i32, c: i32, d: i32, e: i64) -> ();
        #[link_name = "llvm.nvvm.sust.b.3d.i8.clamp"]
        pub fn sust_b_3d_i8_clamp(a: i64, b: i32, c: i32, d: i32, e: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.3d.i8.trap"]
        pub fn sust_b_3d_i8_trap(a: i64, b: i32, c: i32, d: i32, e: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.3d.i8.zero"]
        pub fn sust_b_3d_i8_zero(a: i64, b: i32, c: i32, d: i32, e: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.3d.v2i16.clamp"]
        pub fn sust_b_3d_v2i16_clamp(a: i64, b: i32, c: i32, d: i32, e: i16, f: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.3d.v2i16.trap"]
        pub fn sust_b_3d_v2i16_trap(a: i64, b: i32, c: i32, d: i32, e: i16, f: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.3d.v2i16.zero"]
        pub fn sust_b_3d_v2i16_zero(a: i64, b: i32, c: i32, d: i32, e: i16, f: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.3d.v2i32.clamp"]
        pub fn sust_b_3d_v2i32_clamp(a: i64, b: i32, c: i32, d: i32, e: i32, f: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.3d.v2i32.trap"]
        pub fn sust_b_3d_v2i32_trap(a: i64, b: i32, c: i32, d: i32, e: i32, f: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.3d.v2i32.zero"]
        pub fn sust_b_3d_v2i32_zero(a: i64, b: i32, c: i32, d: i32, e: i32, f: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.3d.v2i64.clamp"]
        pub fn sust_b_3d_v2i64_clamp(a: i64, b: i32, c: i32, d: i32, e: i64, f: i64) -> ();
        #[link_name = "llvm.nvvm.sust.b.3d.v2i64.trap"]
        pub fn sust_b_3d_v2i64_trap(a: i64, b: i32, c: i32, d: i32, e: i64, f: i64) -> ();
        #[link_name = "llvm.nvvm.sust.b.3d.v2i64.zero"]
        pub fn sust_b_3d_v2i64_zero(a: i64, b: i32, c: i32, d: i32, e: i64, f: i64) -> ();
        #[link_name = "llvm.nvvm.sust.b.3d.v2i8.clamp"]
        pub fn sust_b_3d_v2i8_clamp(a: i64, b: i32, c: i32, d: i32, e: i16, f: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.3d.v2i8.trap"]
        pub fn sust_b_3d_v2i8_trap(a: i64, b: i32, c: i32, d: i32, e: i16, f: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.3d.v2i8.zero"]
        pub fn sust_b_3d_v2i8_zero(a: i64, b: i32, c: i32, d: i32, e: i16, f: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.3d.v4i16.clamp"]
        pub fn sust_b_3d_v4i16_clamp(a: i64, b: i32, c: i32, d: i32, e: i16, f: i16, g: i16, h: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.3d.v4i16.trap"]
        pub fn sust_b_3d_v4i16_trap(a: i64, b: i32, c: i32, d: i32, e: i16, f: i16, g: i16, h: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.3d.v4i16.zero"]
        pub fn sust_b_3d_v4i16_zero(a: i64, b: i32, c: i32, d: i32, e: i16, f: i16, g: i16, h: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.3d.v4i32.clamp"]
        pub fn sust_b_3d_v4i32_clamp(a: i64, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.3d.v4i32.trap"]
        pub fn sust_b_3d_v4i32_trap(a: i64, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.3d.v4i32.zero"]
        pub fn sust_b_3d_v4i32_zero(a: i64, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> ();
        #[link_name = "llvm.nvvm.sust.b.3d.v4i8.clamp"]
        pub fn sust_b_3d_v4i8_clamp(a: i64, b: i32, c: i32, d: i32, e: i16, f: i16, g: i16, h: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.3d.v4i8.trap"]
        pub fn sust_b_3d_v4i8_trap(a: i64, b: i32, c: i32, d: i32, e: i16, f: i16, g: i16, h: i16) -> ();
        #[link_name = "llvm.nvvm.sust.b.3d.v4i8.zero"]
        pub fn sust_b_3d_v4i8_zero(a: i64, b: i32, c: i32, d: i32, e: i16, f: i16, g: i16, h: i16) -> ();
        #[link_name = "llvm.nvvm.sust.p.1d.array.i16.trap"]
        pub fn sust_p_1d_array_i16_trap(a: i64, b: i32, c: i32, d: i16) -> ();
        #[link_name = "llvm.nvvm.sust.p.1d.array.i32.trap"]
        pub fn sust_p_1d_array_i32_trap(a: i64, b: i32, c: i32, d: i32) -> ();
        #[link_name = "llvm.nvvm.sust.p.1d.array.i8.trap"]
        pub fn sust_p_1d_array_i8_trap(a: i64, b: i32, c: i32, d: i16) -> ();
        #[link_name = "llvm.nvvm.sust.p.1d.array.v2i16.trap"]
        pub fn sust_p_1d_array_v2i16_trap(a: i64, b: i32, c: i32, d: i16, e: i16) -> ();
        #[link_name = "llvm.nvvm.sust.p.1d.array.v2i32.trap"]
        pub fn sust_p_1d_array_v2i32_trap(a: i64, b: i32, c: i32, d: i32, e: i32) -> ();
        #[link_name = "llvm.nvvm.sust.p.1d.array.v2i8.trap"]
        pub fn sust_p_1d_array_v2i8_trap(a: i64, b: i32, c: i32, d: i16, e: i16) -> ();
        #[link_name = "llvm.nvvm.sust.p.1d.array.v4i16.trap"]
        pub fn sust_p_1d_array_v4i16_trap(a: i64, b: i32, c: i32, d: i16, e: i16, f: i16, g: i16) -> ();
        #[link_name = "llvm.nvvm.sust.p.1d.array.v4i32.trap"]
        pub fn sust_p_1d_array_v4i32_trap(a: i64, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32) -> ();
        #[link_name = "llvm.nvvm.sust.p.1d.array.v4i8.trap"]
        pub fn sust_p_1d_array_v4i8_trap(a: i64, b: i32, c: i32, d: i16, e: i16, f: i16, g: i16) -> ();
        #[link_name = "llvm.nvvm.sust.p.1d.i16.trap"]
        pub fn sust_p_1d_i16_trap(a: i64, b: i32, c: i16) -> ();
        #[link_name = "llvm.nvvm.sust.p.1d.i32.trap"]
        pub fn sust_p_1d_i32_trap(a: i64, b: i32, c: i32) -> ();
        #[link_name = "llvm.nvvm.sust.p.1d.i8.trap"]
        pub fn sust_p_1d_i8_trap(a: i64, b: i32, c: i16) -> ();
        #[link_name = "llvm.nvvm.sust.p.1d.v2i16.trap"]
        pub fn sust_p_1d_v2i16_trap(a: i64, b: i32, c: i16, d: i16) -> ();
        #[link_name = "llvm.nvvm.sust.p.1d.v2i32.trap"]
        pub fn sust_p_1d_v2i32_trap(a: i64, b: i32, c: i32, d: i32) -> ();
        #[link_name = "llvm.nvvm.sust.p.1d.v2i8.trap"]
        pub fn sust_p_1d_v2i8_trap(a: i64, b: i32, c: i16, d: i16) -> ();
        #[link_name = "llvm.nvvm.sust.p.1d.v4i16.trap"]
        pub fn sust_p_1d_v4i16_trap(a: i64, b: i32, c: i16, d: i16, e: i16, f: i16) -> ();
        #[link_name = "llvm.nvvm.sust.p.1d.v4i32.trap"]
        pub fn sust_p_1d_v4i32_trap(a: i64, b: i32, c: i32, d: i32, e: i32, f: i32) -> ();
        #[link_name = "llvm.nvvm.sust.p.1d.v4i8.trap"]
        pub fn sust_p_1d_v4i8_trap(a: i64, b: i32, c: i16, d: i16, e: i16, f: i16) -> ();
        #[link_name = "llvm.nvvm.sust.p.2d.array.i16.trap"]
        pub fn sust_p_2d_array_i16_trap(a: i64, b: i32, c: i32, d: i32, e: i16) -> ();
        #[link_name = "llvm.nvvm.sust.p.2d.array.i32.trap"]
        pub fn sust_p_2d_array_i32_trap(a: i64, b: i32, c: i32, d: i32, e: i32) -> ();
        #[link_name = "llvm.nvvm.sust.p.2d.array.i8.trap"]
        pub fn sust_p_2d_array_i8_trap(a: i64, b: i32, c: i32, d: i32, e: i16) -> ();
        #[link_name = "llvm.nvvm.sust.p.2d.array.v2i16.trap"]
        pub fn sust_p_2d_array_v2i16_trap(a: i64, b: i32, c: i32, d: i32, e: i16, f: i16) -> ();
        #[link_name = "llvm.nvvm.sust.p.2d.array.v2i32.trap"]
        pub fn sust_p_2d_array_v2i32_trap(a: i64, b: i32, c: i32, d: i32, e: i32, f: i32) -> ();
        #[link_name = "llvm.nvvm.sust.p.2d.array.v2i8.trap"]
        pub fn sust_p_2d_array_v2i8_trap(a: i64, b: i32, c: i32, d: i32, e: i16, f: i16) -> ();
        #[link_name = "llvm.nvvm.sust.p.2d.array.v4i16.trap"]
        pub fn sust_p_2d_array_v4i16_trap(a: i64, b: i32, c: i32, d: i32, e: i16, f: i16, g: i16, h: i16) -> ();
        #[link_name = "llvm.nvvm.sust.p.2d.array.v4i32.trap"]
        pub fn sust_p_2d_array_v4i32_trap(a: i64, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> ();
        #[link_name = "llvm.nvvm.sust.p.2d.array.v4i8.trap"]
        pub fn sust_p_2d_array_v4i8_trap(a: i64, b: i32, c: i32, d: i32, e: i16, f: i16, g: i16, h: i16) -> ();
        #[link_name = "llvm.nvvm.sust.p.2d.i16.trap"]
        pub fn sust_p_2d_i16_trap(a: i64, b: i32, c: i32, d: i16) -> ();
        #[link_name = "llvm.nvvm.sust.p.2d.i32.trap"]
        pub fn sust_p_2d_i32_trap(a: i64, b: i32, c: i32, d: i32) -> ();
        #[link_name = "llvm.nvvm.sust.p.2d.i8.trap"]
        pub fn sust_p_2d_i8_trap(a: i64, b: i32, c: i32, d: i16) -> ();
        #[link_name = "llvm.nvvm.sust.p.2d.v2i16.trap"]
        pub fn sust_p_2d_v2i16_trap(a: i64, b: i32, c: i32, d: i16, e: i16) -> ();
        #[link_name = "llvm.nvvm.sust.p.2d.v2i32.trap"]
        pub fn sust_p_2d_v2i32_trap(a: i64, b: i32, c: i32, d: i32, e: i32) -> ();
        #[link_name = "llvm.nvvm.sust.p.2d.v2i8.trap"]
        pub fn sust_p_2d_v2i8_trap(a: i64, b: i32, c: i32, d: i16, e: i16) -> ();
        #[link_name = "llvm.nvvm.sust.p.2d.v4i16.trap"]
        pub fn sust_p_2d_v4i16_trap(a: i64, b: i32, c: i32, d: i16, e: i16, f: i16, g: i16) -> ();
        #[link_name = "llvm.nvvm.sust.p.2d.v4i32.trap"]
        pub fn sust_p_2d_v4i32_trap(a: i64, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32) -> ();
        #[link_name = "llvm.nvvm.sust.p.2d.v4i8.trap"]
        pub fn sust_p_2d_v4i8_trap(a: i64, b: i32, c: i32, d: i16, e: i16, f: i16, g: i16) -> ();
        #[link_name = "llvm.nvvm.sust.p.3d.i16.trap"]
        pub fn sust_p_3d_i16_trap(a: i64, b: i32, c: i32, d: i32, e: i16) -> ();
        #[link_name = "llvm.nvvm.sust.p.3d.i32.trap"]
        pub fn sust_p_3d_i32_trap(a: i64, b: i32, c: i32, d: i32, e: i32) -> ();
        #[link_name = "llvm.nvvm.sust.p.3d.i8.trap"]
        pub fn sust_p_3d_i8_trap(a: i64, b: i32, c: i32, d: i32, e: i16) -> ();
        #[link_name = "llvm.nvvm.sust.p.3d.v2i16.trap"]
        pub fn sust_p_3d_v2i16_trap(a: i64, b: i32, c: i32, d: i32, e: i16, f: i16) -> ();
        #[link_name = "llvm.nvvm.sust.p.3d.v2i32.trap"]
        pub fn sust_p_3d_v2i32_trap(a: i64, b: i32, c: i32, d: i32, e: i32, f: i32) -> ();
        #[link_name = "llvm.nvvm.sust.p.3d.v2i8.trap"]
        pub fn sust_p_3d_v2i8_trap(a: i64, b: i32, c: i32, d: i32, e: i16, f: i16) -> ();
        #[link_name = "llvm.nvvm.sust.p.3d.v4i16.trap"]
        pub fn sust_p_3d_v4i16_trap(a: i64, b: i32, c: i32, d: i32, e: i16, f: i16, g: i16, h: i16) -> ();
        #[link_name = "llvm.nvvm.sust.p.3d.v4i32.trap"]
        pub fn sust_p_3d_v4i32_trap(a: i64, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> ();
        #[link_name = "llvm.nvvm.sust.p.3d.v4i8.trap"]
        pub fn sust_p_3d_v4i8_trap(a: i64, b: i32, c: i32, d: i32, e: i16, f: i16, g: i16, h: i16) -> ();
        #[link_name = "llvm.nvvm.swap.lo.hi.b64"]
        pub fn swap_lo_hi_b64(a: i64) -> i64;
        #[link_name = "llvm.nvvm.texsurf.handle.internal"]
        pub fn texsurf_handle_internal(a: *mut i8) -> i64;
        #[link_name = "llvm.nvvm.trunc.d"]
        pub fn trunc_d(a: f64) -> f64;
        #[link_name = "llvm.nvvm.trunc.f"]
        pub fn trunc_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.trunc.ftz.f"]
        pub fn trunc_ftz_f(a: f32) -> f32;
        #[link_name = "llvm.nvvm.txq.array.size"]
        pub fn txq_array_size(a: i64) -> i32;
        #[link_name = "llvm.nvvm.txq.channel.data.type"]
        pub fn txq_channel_data_type(a: i64) -> i32;
        #[link_name = "llvm.nvvm.txq.channel.order"]
        pub fn txq_channel_order(a: i64) -> i32;
        #[link_name = "llvm.nvvm.txq.depth"]
        pub fn txq_depth(a: i64) -> i32;
        #[link_name = "llvm.nvvm.txq.height"]
        pub fn txq_height(a: i64) -> i32;
        #[link_name = "llvm.nvvm.txq.num.mipmap.levels"]
        pub fn txq_num_mipmap_levels(a: i64) -> i32;
        #[link_name = "llvm.nvvm.txq.num.samples"]
        pub fn txq_num_samples(a: i64) -> i32;
        #[link_name = "llvm.nvvm.txq.width"]
        pub fn txq_width(a: i64) -> i32;
        #[link_name = "llvm.nvvm.ui2d.rm"]
        pub fn ui2d_rm(a: i32) -> f64;
        #[link_name = "llvm.nvvm.ui2d.rn"]
        pub fn ui2d_rn(a: i32) -> f64;
        #[link_name = "llvm.nvvm.ui2d.rp"]
        pub fn ui2d_rp(a: i32) -> f64;
        #[link_name = "llvm.nvvm.ui2d.rz"]
        pub fn ui2d_rz(a: i32) -> f64;
        #[link_name = "llvm.nvvm.ui2f.rm"]
        pub fn ui2f_rm(a: i32) -> f32;
        #[link_name = "llvm.nvvm.ui2f.rn"]
        pub fn ui2f_rn(a: i32) -> f32;
        #[link_name = "llvm.nvvm.ui2f.rp"]
        pub fn ui2f_rp(a: i32) -> f32;
        #[link_name = "llvm.nvvm.ui2f.rz"]
        pub fn ui2f_rz(a: i32) -> f32;
        #[link_name = "llvm.nvvm.ull2d.rm"]
        pub fn ull2d_rm(a: i64) -> f64;
        #[link_name = "llvm.nvvm.ull2d.rn"]
        pub fn ull2d_rn(a: i64) -> f64;
        #[link_name = "llvm.nvvm.ull2d.rp"]
        pub fn ull2d_rp(a: i64) -> f64;
        #[link_name = "llvm.nvvm.ull2d.rz"]
        pub fn ull2d_rz(a: i64) -> f64;
        #[link_name = "llvm.nvvm.ull2f.rm"]
        pub fn ull2f_rm(a: i64) -> f32;
        #[link_name = "llvm.nvvm.ull2f.rn"]
        pub fn ull2f_rn(a: i64) -> f32;
        #[link_name = "llvm.nvvm.ull2f.rp"]
        pub fn ull2f_rp(a: i64) -> f32;
        #[link_name = "llvm.nvvm.ull2f.rz"]
        pub fn ull2f_rz(a: i64) -> f32;
    }
}
/// LLVM intrinsics for the ppc architecture.
pub mod ppc {
    extern {
        #[link_name = "llvm.ppc.altivec.dss"]
        pub fn altivec_dss(a: i32) -> ();
        #[link_name = "llvm.ppc.altivec.dssall"]
        pub fn altivec_dssall() -> ();
        #[link_name = "llvm.ppc.altivec.dst"]
        pub fn altivec_dst(a: *mut i8, b: i32, c: i32) -> ();
        #[link_name = "llvm.ppc.altivec.dstst"]
        pub fn altivec_dstst(a: *mut i8, b: i32, c: i32) -> ();
        #[link_name = "llvm.ppc.altivec.dststt"]
        pub fn altivec_dststt(a: *mut i8, b: i32, c: i32) -> ();
        #[link_name = "llvm.ppc.altivec.dstt"]
        pub fn altivec_dstt(a: *mut i8, b: i32, c: i32) -> ();
        #[link_name = "llvm.ppc.altivec.lvebx"]
        pub fn altivec_lvebx(a: *mut i8) -> ::simdty::i8x16;
        #[link_name = "llvm.ppc.altivec.lvehx"]
        pub fn altivec_lvehx(a: *mut i8) -> ::simdty::i16x8;
        #[link_name = "llvm.ppc.altivec.lvewx"]
        pub fn altivec_lvewx(a: *mut i8) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.lvsl"]
        pub fn altivec_lvsl(a: *mut i8) -> ::simdty::i8x16;
        #[link_name = "llvm.ppc.altivec.lvsr"]
        pub fn altivec_lvsr(a: *mut i8) -> ::simdty::i8x16;
        #[link_name = "llvm.ppc.altivec.lvx"]
        pub fn altivec_lvx(a: *mut i8) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.lvxl"]
        pub fn altivec_lvxl(a: *mut i8) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.mfvscr"]
        pub fn altivec_mfvscr() -> ::simdty::i16x8;
        #[link_name = "llvm.ppc.altivec.mtvscr"]
        pub fn altivec_mtvscr(a: ::simdty::i32x4) -> ();
        #[link_name = "llvm.ppc.altivec.stvebx"]
        pub fn altivec_stvebx(a: ::simdty::i8x16, b: *mut i8) -> ();
        #[link_name = "llvm.ppc.altivec.stvehx"]
        pub fn altivec_stvehx(a: ::simdty::i16x8, b: *mut i8) -> ();
        #[link_name = "llvm.ppc.altivec.stvewx"]
        pub fn altivec_stvewx(a: ::simdty::i32x4, b: *mut i8) -> ();
        #[link_name = "llvm.ppc.altivec.stvx"]
        pub fn altivec_stvx(a: ::simdty::i32x4, b: *mut i8) -> ();
        #[link_name = "llvm.ppc.altivec.stvxl"]
        pub fn altivec_stvxl(a: ::simdty::i32x4, b: *mut i8) -> ();
        #[link_name = "llvm.ppc.altivec.vaddcuw"]
        pub fn altivec_vaddcuw(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vaddsbs"]
        pub fn altivec_vaddsbs(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.ppc.altivec.vaddshs"]
        pub fn altivec_vaddshs(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.ppc.altivec.vaddsws"]
        pub fn altivec_vaddsws(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vaddubs"]
        pub fn altivec_vaddubs(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.ppc.altivec.vadduhs"]
        pub fn altivec_vadduhs(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.ppc.altivec.vadduws"]
        pub fn altivec_vadduws(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vavgsb"]
        pub fn altivec_vavgsb(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.ppc.altivec.vavgsh"]
        pub fn altivec_vavgsh(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.ppc.altivec.vavgsw"]
        pub fn altivec_vavgsw(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vavgub"]
        pub fn altivec_vavgub(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.ppc.altivec.vavguh"]
        pub fn altivec_vavguh(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.ppc.altivec.vavguw"]
        pub fn altivec_vavguw(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vcfsx"]
        pub fn altivec_vcfsx(a: ::simdty::i32x4, b: i32) -> ::simdty::f32x4;
        #[link_name = "llvm.ppc.altivec.vcfux"]
        pub fn altivec_vcfux(a: ::simdty::i32x4, b: i32) -> ::simdty::f32x4;
        #[link_name = "llvm.ppc.altivec.vcmpbfp"]
        pub fn altivec_vcmpbfp(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vcmpbfp.p"]
        pub fn altivec_vcmpbfp_p(a: i32, b: ::simdty::f32x4, c: ::simdty::f32x4) -> i32;
        #[link_name = "llvm.ppc.altivec.vcmpeqfp"]
        pub fn altivec_vcmpeqfp(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vcmpeqfp.p"]
        pub fn altivec_vcmpeqfp_p(a: i32, b: ::simdty::f32x4, c: ::simdty::f32x4) -> i32;
        #[link_name = "llvm.ppc.altivec.vcmpequb"]
        pub fn altivec_vcmpequb(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.ppc.altivec.vcmpequb.p"]
        pub fn altivec_vcmpequb_p(a: i32, b: ::simdty::i8x16, c: ::simdty::i8x16) -> i32;
        #[link_name = "llvm.ppc.altivec.vcmpequh"]
        pub fn altivec_vcmpequh(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.ppc.altivec.vcmpequh.p"]
        pub fn altivec_vcmpequh_p(a: i32, b: ::simdty::i16x8, c: ::simdty::i16x8) -> i32;
        #[link_name = "llvm.ppc.altivec.vcmpequw"]
        pub fn altivec_vcmpequw(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vcmpequw.p"]
        pub fn altivec_vcmpequw_p(a: i32, b: ::simdty::i32x4, c: ::simdty::i32x4) -> i32;
        #[link_name = "llvm.ppc.altivec.vcmpgefp"]
        pub fn altivec_vcmpgefp(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vcmpgefp.p"]
        pub fn altivec_vcmpgefp_p(a: i32, b: ::simdty::f32x4, c: ::simdty::f32x4) -> i32;
        #[link_name = "llvm.ppc.altivec.vcmpgtfp"]
        pub fn altivec_vcmpgtfp(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vcmpgtfp.p"]
        pub fn altivec_vcmpgtfp_p(a: i32, b: ::simdty::f32x4, c: ::simdty::f32x4) -> i32;
        #[link_name = "llvm.ppc.altivec.vcmpgtsb"]
        pub fn altivec_vcmpgtsb(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.ppc.altivec.vcmpgtsb.p"]
        pub fn altivec_vcmpgtsb_p(a: i32, b: ::simdty::i8x16, c: ::simdty::i8x16) -> i32;
        #[link_name = "llvm.ppc.altivec.vcmpgtsh"]
        pub fn altivec_vcmpgtsh(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.ppc.altivec.vcmpgtsh.p"]
        pub fn altivec_vcmpgtsh_p(a: i32, b: ::simdty::i16x8, c: ::simdty::i16x8) -> i32;
        #[link_name = "llvm.ppc.altivec.vcmpgtsw"]
        pub fn altivec_vcmpgtsw(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vcmpgtsw.p"]
        pub fn altivec_vcmpgtsw_p(a: i32, b: ::simdty::i32x4, c: ::simdty::i32x4) -> i32;
        #[link_name = "llvm.ppc.altivec.vcmpgtub"]
        pub fn altivec_vcmpgtub(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.ppc.altivec.vcmpgtub.p"]
        pub fn altivec_vcmpgtub_p(a: i32, b: ::simdty::i8x16, c: ::simdty::i8x16) -> i32;
        #[link_name = "llvm.ppc.altivec.vcmpgtuh"]
        pub fn altivec_vcmpgtuh(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.ppc.altivec.vcmpgtuh.p"]
        pub fn altivec_vcmpgtuh_p(a: i32, b: ::simdty::i16x8, c: ::simdty::i16x8) -> i32;
        #[link_name = "llvm.ppc.altivec.vcmpgtuw"]
        pub fn altivec_vcmpgtuw(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vcmpgtuw.p"]
        pub fn altivec_vcmpgtuw_p(a: i32, b: ::simdty::i32x4, c: ::simdty::i32x4) -> i32;
        #[link_name = "llvm.ppc.altivec.vctsxs"]
        pub fn altivec_vctsxs(a: ::simdty::f32x4, b: i32) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vctuxs"]
        pub fn altivec_vctuxs(a: ::simdty::f32x4, b: i32) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vexptefp"]
        pub fn altivec_vexptefp(a: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.ppc.altivec.vlogefp"]
        pub fn altivec_vlogefp(a: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.ppc.altivec.vmaddfp"]
        pub fn altivec_vmaddfp(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.ppc.altivec.vmaxfp"]
        pub fn altivec_vmaxfp(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.ppc.altivec.vmaxsb"]
        pub fn altivec_vmaxsb(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.ppc.altivec.vmaxsh"]
        pub fn altivec_vmaxsh(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.ppc.altivec.vmaxsw"]
        pub fn altivec_vmaxsw(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vmaxub"]
        pub fn altivec_vmaxub(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.ppc.altivec.vmaxuh"]
        pub fn altivec_vmaxuh(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.ppc.altivec.vmaxuw"]
        pub fn altivec_vmaxuw(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vmhaddshs"]
        pub fn altivec_vmhaddshs(a: ::simdty::i16x8, b: ::simdty::i16x8, c: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.ppc.altivec.vmhraddshs"]
        pub fn altivec_vmhraddshs(a: ::simdty::i16x8, b: ::simdty::i16x8, c: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.ppc.altivec.vminfp"]
        pub fn altivec_vminfp(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.ppc.altivec.vminsb"]
        pub fn altivec_vminsb(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.ppc.altivec.vminsh"]
        pub fn altivec_vminsh(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.ppc.altivec.vminsw"]
        pub fn altivec_vminsw(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vminub"]
        pub fn altivec_vminub(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.ppc.altivec.vminuh"]
        pub fn altivec_vminuh(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.ppc.altivec.vminuw"]
        pub fn altivec_vminuw(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vmladduhm"]
        pub fn altivec_vmladduhm(a: ::simdty::i16x8, b: ::simdty::i16x8, c: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.ppc.altivec.vmsummbm"]
        pub fn altivec_vmsummbm(a: ::simdty::i8x16, b: ::simdty::i8x16, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vmsumshm"]
        pub fn altivec_vmsumshm(a: ::simdty::i16x8, b: ::simdty::i16x8, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vmsumshs"]
        pub fn altivec_vmsumshs(a: ::simdty::i16x8, b: ::simdty::i16x8, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vmsumubm"]
        pub fn altivec_vmsumubm(a: ::simdty::i8x16, b: ::simdty::i8x16, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vmsumuhm"]
        pub fn altivec_vmsumuhm(a: ::simdty::i16x8, b: ::simdty::i16x8, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vmsumuhs"]
        pub fn altivec_vmsumuhs(a: ::simdty::i16x8, b: ::simdty::i16x8, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vmulesb"]
        pub fn altivec_vmulesb(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i16x8;
        #[link_name = "llvm.ppc.altivec.vmulesh"]
        pub fn altivec_vmulesh(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vmuleub"]
        pub fn altivec_vmuleub(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i16x8;
        #[link_name = "llvm.ppc.altivec.vmuleuh"]
        pub fn altivec_vmuleuh(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vmulosb"]
        pub fn altivec_vmulosb(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i16x8;
        #[link_name = "llvm.ppc.altivec.vmulosh"]
        pub fn altivec_vmulosh(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vmuloub"]
        pub fn altivec_vmuloub(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i16x8;
        #[link_name = "llvm.ppc.altivec.vmulouh"]
        pub fn altivec_vmulouh(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vnmsubfp"]
        pub fn altivec_vnmsubfp(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.ppc.altivec.vperm"]
        pub fn altivec_vperm(a: ::simdty::i32x4, b: ::simdty::i32x4, c: ::simdty::i8x16) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vpkpx"]
        pub fn altivec_vpkpx(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i16x8;
        #[link_name = "llvm.ppc.altivec.vpkshss"]
        pub fn altivec_vpkshss(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i8x16;
        #[link_name = "llvm.ppc.altivec.vpkshus"]
        pub fn altivec_vpkshus(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i8x16;
        #[link_name = "llvm.ppc.altivec.vpkswss"]
        pub fn altivec_vpkswss(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i8x16;
        #[link_name = "llvm.ppc.altivec.vpkswus"]
        pub fn altivec_vpkswus(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i16x8;
        #[link_name = "llvm.ppc.altivec.vpkuhus"]
        pub fn altivec_vpkuhus(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i8x16;
        #[link_name = "llvm.ppc.altivec.vpkuwus"]
        pub fn altivec_vpkuwus(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i16x8;
        #[link_name = "llvm.ppc.altivec.vrefp"]
        pub fn altivec_vrefp(a: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.ppc.altivec.vrfim"]
        pub fn altivec_vrfim(a: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.ppc.altivec.vrfin"]
        pub fn altivec_vrfin(a: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.ppc.altivec.vrfip"]
        pub fn altivec_vrfip(a: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.ppc.altivec.vrfiz"]
        pub fn altivec_vrfiz(a: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.ppc.altivec.vrlb"]
        pub fn altivec_vrlb(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.ppc.altivec.vrlh"]
        pub fn altivec_vrlh(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.ppc.altivec.vrlw"]
        pub fn altivec_vrlw(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vrsqrtefp"]
        pub fn altivec_vrsqrtefp(a: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.ppc.altivec.vsel"]
        pub fn altivec_vsel(a: ::simdty::i32x4, b: ::simdty::i32x4, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vsl"]
        pub fn altivec_vsl(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vslb"]
        pub fn altivec_vslb(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.ppc.altivec.vslh"]
        pub fn altivec_vslh(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.ppc.altivec.vslo"]
        pub fn altivec_vslo(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vslw"]
        pub fn altivec_vslw(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vsr"]
        pub fn altivec_vsr(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vsrab"]
        pub fn altivec_vsrab(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.ppc.altivec.vsrah"]
        pub fn altivec_vsrah(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.ppc.altivec.vsraw"]
        pub fn altivec_vsraw(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vsrb"]
        pub fn altivec_vsrb(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.ppc.altivec.vsrh"]
        pub fn altivec_vsrh(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.ppc.altivec.vsro"]
        pub fn altivec_vsro(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vsrw"]
        pub fn altivec_vsrw(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vsubcuw"]
        pub fn altivec_vsubcuw(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vsubsbs"]
        pub fn altivec_vsubsbs(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.ppc.altivec.vsubshs"]
        pub fn altivec_vsubshs(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.ppc.altivec.vsubsws"]
        pub fn altivec_vsubsws(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vsububs"]
        pub fn altivec_vsububs(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.ppc.altivec.vsubuhs"]
        pub fn altivec_vsubuhs(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.ppc.altivec.vsubuws"]
        pub fn altivec_vsubuws(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vsum2sws"]
        pub fn altivec_vsum2sws(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vsum4sbs"]
        pub fn altivec_vsum4sbs(a: ::simdty::i8x16, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vsum4shs"]
        pub fn altivec_vsum4shs(a: ::simdty::i16x8, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vsum4ubs"]
        pub fn altivec_vsum4ubs(a: ::simdty::i8x16, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vsumsws"]
        pub fn altivec_vsumsws(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vupkhpx"]
        pub fn altivec_vupkhpx(a: ::simdty::i16x8) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vupkhsb"]
        pub fn altivec_vupkhsb(a: ::simdty::i8x16) -> ::simdty::i16x8;
        #[link_name = "llvm.ppc.altivec.vupkhsh"]
        pub fn altivec_vupkhsh(a: ::simdty::i16x8) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vupklpx"]
        pub fn altivec_vupklpx(a: ::simdty::i16x8) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.altivec.vupklsb"]
        pub fn altivec_vupklsb(a: ::simdty::i8x16) -> ::simdty::i16x8;
        #[link_name = "llvm.ppc.altivec.vupklsh"]
        pub fn altivec_vupklsh(a: ::simdty::i16x8) -> ::simdty::i32x4;
        #[link_name = "llvm.ppc.dcba"]
        pub fn dcba(a: *mut i8) -> ();
        #[link_name = "llvm.ppc.dcbf"]
        pub fn dcbf(a: *mut i8) -> ();
        #[link_name = "llvm.ppc.dcbi"]
        pub fn dcbi(a: *mut i8) -> ();
        #[link_name = "llvm.ppc.dcbst"]
        pub fn dcbst(a: *mut i8) -> ();
        #[link_name = "llvm.ppc.dcbt"]
        pub fn dcbt(a: *mut i8) -> ();
        #[link_name = "llvm.ppc.dcbtst"]
        pub fn dcbtst(a: *mut i8) -> ();
        #[link_name = "llvm.ppc.dcbz"]
        pub fn dcbz(a: *mut i8) -> ();
        #[link_name = "llvm.ppc.dcbzl"]
        pub fn dcbzl(a: *mut i8) -> ();
        #[link_name = "llvm.ppc.is.decremented.ctr.nonzero"]
        pub fn is_decremented_ctr_nonzero() -> bool;
        #[link_name = "llvm.ppc.lwsync"]
        pub fn lwsync() -> ();
        #[link_name = "llvm.ppc.sync"]
        pub fn sync() -> ();
    }
}
/// LLVM intrinsics for the ptx architecture.
pub mod ptx {
    extern {
        #[link_name = "llvm.ptx.bar.sync"]
        pub fn bar_sync(a: i32) -> ();
        #[link_name = "llvm.ptx.read.clock"]
        pub fn read_clock() -> i32;
        #[link_name = "llvm.ptx.read.clock64"]
        pub fn read_clock64() -> i64;
        #[link_name = "llvm.ptx.read.ctaid.w"]
        pub fn read_ctaid_w() -> i32;
        #[link_name = "llvm.ptx.read.ctaid.x"]
        pub fn read_ctaid_x() -> i32;
        #[link_name = "llvm.ptx.read.ctaid.y"]
        pub fn read_ctaid_y() -> i32;
        #[link_name = "llvm.ptx.read.ctaid.z"]
        pub fn read_ctaid_z() -> i32;
        #[link_name = "llvm.ptx.read.gridid"]
        pub fn read_gridid() -> i32;
        #[link_name = "llvm.ptx.read.laneid"]
        pub fn read_laneid() -> i32;
        #[link_name = "llvm.ptx.read.lanemask.eq"]
        pub fn read_lanemask_eq() -> i32;
        #[link_name = "llvm.ptx.read.lanemask.ge"]
        pub fn read_lanemask_ge() -> i32;
        #[link_name = "llvm.ptx.read.lanemask.gt"]
        pub fn read_lanemask_gt() -> i32;
        #[link_name = "llvm.ptx.read.lanemask.le"]
        pub fn read_lanemask_le() -> i32;
        #[link_name = "llvm.ptx.read.lanemask.lt"]
        pub fn read_lanemask_lt() -> i32;
        #[link_name = "llvm.ptx.read.nctaid.w"]
        pub fn read_nctaid_w() -> i32;
        #[link_name = "llvm.ptx.read.nctaid.x"]
        pub fn read_nctaid_x() -> i32;
        #[link_name = "llvm.ptx.read.nctaid.y"]
        pub fn read_nctaid_y() -> i32;
        #[link_name = "llvm.ptx.read.nctaid.z"]
        pub fn read_nctaid_z() -> i32;
        #[link_name = "llvm.ptx.read.nsmid"]
        pub fn read_nsmid() -> i32;
        #[link_name = "llvm.ptx.read.ntid.w"]
        pub fn read_ntid_w() -> i32;
        #[link_name = "llvm.ptx.read.ntid.x"]
        pub fn read_ntid_x() -> i32;
        #[link_name = "llvm.ptx.read.ntid.y"]
        pub fn read_ntid_y() -> i32;
        #[link_name = "llvm.ptx.read.ntid.z"]
        pub fn read_ntid_z() -> i32;
        #[link_name = "llvm.ptx.read.nwarpid"]
        pub fn read_nwarpid() -> i32;
        #[link_name = "llvm.ptx.read.pm0"]
        pub fn read_pm0() -> i32;
        #[link_name = "llvm.ptx.read.pm1"]
        pub fn read_pm1() -> i32;
        #[link_name = "llvm.ptx.read.pm2"]
        pub fn read_pm2() -> i32;
        #[link_name = "llvm.ptx.read.pm3"]
        pub fn read_pm3() -> i32;
        #[link_name = "llvm.ptx.read.smid"]
        pub fn read_smid() -> i32;
        #[link_name = "llvm.ptx.read.tid.w"]
        pub fn read_tid_w() -> i32;
        #[link_name = "llvm.ptx.read.tid.x"]
        pub fn read_tid_x() -> i32;
        #[link_name = "llvm.ptx.read.tid.y"]
        pub fn read_tid_y() -> i32;
        #[link_name = "llvm.ptx.read.tid.z"]
        pub fn read_tid_z() -> i32;
        #[link_name = "llvm.ptx.read.warpid"]
        pub fn read_warpid() -> i32;
    }
}
/// LLVM intrinsics for the r600 architecture.
pub mod r600 {
    extern {
        #[link_name = "llvm.r600.read.global.size.x"]
        pub fn read_global_size_x() -> i32;
        #[link_name = "llvm.r600.read.global.size.y"]
        pub fn read_global_size_y() -> i32;
        #[link_name = "llvm.r600.read.global.size.z"]
        pub fn read_global_size_z() -> i32;
        #[link_name = "llvm.r600.read.local.size.x"]
        pub fn read_local_size_x() -> i32;
        #[link_name = "llvm.r600.read.local.size.y"]
        pub fn read_local_size_y() -> i32;
        #[link_name = "llvm.r600.read.local.size.z"]
        pub fn read_local_size_z() -> i32;
        #[link_name = "llvm.r600.read.ngroups.x"]
        pub fn read_ngroups_x() -> i32;
        #[link_name = "llvm.r600.read.ngroups.y"]
        pub fn read_ngroups_y() -> i32;
        #[link_name = "llvm.r600.read.ngroups.z"]
        pub fn read_ngroups_z() -> i32;
        #[link_name = "llvm.r600.read.tgid.x"]
        pub fn read_tgid_x() -> i32;
        #[link_name = "llvm.r600.read.tgid.y"]
        pub fn read_tgid_y() -> i32;
        #[link_name = "llvm.r600.read.tgid.z"]
        pub fn read_tgid_z() -> i32;
        #[link_name = "llvm.r600.read.tidig.x"]
        pub fn read_tidig_x() -> i32;
        #[link_name = "llvm.r600.read.tidig.y"]
        pub fn read_tidig_y() -> i32;
        #[link_name = "llvm.r600.read.tidig.z"]
        pub fn read_tidig_z() -> i32;
    }
}
/// LLVM intrinsics for the x86 architecture.
pub mod x86 {
    extern {
        #[link_name = "llvm.x86.addcarry.u32"]
        pub fn addcarry_u32(a: i8, b: i32, c: i32, d: *mut i8) -> i8;
        #[link_name = "llvm.x86.addcarry.u64"]
        pub fn addcarry_u64(a: i8, b: i64, c: i64, d: *mut i8) -> i8;
        #[link_name = "llvm.x86.addcarryx.u32"]
        pub fn addcarryx_u32(a: i8, b: i32, c: i32, d: *mut i8) -> i8;
        #[link_name = "llvm.x86.addcarryx.u64"]
        pub fn addcarryx_u64(a: i8, b: i64, c: i64, d: *mut i8) -> i8;
        #[link_name = "llvm.x86.aesni.aesdec"]
        pub fn aesni_aesdec(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.aesni.aesdeclast"]
        pub fn aesni_aesdeclast(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.aesni.aesenc"]
        pub fn aesni_aesenc(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.aesni.aesenclast"]
        pub fn aesni_aesenclast(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.aesni.aesimc"]
        pub fn aesni_aesimc(a: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.aesni.aeskeygenassist"]
        pub fn aesni_aeskeygenassist(a: ::simdty::i64x2, b: i8) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.avx2.gather.d.d"]
        pub fn avx2_gather_d_d(a: ::simdty::i32x4, b: *mut i8, c: ::simdty::i32x4, d: ::simdty::i32x4, e: i8) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.avx2.gather.d.d.256"]
        pub fn avx2_gather_d_d_256(a: ::simdty::i32x8, b: *mut i8, c: ::simdty::i32x8, d: ::simdty::i32x8, e: i8) -> ::simdty::i32x8;
        #[link_name = "llvm.x86.avx2.gather.d.pd"]
        pub fn avx2_gather_d_pd(a: ::simdty::f64x2, b: *mut i8, c: ::simdty::i32x4, d: ::simdty::f64x2, e: i8) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.avx2.gather.d.pd.256"]
        pub fn avx2_gather_d_pd_256(a: ::simdty::f64x4, b: *mut i8, c: ::simdty::i32x4, d: ::simdty::f64x4, e: i8) -> ::simdty::f64x4;
        #[link_name = "llvm.x86.avx2.gather.d.ps"]
        pub fn avx2_gather_d_ps(a: ::simdty::f32x4, b: *mut i8, c: ::simdty::i32x4, d: ::simdty::f32x4, e: i8) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.avx2.gather.d.ps.256"]
        pub fn avx2_gather_d_ps_256(a: ::simdty::f32x8, b: *mut i8, c: ::simdty::i32x8, d: ::simdty::f32x8, e: i8) -> ::simdty::f32x8;
        #[link_name = "llvm.x86.avx2.gather.d.q"]
        pub fn avx2_gather_d_q(a: ::simdty::i64x2, b: *mut i8, c: ::simdty::i32x4, d: ::simdty::i64x2, e: i8) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.avx2.gather.d.q.256"]
        pub fn avx2_gather_d_q_256(a: ::simdty::i64x4, b: *mut i8, c: ::simdty::i32x4, d: ::simdty::i64x4, e: i8) -> ::simdty::i64x4;
        #[link_name = "llvm.x86.avx2.gather.q.d"]
        pub fn avx2_gather_q_d(a: ::simdty::i32x4, b: *mut i8, c: ::simdty::i64x2, d: ::simdty::i32x4, e: i8) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.avx2.gather.q.d.256"]
        pub fn avx2_gather_q_d_256(a: ::simdty::i32x4, b: *mut i8, c: ::simdty::i64x4, d: ::simdty::i32x4, e: i8) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.avx2.gather.q.pd"]
        pub fn avx2_gather_q_pd(a: ::simdty::f64x2, b: *mut i8, c: ::simdty::i64x2, d: ::simdty::f64x2, e: i8) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.avx2.gather.q.pd.256"]
        pub fn avx2_gather_q_pd_256(a: ::simdty::f64x4, b: *mut i8, c: ::simdty::i64x4, d: ::simdty::f64x4, e: i8) -> ::simdty::f64x4;
        #[link_name = "llvm.x86.avx2.gather.q.ps"]
        pub fn avx2_gather_q_ps(a: ::simdty::f32x4, b: *mut i8, c: ::simdty::i64x2, d: ::simdty::f32x4, e: i8) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.avx2.gather.q.ps.256"]
        pub fn avx2_gather_q_ps_256(a: ::simdty::f32x4, b: *mut i8, c: ::simdty::i64x4, d: ::simdty::f32x4, e: i8) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.avx2.gather.q.q"]
        pub fn avx2_gather_q_q(a: ::simdty::i64x2, b: *mut i8, c: ::simdty::i64x2, d: ::simdty::i64x2, e: i8) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.avx2.gather.q.q.256"]
        pub fn avx2_gather_q_q_256(a: ::simdty::i64x4, b: *mut i8, c: ::simdty::i64x4, d: ::simdty::i64x4, e: i8) -> ::simdty::i64x4;
        #[link_name = "llvm.x86.avx2.maskload.d"]
        pub fn avx2_maskload_d(a: *mut i8, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.avx2.maskload.d.256"]
        pub fn avx2_maskload_d_256(a: *mut i8, b: ::simdty::i32x8) -> ::simdty::i32x8;
        #[link_name = "llvm.x86.avx2.maskload.q"]
        pub fn avx2_maskload_q(a: *mut i8, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.avx2.maskload.q.256"]
        pub fn avx2_maskload_q_256(a: *mut i8, b: ::simdty::i64x4) -> ::simdty::i64x4;
        #[link_name = "llvm.x86.avx2.maskstore.d"]
        pub fn avx2_maskstore_d(a: *mut i8, b: ::simdty::i32x4, c: ::simdty::i32x4) -> ();
        #[link_name = "llvm.x86.avx2.maskstore.d.256"]
        pub fn avx2_maskstore_d_256(a: *mut i8, b: ::simdty::i32x8, c: ::simdty::i32x8) -> ();
        #[link_name = "llvm.x86.avx2.maskstore.q"]
        pub fn avx2_maskstore_q(a: *mut i8, b: ::simdty::i64x2, c: ::simdty::i64x2) -> ();
        #[link_name = "llvm.x86.avx2.maskstore.q.256"]
        pub fn avx2_maskstore_q_256(a: *mut i8, b: ::simdty::i64x4, c: ::simdty::i64x4) -> ();
        #[link_name = "llvm.x86.avx2.movntdqa"]
        pub fn avx2_movntdqa(a: *mut i8) -> ::simdty::i64x4;
        #[link_name = "llvm.x86.avx2.mpsadbw"]
        pub fn avx2_mpsadbw(a: ::simdty::i8x32, b: ::simdty::i8x32, c: i8) -> ::simdty::i16x16;
        #[link_name = "llvm.x86.avx2.pabs.b"]
        pub fn avx2_pabs_b(a: ::simdty::i8x32) -> ::simdty::i8x32;
        #[link_name = "llvm.x86.avx2.pabs.d"]
        pub fn avx2_pabs_d(a: ::simdty::i32x8) -> ::simdty::i32x8;
        #[link_name = "llvm.x86.avx2.pabs.w"]
        pub fn avx2_pabs_w(a: ::simdty::i16x16) -> ::simdty::i16x16;
        #[link_name = "llvm.x86.avx2.packssdw"]
        pub fn avx2_packssdw(a: ::simdty::i32x8, b: ::simdty::i32x8) -> ::simdty::i16x16;
        #[link_name = "llvm.x86.avx2.packsswb"]
        pub fn avx2_packsswb(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i8x32;
        #[link_name = "llvm.x86.avx2.packusdw"]
        pub fn avx2_packusdw(a: ::simdty::i32x8, b: ::simdty::i32x8) -> ::simdty::i16x16;
        #[link_name = "llvm.x86.avx2.packuswb"]
        pub fn avx2_packuswb(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i8x32;
        #[link_name = "llvm.x86.avx2.padds.b"]
        pub fn avx2_padds_b(a: ::simdty::i8x32, b: ::simdty::i8x32) -> ::simdty::i8x32;
        #[link_name = "llvm.x86.avx2.padds.w"]
        pub fn avx2_padds_w(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i16x16;
        #[link_name = "llvm.x86.avx2.paddus.b"]
        pub fn avx2_paddus_b(a: ::simdty::i8x32, b: ::simdty::i8x32) -> ::simdty::i8x32;
        #[link_name = "llvm.x86.avx2.paddus.w"]
        pub fn avx2_paddus_w(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i16x16;
        #[link_name = "llvm.x86.avx2.pavg.b"]
        pub fn avx2_pavg_b(a: ::simdty::i8x32, b: ::simdty::i8x32) -> ::simdty::i8x32;
        #[link_name = "llvm.x86.avx2.pavg.w"]
        pub fn avx2_pavg_w(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i16x16;
        #[link_name = "llvm.x86.avx2.pblendd.128"]
        pub fn avx2_pblendd_128(a: ::simdty::i32x4, b: ::simdty::i32x4, c: i8) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.avx2.pblendd.256"]
        pub fn avx2_pblendd_256(a: ::simdty::i32x8, b: ::simdty::i32x8, c: i8) -> ::simdty::i32x8;
        #[link_name = "llvm.x86.avx2.pblendvb"]
        pub fn avx2_pblendvb(a: ::simdty::i8x32, b: ::simdty::i8x32, c: ::simdty::i8x32) -> ::simdty::i8x32;
        #[link_name = "llvm.x86.avx2.pblendw"]
        pub fn avx2_pblendw(a: ::simdty::i16x16, b: ::simdty::i16x16, c: i8) -> ::simdty::i16x16;
        #[link_name = "llvm.x86.avx2.pbroadcastb.128"]
        pub fn avx2_pbroadcastb_128(a: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.x86.avx2.pbroadcastb.256"]
        pub fn avx2_pbroadcastb_256(a: ::simdty::i8x16) -> ::simdty::i8x32;
        #[link_name = "llvm.x86.avx2.pbroadcastd.128"]
        pub fn avx2_pbroadcastd_128(a: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.avx2.pbroadcastd.256"]
        pub fn avx2_pbroadcastd_256(a: ::simdty::i32x4) -> ::simdty::i32x8;
        #[link_name = "llvm.x86.avx2.pbroadcastq.128"]
        pub fn avx2_pbroadcastq_128(a: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.avx2.pbroadcastq.256"]
        pub fn avx2_pbroadcastq_256(a: ::simdty::i64x2) -> ::simdty::i64x4;
        #[link_name = "llvm.x86.avx2.pbroadcastw.128"]
        pub fn avx2_pbroadcastw_128(a: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.avx2.pbroadcastw.256"]
        pub fn avx2_pbroadcastw_256(a: ::simdty::i16x8) -> ::simdty::i16x16;
        #[link_name = "llvm.x86.avx2.permd"]
        pub fn avx2_permd(a: ::simdty::i32x8, b: ::simdty::i32x8) -> ::simdty::i32x8;
        #[link_name = "llvm.x86.avx2.permps"]
        pub fn avx2_permps(a: ::simdty::f32x8, b: ::simdty::f32x8) -> ::simdty::f32x8;
        #[link_name = "llvm.x86.avx2.phadd.d"]
        pub fn avx2_phadd_d(a: ::simdty::i32x8, b: ::simdty::i32x8) -> ::simdty::i32x8;
        #[link_name = "llvm.x86.avx2.phadd.sw"]
        pub fn avx2_phadd_sw(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i16x16;
        #[link_name = "llvm.x86.avx2.phadd.w"]
        pub fn avx2_phadd_w(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i16x16;
        #[link_name = "llvm.x86.avx2.phsub.d"]
        pub fn avx2_phsub_d(a: ::simdty::i32x8, b: ::simdty::i32x8) -> ::simdty::i32x8;
        #[link_name = "llvm.x86.avx2.phsub.sw"]
        pub fn avx2_phsub_sw(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i16x16;
        #[link_name = "llvm.x86.avx2.phsub.w"]
        pub fn avx2_phsub_w(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i16x16;
        #[link_name = "llvm.x86.avx2.pmadd.ub.sw"]
        pub fn avx2_pmadd_ub_sw(a: ::simdty::i8x32, b: ::simdty::i8x32) -> ::simdty::i16x16;
        #[link_name = "llvm.x86.avx2.pmadd.wd"]
        pub fn avx2_pmadd_wd(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i32x8;
        #[link_name = "llvm.x86.avx2.pmaxs.b"]
        pub fn avx2_pmaxs_b(a: ::simdty::i8x32, b: ::simdty::i8x32) -> ::simdty::i8x32;
        #[link_name = "llvm.x86.avx2.pmaxs.d"]
        pub fn avx2_pmaxs_d(a: ::simdty::i32x8, b: ::simdty::i32x8) -> ::simdty::i32x8;
        #[link_name = "llvm.x86.avx2.pmaxs.w"]
        pub fn avx2_pmaxs_w(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i16x16;
        #[link_name = "llvm.x86.avx2.pmaxu.b"]
        pub fn avx2_pmaxu_b(a: ::simdty::i8x32, b: ::simdty::i8x32) -> ::simdty::i8x32;
        #[link_name = "llvm.x86.avx2.pmaxu.d"]
        pub fn avx2_pmaxu_d(a: ::simdty::i32x8, b: ::simdty::i32x8) -> ::simdty::i32x8;
        #[link_name = "llvm.x86.avx2.pmaxu.w"]
        pub fn avx2_pmaxu_w(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i16x16;
        #[link_name = "llvm.x86.avx2.pmins.b"]
        pub fn avx2_pmins_b(a: ::simdty::i8x32, b: ::simdty::i8x32) -> ::simdty::i8x32;
        #[link_name = "llvm.x86.avx2.pmins.d"]
        pub fn avx2_pmins_d(a: ::simdty::i32x8, b: ::simdty::i32x8) -> ::simdty::i32x8;
        #[link_name = "llvm.x86.avx2.pmins.w"]
        pub fn avx2_pmins_w(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i16x16;
        #[link_name = "llvm.x86.avx2.pminu.b"]
        pub fn avx2_pminu_b(a: ::simdty::i8x32, b: ::simdty::i8x32) -> ::simdty::i8x32;
        #[link_name = "llvm.x86.avx2.pminu.d"]
        pub fn avx2_pminu_d(a: ::simdty::i32x8, b: ::simdty::i32x8) -> ::simdty::i32x8;
        #[link_name = "llvm.x86.avx2.pminu.w"]
        pub fn avx2_pminu_w(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i16x16;
        #[link_name = "llvm.x86.avx2.pmovmskb"]
        pub fn avx2_pmovmskb(a: ::simdty::i8x32) -> i32;
        #[link_name = "llvm.x86.avx2.pmovsxbd"]
        pub fn avx2_pmovsxbd(a: ::simdty::i8x16) -> ::simdty::i32x8;
        #[link_name = "llvm.x86.avx2.pmovsxbq"]
        pub fn avx2_pmovsxbq(a: ::simdty::i8x16) -> ::simdty::i64x4;
        #[link_name = "llvm.x86.avx2.pmovsxbw"]
        pub fn avx2_pmovsxbw(a: ::simdty::i8x16) -> ::simdty::i16x16;
        #[link_name = "llvm.x86.avx2.pmovsxdq"]
        pub fn avx2_pmovsxdq(a: ::simdty::i32x4) -> ::simdty::i64x4;
        #[link_name = "llvm.x86.avx2.pmovsxwd"]
        pub fn avx2_pmovsxwd(a: ::simdty::i16x8) -> ::simdty::i32x8;
        #[link_name = "llvm.x86.avx2.pmovsxwq"]
        pub fn avx2_pmovsxwq(a: ::simdty::i16x8) -> ::simdty::i64x4;
        #[link_name = "llvm.x86.avx2.pmovzxbd"]
        pub fn avx2_pmovzxbd(a: ::simdty::i8x16) -> ::simdty::i32x8;
        #[link_name = "llvm.x86.avx2.pmovzxbq"]
        pub fn avx2_pmovzxbq(a: ::simdty::i8x16) -> ::simdty::i64x4;
        #[link_name = "llvm.x86.avx2.pmovzxbw"]
        pub fn avx2_pmovzxbw(a: ::simdty::i8x16) -> ::simdty::i16x16;
        #[link_name = "llvm.x86.avx2.pmovzxdq"]
        pub fn avx2_pmovzxdq(a: ::simdty::i32x4) -> ::simdty::i64x4;
        #[link_name = "llvm.x86.avx2.pmovzxwd"]
        pub fn avx2_pmovzxwd(a: ::simdty::i16x8) -> ::simdty::i32x8;
        #[link_name = "llvm.x86.avx2.pmovzxwq"]
        pub fn avx2_pmovzxwq(a: ::simdty::i16x8) -> ::simdty::i64x4;
        #[link_name = "llvm.x86.avx2.pmul.dq"]
        pub fn avx2_pmul_dq(a: ::simdty::i32x8, b: ::simdty::i32x8) -> ::simdty::i64x4;
        #[link_name = "llvm.x86.avx2.pmul.hr.sw"]
        pub fn avx2_pmul_hr_sw(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i16x16;
        #[link_name = "llvm.x86.avx2.pmulh.w"]
        pub fn avx2_pmulh_w(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i16x16;
        #[link_name = "llvm.x86.avx2.pmulhu.w"]
        pub fn avx2_pmulhu_w(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i16x16;
        #[link_name = "llvm.x86.avx2.pmulu.dq"]
        pub fn avx2_pmulu_dq(a: ::simdty::i32x8, b: ::simdty::i32x8) -> ::simdty::i64x4;
        #[link_name = "llvm.x86.avx2.psad.bw"]
        pub fn avx2_psad_bw(a: ::simdty::i8x32, b: ::simdty::i8x32) -> ::simdty::i64x4;
        #[link_name = "llvm.x86.avx2.pshuf.b"]
        pub fn avx2_pshuf_b(a: ::simdty::i8x32, b: ::simdty::i8x32) -> ::simdty::i8x32;
        #[link_name = "llvm.x86.avx2.psign.b"]
        pub fn avx2_psign_b(a: ::simdty::i8x32, b: ::simdty::i8x32) -> ::simdty::i8x32;
        #[link_name = "llvm.x86.avx2.psign.d"]
        pub fn avx2_psign_d(a: ::simdty::i32x8, b: ::simdty::i32x8) -> ::simdty::i32x8;
        #[link_name = "llvm.x86.avx2.psign.w"]
        pub fn avx2_psign_w(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i16x16;
        #[link_name = "llvm.x86.avx2.psll.d"]
        pub fn avx2_psll_d(a: ::simdty::i32x8, b: ::simdty::i32x4) -> ::simdty::i32x8;
        #[link_name = "llvm.x86.avx2.psll.dq"]
        pub fn avx2_psll_dq(a: ::simdty::i64x4, b: i32) -> ::simdty::i64x4;
        #[link_name = "llvm.x86.avx2.psll.dq.bs"]
        pub fn avx2_psll_dq_bs(a: ::simdty::i64x4, b: i32) -> ::simdty::i64x4;
        #[link_name = "llvm.x86.avx2.psll.q"]
        pub fn avx2_psll_q(a: ::simdty::i64x4, b: ::simdty::i64x2) -> ::simdty::i64x4;
        #[link_name = "llvm.x86.avx2.psll.w"]
        pub fn avx2_psll_w(a: ::simdty::i16x16, b: ::simdty::i16x8) -> ::simdty::i16x16;
        #[link_name = "llvm.x86.avx2.pslli.d"]
        pub fn avx2_pslli_d(a: ::simdty::i32x8, b: i32) -> ::simdty::i32x8;
        #[link_name = "llvm.x86.avx2.pslli.q"]
        pub fn avx2_pslli_q(a: ::simdty::i64x4, b: i32) -> ::simdty::i64x4;
        #[link_name = "llvm.x86.avx2.pslli.w"]
        pub fn avx2_pslli_w(a: ::simdty::i16x16, b: i32) -> ::simdty::i16x16;
        #[link_name = "llvm.x86.avx2.psllv.d"]
        pub fn avx2_psllv_d(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.avx2.psllv.d.256"]
        pub fn avx2_psllv_d_256(a: ::simdty::i32x8, b: ::simdty::i32x8) -> ::simdty::i32x8;
        #[link_name = "llvm.x86.avx2.psllv.q"]
        pub fn avx2_psllv_q(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.avx2.psllv.q.256"]
        pub fn avx2_psllv_q_256(a: ::simdty::i64x4, b: ::simdty::i64x4) -> ::simdty::i64x4;
        #[link_name = "llvm.x86.avx2.psra.d"]
        pub fn avx2_psra_d(a: ::simdty::i32x8, b: ::simdty::i32x4) -> ::simdty::i32x8;
        #[link_name = "llvm.x86.avx2.psra.w"]
        pub fn avx2_psra_w(a: ::simdty::i16x16, b: ::simdty::i16x8) -> ::simdty::i16x16;
        #[link_name = "llvm.x86.avx2.psrai.d"]
        pub fn avx2_psrai_d(a: ::simdty::i32x8, b: i32) -> ::simdty::i32x8;
        #[link_name = "llvm.x86.avx2.psrai.w"]
        pub fn avx2_psrai_w(a: ::simdty::i16x16, b: i32) -> ::simdty::i16x16;
        #[link_name = "llvm.x86.avx2.psrav.d"]
        pub fn avx2_psrav_d(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.avx2.psrav.d.256"]
        pub fn avx2_psrav_d_256(a: ::simdty::i32x8, b: ::simdty::i32x8) -> ::simdty::i32x8;
        #[link_name = "llvm.x86.avx2.psrl.d"]
        pub fn avx2_psrl_d(a: ::simdty::i32x8, b: ::simdty::i32x4) -> ::simdty::i32x8;
        #[link_name = "llvm.x86.avx2.psrl.dq"]
        pub fn avx2_psrl_dq(a: ::simdty::i64x4, b: i32) -> ::simdty::i64x4;
        #[link_name = "llvm.x86.avx2.psrl.dq.bs"]
        pub fn avx2_psrl_dq_bs(a: ::simdty::i64x4, b: i32) -> ::simdty::i64x4;
        #[link_name = "llvm.x86.avx2.psrl.q"]
        pub fn avx2_psrl_q(a: ::simdty::i64x4, b: ::simdty::i64x2) -> ::simdty::i64x4;
        #[link_name = "llvm.x86.avx2.psrl.w"]
        pub fn avx2_psrl_w(a: ::simdty::i16x16, b: ::simdty::i16x8) -> ::simdty::i16x16;
        #[link_name = "llvm.x86.avx2.psrli.d"]
        pub fn avx2_psrli_d(a: ::simdty::i32x8, b: i32) -> ::simdty::i32x8;
        #[link_name = "llvm.x86.avx2.psrli.q"]
        pub fn avx2_psrli_q(a: ::simdty::i64x4, b: i32) -> ::simdty::i64x4;
        #[link_name = "llvm.x86.avx2.psrli.w"]
        pub fn avx2_psrli_w(a: ::simdty::i16x16, b: i32) -> ::simdty::i16x16;
        #[link_name = "llvm.x86.avx2.psrlv.d"]
        pub fn avx2_psrlv_d(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.avx2.psrlv.d.256"]
        pub fn avx2_psrlv_d_256(a: ::simdty::i32x8, b: ::simdty::i32x8) -> ::simdty::i32x8;
        #[link_name = "llvm.x86.avx2.psrlv.q"]
        pub fn avx2_psrlv_q(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.avx2.psrlv.q.256"]
        pub fn avx2_psrlv_q_256(a: ::simdty::i64x4, b: ::simdty::i64x4) -> ::simdty::i64x4;
        #[link_name = "llvm.x86.avx2.psubs.b"]
        pub fn avx2_psubs_b(a: ::simdty::i8x32, b: ::simdty::i8x32) -> ::simdty::i8x32;
        #[link_name = "llvm.x86.avx2.psubs.w"]
        pub fn avx2_psubs_w(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i16x16;
        #[link_name = "llvm.x86.avx2.psubus.b"]
        pub fn avx2_psubus_b(a: ::simdty::i8x32, b: ::simdty::i8x32) -> ::simdty::i8x32;
        #[link_name = "llvm.x86.avx2.psubus.w"]
        pub fn avx2_psubus_w(a: ::simdty::i16x16, b: ::simdty::i16x16) -> ::simdty::i16x16;
        #[link_name = "llvm.x86.avx2.vbroadcast.sd.pd.256"]
        pub fn avx2_vbroadcast_sd_pd_256(a: ::simdty::f64x2) -> ::simdty::f64x4;
        #[link_name = "llvm.x86.avx2.vbroadcast.ss.ps"]
        pub fn avx2_vbroadcast_ss_ps(a: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.avx2.vbroadcast.ss.ps.256"]
        pub fn avx2_vbroadcast_ss_ps_256(a: ::simdty::f32x4) -> ::simdty::f32x8;
        #[link_name = "llvm.x86.avx2.vbroadcasti128"]
        pub fn avx2_vbroadcasti128(a: *mut i8) -> ::simdty::i64x4;
        #[link_name = "llvm.x86.avx2.vextracti128"]
        pub fn avx2_vextracti128(a: ::simdty::i64x4, b: i8) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.avx2.vinserti128"]
        pub fn avx2_vinserti128(a: ::simdty::i64x4, b: ::simdty::i64x2, c: i8) -> ::simdty::i64x4;
        #[link_name = "llvm.x86.avx2.vperm2i128"]
        pub fn avx2_vperm2i128(a: ::simdty::i64x4, b: ::simdty::i64x4, c: i8) -> ::simdty::i64x4;
        #[link_name = "llvm.x86.avx512.cvtsd2usi"]
        pub fn avx512_cvtsd2usi(a: ::simdty::f64x2) -> i32;
        #[link_name = "llvm.x86.avx512.cvtsd2usi64"]
        pub fn avx512_cvtsd2usi64(a: ::simdty::f64x2) -> i64;
        #[link_name = "llvm.x86.avx512.cvtss2usi"]
        pub fn avx512_cvtss2usi(a: ::simdty::f32x4) -> i32;
        #[link_name = "llvm.x86.avx512.cvtss2usi64"]
        pub fn avx512_cvtss2usi64(a: ::simdty::f32x4) -> i64;
        #[link_name = "llvm.x86.avx512.cvttsd2usi"]
        pub fn avx512_cvttsd2usi(a: ::simdty::f64x2) -> i32;
        #[link_name = "llvm.x86.avx512.cvttsd2usi64"]
        pub fn avx512_cvttsd2usi64(a: ::simdty::f64x2) -> i64;
        #[link_name = "llvm.x86.avx512.cvttss2usi"]
        pub fn avx512_cvttss2usi(a: ::simdty::f32x4) -> i32;
        #[link_name = "llvm.x86.avx512.cvttss2usi64"]
        pub fn avx512_cvttss2usi64(a: ::simdty::f32x4) -> i64;
        #[link_name = "llvm.x86.avx512.cvtusi2sd"]
        pub fn avx512_cvtusi2sd(a: ::simdty::f64x2, b: i32) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.avx512.cvtusi2ss"]
        pub fn avx512_cvtusi2ss(a: ::simdty::f32x4, b: i32) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.avx512.cvtusi642sd"]
        pub fn avx512_cvtusi642sd(a: ::simdty::f64x2, b: i64) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.avx512.cvtusi642ss"]
        pub fn avx512_cvtusi642ss(a: ::simdty::f32x4, b: i64) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.avx512.gather.dpd.512"]
        pub fn avx512_gather_dpd_512(a: ::simdty::f64x8, b: *mut i8, c: ::simdty::i32x8, d: i8, e: i32) -> ::simdty::f64x8;
        #[link_name = "llvm.x86.avx512.gather.dpi.512"]
        pub fn avx512_gather_dpi_512(a: ::simdty::i32x16, b: *mut i8, c: ::simdty::i32x16, d: i16, e: i32) -> ::simdty::i32x16;
        #[link_name = "llvm.x86.avx512.gather.dpq.512"]
        pub fn avx512_gather_dpq_512(a: ::simdty::i64x8, b: *mut i8, c: ::simdty::i32x8, d: i8, e: i32) -> ::simdty::i64x8;
        #[link_name = "llvm.x86.avx512.gather.dps.512"]
        pub fn avx512_gather_dps_512(a: ::simdty::f32x16, b: *mut i8, c: ::simdty::i32x16, d: i16, e: i32) -> ::simdty::f32x16;
        #[link_name = "llvm.x86.avx512.gather.qpd.512"]
        pub fn avx512_gather_qpd_512(a: ::simdty::f64x8, b: *mut i8, c: ::simdty::i64x8, d: i8, e: i32) -> ::simdty::f64x8;
        #[link_name = "llvm.x86.avx512.gather.qpi.512"]
        pub fn avx512_gather_qpi_512(a: ::simdty::i32x8, b: *mut i8, c: ::simdty::i64x8, d: i8, e: i32) -> ::simdty::i32x8;
        #[link_name = "llvm.x86.avx512.gather.qpq.512"]
        pub fn avx512_gather_qpq_512(a: ::simdty::i64x8, b: *mut i8, c: ::simdty::i64x8, d: i8, e: i32) -> ::simdty::i64x8;
        #[link_name = "llvm.x86.avx512.gather.qps.512"]
        pub fn avx512_gather_qps_512(a: ::simdty::f32x8, b: *mut i8, c: ::simdty::i64x8, d: i8, e: i32) -> ::simdty::f32x8;
        #[link_name = "llvm.x86.avx512.gatherpf.dpd.512"]
        pub fn avx512_gatherpf_dpd_512(a: i8, b: ::simdty::i32x8, c: *mut i8, d: i32, e: i32) -> ();
        #[link_name = "llvm.x86.avx512.gatherpf.dps.512"]
        pub fn avx512_gatherpf_dps_512(a: i16, b: ::simdty::i32x16, c: *mut i8, d: i32, e: i32) -> ();
        #[link_name = "llvm.x86.avx512.gatherpf.qpd.512"]
        pub fn avx512_gatherpf_qpd_512(a: i8, b: ::simdty::i64x8, c: *mut i8, d: i32, e: i32) -> ();
        #[link_name = "llvm.x86.avx512.gatherpf.qps.512"]
        pub fn avx512_gatherpf_qps_512(a: i8, b: ::simdty::i64x8, c: *mut i8, d: i32, e: i32) -> ();
        #[link_name = "llvm.x86.avx512.kand.w"]
        pub fn avx512_kand_w(a: i16, b: i16) -> i16;
        #[link_name = "llvm.x86.avx512.kandn.w"]
        pub fn avx512_kandn_w(a: i16, b: i16) -> i16;
        #[link_name = "llvm.x86.avx512.knot.w"]
        pub fn avx512_knot_w(a: i16) -> i16;
        #[link_name = "llvm.x86.avx512.kor.w"]
        pub fn avx512_kor_w(a: i16, b: i16) -> i16;
        #[link_name = "llvm.x86.avx512.kortestc.w"]
        pub fn avx512_kortestc_w(a: i16, b: i16) -> i32;
        #[link_name = "llvm.x86.avx512.kortestz.w"]
        pub fn avx512_kortestz_w(a: i16, b: i16) -> i32;
        #[link_name = "llvm.x86.avx512.kunpck.bw"]
        pub fn avx512_kunpck_bw(a: i16, b: i16) -> i16;
        #[link_name = "llvm.x86.avx512.kxnor.w"]
        pub fn avx512_kxnor_w(a: i16, b: i16) -> i16;
        #[link_name = "llvm.x86.avx512.kxor.w"]
        pub fn avx512_kxor_w(a: i16, b: i16) -> i16;
        #[link_name = "llvm.x86.avx512.mask.blend.d.512"]
        pub fn avx512_mask_blend_d_512(a: ::simdty::i32x16, b: ::simdty::i32x16, c: i16) -> ::simdty::i32x16;
        #[link_name = "llvm.x86.avx512.mask.blend.pd.512"]
        pub fn avx512_mask_blend_pd_512(a: ::simdty::f64x8, b: ::simdty::f64x8, c: i8) -> ::simdty::f64x8;
        #[link_name = "llvm.x86.avx512.mask.blend.ps.512"]
        pub fn avx512_mask_blend_ps_512(a: ::simdty::f32x16, b: ::simdty::f32x16, c: i16) -> ::simdty::f32x16;
        #[link_name = "llvm.x86.avx512.mask.blend.q.512"]
        pub fn avx512_mask_blend_q_512(a: ::simdty::i64x8, b: ::simdty::i64x8, c: i8) -> ::simdty::i64x8;
        #[link_name = "llvm.x86.avx512.mask.cmp.pd.512"]
        pub fn avx512_mask_cmp_pd_512(a: ::simdty::f64x8, b: ::simdty::f64x8, c: i32, d: i8, e: i32) -> i8;
        #[link_name = "llvm.x86.avx512.mask.cmp.ps.512"]
        pub fn avx512_mask_cmp_ps_512(a: ::simdty::f32x16, b: ::simdty::f32x16, c: i32, d: i16, e: i32) -> i16;
        #[link_name = "llvm.x86.avx512.mask.conflict.d.512"]
        pub fn avx512_mask_conflict_d_512(a: ::simdty::i32x16, b: ::simdty::i32x16, c: i16) -> ::simdty::i32x16;
        #[link_name = "llvm.x86.avx512.mask.conflict.q.512"]
        pub fn avx512_mask_conflict_q_512(a: ::simdty::i64x8, b: ::simdty::i64x8, c: i8) -> ::simdty::i64x8;
        #[link_name = "llvm.x86.avx512.mask.cvtdq2pd.512"]
        pub fn avx512_mask_cvtdq2pd_512(a: ::simdty::i32x8, b: ::simdty::f64x8, c: i8) -> ::simdty::f64x8;
        #[link_name = "llvm.x86.avx512.mask.cvtdq2ps.512"]
        pub fn avx512_mask_cvtdq2ps_512(a: ::simdty::i32x16, b: ::simdty::f32x16, c: i16, d: i32) -> ::simdty::f32x16;
        #[link_name = "llvm.x86.avx512.mask.cvtpd2dq.512"]
        pub fn avx512_mask_cvtpd2dq_512(a: ::simdty::f64x8, b: ::simdty::i32x8, c: i8, d: i32) -> ::simdty::i32x8;
        #[link_name = "llvm.x86.avx512.mask.cvtpd2ps.512"]
        pub fn avx512_mask_cvtpd2ps_512(a: ::simdty::f64x8, b: ::simdty::f32x8, c: i8, d: i32) -> ::simdty::f32x8;
        #[link_name = "llvm.x86.avx512.mask.cvtpd2udq.512"]
        pub fn avx512_mask_cvtpd2udq_512(a: ::simdty::f64x8, b: ::simdty::i32x8, c: i8, d: i32) -> ::simdty::i32x8;
        #[link_name = "llvm.x86.avx512.mask.cvtps2dq.512"]
        pub fn avx512_mask_cvtps2dq_512(a: ::simdty::f32x16, b: ::simdty::i32x16, c: i16, d: i32) -> ::simdty::i32x16;
        #[link_name = "llvm.x86.avx512.mask.cvtps2udq.512"]
        pub fn avx512_mask_cvtps2udq_512(a: ::simdty::f32x16, b: ::simdty::i32x16, c: i16, d: i32) -> ::simdty::i32x16;
        #[link_name = "llvm.x86.avx512.mask.cvttpd2dq.512"]
        pub fn avx512_mask_cvttpd2dq_512(a: ::simdty::f64x8, b: ::simdty::i32x8, c: i8, d: i32) -> ::simdty::i32x8;
        #[link_name = "llvm.x86.avx512.mask.cvttpd2udq.512"]
        pub fn avx512_mask_cvttpd2udq_512(a: ::simdty::f64x8, b: ::simdty::i32x8, c: i8, d: i32) -> ::simdty::i32x8;
        #[link_name = "llvm.x86.avx512.mask.cvttps2dq.512"]
        pub fn avx512_mask_cvttps2dq_512(a: ::simdty::f32x16, b: ::simdty::i32x16, c: i16, d: i32) -> ::simdty::i32x16;
        #[link_name = "llvm.x86.avx512.mask.cvttps2udq.512"]
        pub fn avx512_mask_cvttps2udq_512(a: ::simdty::f32x16, b: ::simdty::i32x16, c: i16, d: i32) -> ::simdty::i32x16;
        #[link_name = "llvm.x86.avx512.mask.cvtudq2pd.512"]
        pub fn avx512_mask_cvtudq2pd_512(a: ::simdty::i32x8, b: ::simdty::f64x8, c: i8) -> ::simdty::f64x8;
        #[link_name = "llvm.x86.avx512.mask.cvtudq2ps.512"]
        pub fn avx512_mask_cvtudq2ps_512(a: ::simdty::i32x16, b: ::simdty::f32x16, c: i16, d: i32) -> ::simdty::f32x16;
        #[link_name = "llvm.x86.avx512.mask.loadu.d.512"]
        pub fn avx512_mask_loadu_d_512(a: *mut i8, b: ::simdty::i32x16, c: i16) -> ::simdty::i32x16;
        #[link_name = "llvm.x86.avx512.mask.loadu.pd.512"]
        pub fn avx512_mask_loadu_pd_512(a: *mut i8, b: ::simdty::f64x8, c: i8) -> ::simdty::f64x8;
        #[link_name = "llvm.x86.avx512.mask.loadu.ps.512"]
        pub fn avx512_mask_loadu_ps_512(a: *mut i8, b: ::simdty::f32x16, c: i16) -> ::simdty::f32x16;
        #[link_name = "llvm.x86.avx512.mask.loadu.q.512"]
        pub fn avx512_mask_loadu_q_512(a: *mut i8, b: ::simdty::i64x8, c: i8) -> ::simdty::i64x8;
        #[link_name = "llvm.x86.avx512.mask.lzcnt.d.512"]
        pub fn avx512_mask_lzcnt_d_512(a: ::simdty::i32x16, b: ::simdty::i32x16, c: i16) -> ::simdty::i32x16;
        #[link_name = "llvm.x86.avx512.mask.lzcnt.q.512"]
        pub fn avx512_mask_lzcnt_q_512(a: ::simdty::i64x8, b: ::simdty::i64x8, c: i8) -> ::simdty::i64x8;
        #[link_name = "llvm.x86.avx512.mask.max.pd.512"]
        pub fn avx512_mask_max_pd_512(a: ::simdty::f64x8, b: ::simdty::f64x8, c: ::simdty::f64x8, d: i8, e: i32) -> ::simdty::f64x8;
        #[link_name = "llvm.x86.avx512.mask.max.ps.512"]
        pub fn avx512_mask_max_ps_512(a: ::simdty::f32x16, b: ::simdty::f32x16, c: ::simdty::f32x16, d: i16, e: i32) -> ::simdty::f32x16;
        #[link_name = "llvm.x86.avx512.mask.min.pd.512"]
        pub fn avx512_mask_min_pd_512(a: ::simdty::f64x8, b: ::simdty::f64x8, c: ::simdty::f64x8, d: i8, e: i32) -> ::simdty::f64x8;
        #[link_name = "llvm.x86.avx512.mask.min.ps.512"]
        pub fn avx512_mask_min_ps_512(a: ::simdty::f32x16, b: ::simdty::f32x16, c: ::simdty::f32x16, d: i16, e: i32) -> ::simdty::f32x16;
        #[link_name = "llvm.x86.avx512.mask.pabs.d.512"]
        pub fn avx512_mask_pabs_d_512(a: ::simdty::i32x16, b: ::simdty::i32x16, c: i16) -> ::simdty::i32x16;
        #[link_name = "llvm.x86.avx512.mask.pabs.q.512"]
        pub fn avx512_mask_pabs_q_512(a: ::simdty::i64x8, b: ::simdty::i64x8, c: i8) -> ::simdty::i64x8;
        #[link_name = "llvm.x86.avx512.mask.pand.d.512"]
        pub fn avx512_mask_pand_d_512(a: ::simdty::i32x16, b: ::simdty::i32x16, c: ::simdty::i32x16, d: i16) -> ::simdty::i32x16;
        #[link_name = "llvm.x86.avx512.mask.pand.q.512"]
        pub fn avx512_mask_pand_q_512(a: ::simdty::i64x8, b: ::simdty::i64x8, c: ::simdty::i64x8, d: i8) -> ::simdty::i64x8;
        #[link_name = "llvm.x86.avx512.mask.pbroadcast.d.gpr.512"]
        pub fn avx512_mask_pbroadcast_d_gpr_512(a: i32, b: ::simdty::i32x16, c: i16) -> ::simdty::i32x16;
        #[link_name = "llvm.x86.avx512.mask.pbroadcast.q.gpr.512"]
        pub fn avx512_mask_pbroadcast_q_gpr_512(a: i64, b: ::simdty::i64x8, c: i8) -> ::simdty::i64x8;
        #[link_name = "llvm.x86.avx512.mask.pbroadcast.q.mem.512"]
        pub fn avx512_mask_pbroadcast_q_mem_512(a: i64, b: ::simdty::i64x8, c: i8) -> ::simdty::i64x8;
        #[link_name = "llvm.x86.avx512.mask.pcmpeq.b.128"]
        pub fn avx512_mask_pcmpeq_b_128(a: ::simdty::i8x16, b: ::simdty::i8x16, c: i16) -> i16;
        #[link_name = "llvm.x86.avx512.mask.pcmpeq.b.256"]
        pub fn avx512_mask_pcmpeq_b_256(a: ::simdty::i8x32, b: ::simdty::i8x32, c: i32) -> i32;
        #[link_name = "llvm.x86.avx512.mask.pcmpeq.b.512"]
        pub fn avx512_mask_pcmpeq_b_512(a: ::simdty::i8x64, b: ::simdty::i8x64, c: i64) -> i64;
        #[link_name = "llvm.x86.avx512.mask.pcmpeq.d.128"]
        pub fn avx512_mask_pcmpeq_d_128(a: ::simdty::i32x4, b: ::simdty::i32x4, c: i8) -> i8;
        #[link_name = "llvm.x86.avx512.mask.pcmpeq.d.256"]
        pub fn avx512_mask_pcmpeq_d_256(a: ::simdty::i32x8, b: ::simdty::i32x8, c: i8) -> i8;
        #[link_name = "llvm.x86.avx512.mask.pcmpeq.d.512"]
        pub fn avx512_mask_pcmpeq_d_512(a: ::simdty::i32x16, b: ::simdty::i32x16, c: i16) -> i16;
        #[link_name = "llvm.x86.avx512.mask.pcmpeq.q.128"]
        pub fn avx512_mask_pcmpeq_q_128(a: ::simdty::i64x2, b: ::simdty::i64x2, c: i8) -> i8;
        #[link_name = "llvm.x86.avx512.mask.pcmpeq.q.256"]
        pub fn avx512_mask_pcmpeq_q_256(a: ::simdty::i64x4, b: ::simdty::i64x4, c: i8) -> i8;
        #[link_name = "llvm.x86.avx512.mask.pcmpeq.q.512"]
        pub fn avx512_mask_pcmpeq_q_512(a: ::simdty::i64x8, b: ::simdty::i64x8, c: i8) -> i8;
        #[link_name = "llvm.x86.avx512.mask.pcmpeq.w.128"]
        pub fn avx512_mask_pcmpeq_w_128(a: ::simdty::i16x8, b: ::simdty::i16x8, c: i8) -> i8;
        #[link_name = "llvm.x86.avx512.mask.pcmpeq.w.256"]
        pub fn avx512_mask_pcmpeq_w_256(a: ::simdty::i16x16, b: ::simdty::i16x16, c: i16) -> i16;
        #[link_name = "llvm.x86.avx512.mask.pcmpeq.w.512"]
        pub fn avx512_mask_pcmpeq_w_512(a: ::simdty::i16x32, b: ::simdty::i16x32, c: i32) -> i32;
        #[link_name = "llvm.x86.avx512.mask.pcmpgt.b.128"]
        pub fn avx512_mask_pcmpgt_b_128(a: ::simdty::i8x16, b: ::simdty::i8x16, c: i16) -> i16;
        #[link_name = "llvm.x86.avx512.mask.pcmpgt.b.256"]
        pub fn avx512_mask_pcmpgt_b_256(a: ::simdty::i8x32, b: ::simdty::i8x32, c: i32) -> i32;
        #[link_name = "llvm.x86.avx512.mask.pcmpgt.b.512"]
        pub fn avx512_mask_pcmpgt_b_512(a: ::simdty::i8x64, b: ::simdty::i8x64, c: i64) -> i64;
        #[link_name = "llvm.x86.avx512.mask.pcmpgt.d.128"]
        pub fn avx512_mask_pcmpgt_d_128(a: ::simdty::i32x4, b: ::simdty::i32x4, c: i8) -> i8;
        #[link_name = "llvm.x86.avx512.mask.pcmpgt.d.256"]
        pub fn avx512_mask_pcmpgt_d_256(a: ::simdty::i32x8, b: ::simdty::i32x8, c: i8) -> i8;
        #[link_name = "llvm.x86.avx512.mask.pcmpgt.d.512"]
        pub fn avx512_mask_pcmpgt_d_512(a: ::simdty::i32x16, b: ::simdty::i32x16, c: i16) -> i16;
        #[link_name = "llvm.x86.avx512.mask.pcmpgt.q.128"]
        pub fn avx512_mask_pcmpgt_q_128(a: ::simdty::i64x2, b: ::simdty::i64x2, c: i8) -> i8;
        #[link_name = "llvm.x86.avx512.mask.pcmpgt.q.256"]
        pub fn avx512_mask_pcmpgt_q_256(a: ::simdty::i64x4, b: ::simdty::i64x4, c: i8) -> i8;
        #[link_name = "llvm.x86.avx512.mask.pcmpgt.q.512"]
        pub fn avx512_mask_pcmpgt_q_512(a: ::simdty::i64x8, b: ::simdty::i64x8, c: i8) -> i8;
        #[link_name = "llvm.x86.avx512.mask.pcmpgt.w.128"]
        pub fn avx512_mask_pcmpgt_w_128(a: ::simdty::i16x8, b: ::simdty::i16x8, c: i8) -> i8;
        #[link_name = "llvm.x86.avx512.mask.pcmpgt.w.256"]
        pub fn avx512_mask_pcmpgt_w_256(a: ::simdty::i16x16, b: ::simdty::i16x16, c: i16) -> i16;
        #[link_name = "llvm.x86.avx512.mask.pcmpgt.w.512"]
        pub fn avx512_mask_pcmpgt_w_512(a: ::simdty::i16x32, b: ::simdty::i16x32, c: i32) -> i32;
        #[link_name = "llvm.x86.avx512.mask.pmaxs.d.512"]
        pub fn avx512_mask_pmaxs_d_512(a: ::simdty::i32x16, b: ::simdty::i32x16, c: ::simdty::i32x16, d: i16) -> ::simdty::i32x16;
        #[link_name = "llvm.x86.avx512.mask.pmaxs.q.512"]
        pub fn avx512_mask_pmaxs_q_512(a: ::simdty::i64x8, b: ::simdty::i64x8, c: ::simdty::i64x8, d: i8) -> ::simdty::i64x8;
        #[link_name = "llvm.x86.avx512.mask.pmaxu.d.512"]
        pub fn avx512_mask_pmaxu_d_512(a: ::simdty::i32x16, b: ::simdty::i32x16, c: ::simdty::i32x16, d: i16) -> ::simdty::i32x16;
        #[link_name = "llvm.x86.avx512.mask.pmaxu.q.512"]
        pub fn avx512_mask_pmaxu_q_512(a: ::simdty::i64x8, b: ::simdty::i64x8, c: ::simdty::i64x8, d: i8) -> ::simdty::i64x8;
        #[link_name = "llvm.x86.avx512.mask.pmins.d.512"]
        pub fn avx512_mask_pmins_d_512(a: ::simdty::i32x16, b: ::simdty::i32x16, c: ::simdty::i32x16, d: i16) -> ::simdty::i32x16;
        #[link_name = "llvm.x86.avx512.mask.pmins.q.512"]
        pub fn avx512_mask_pmins_q_512(a: ::simdty::i64x8, b: ::simdty::i64x8, c: ::simdty::i64x8, d: i8) -> ::simdty::i64x8;
        #[link_name = "llvm.x86.avx512.mask.pminu.d.512"]
        pub fn avx512_mask_pminu_d_512(a: ::simdty::i32x16, b: ::simdty::i32x16, c: ::simdty::i32x16, d: i16) -> ::simdty::i32x16;
        #[link_name = "llvm.x86.avx512.mask.pminu.q.512"]
        pub fn avx512_mask_pminu_q_512(a: ::simdty::i64x8, b: ::simdty::i64x8, c: ::simdty::i64x8, d: i8) -> ::simdty::i64x8;
        #[link_name = "llvm.x86.avx512.mask.pmul.dq.512"]
        pub fn avx512_mask_pmul_dq_512(a: ::simdty::i32x16, b: ::simdty::i32x16, c: ::simdty::i64x8, d: i8) -> ::simdty::i64x8;
        #[link_name = "llvm.x86.avx512.mask.pmulu.dq.512"]
        pub fn avx512_mask_pmulu_dq_512(a: ::simdty::i32x16, b: ::simdty::i32x16, c: ::simdty::i64x8, d: i8) -> ::simdty::i64x8;
        #[link_name = "llvm.x86.avx512.mask.ptestm.d.512"]
        pub fn avx512_mask_ptestm_d_512(a: ::simdty::i32x16, b: ::simdty::i32x16, c: i16) -> i16;
        #[link_name = "llvm.x86.avx512.mask.ptestm.q.512"]
        pub fn avx512_mask_ptestm_q_512(a: ::simdty::i64x8, b: ::simdty::i64x8, c: i8) -> i8;
        #[link_name = "llvm.x86.avx512.mask.rndscale.pd.512"]
        pub fn avx512_mask_rndscale_pd_512(a: ::simdty::f64x8, b: i32, c: ::simdty::f64x8, d: i8, e: i32) -> ::simdty::f64x8;
        #[link_name = "llvm.x86.avx512.mask.rndscale.ps.512"]
        pub fn avx512_mask_rndscale_ps_512(a: ::simdty::f32x16, b: i32, c: ::simdty::f32x16, d: i16, e: i32) -> ::simdty::f32x16;
        #[link_name = "llvm.x86.avx512.mask.store.ss"]
        pub fn avx512_mask_store_ss(a: *mut i8, b: ::simdty::f32x4, c: i8) -> ();
        #[link_name = "llvm.x86.avx512.mask.storeu.d.512"]
        pub fn avx512_mask_storeu_d_512(a: *mut i8, b: ::simdty::i32x16, c: i16) -> ();
        #[link_name = "llvm.x86.avx512.mask.storeu.pd.512"]
        pub fn avx512_mask_storeu_pd_512(a: *mut i8, b: ::simdty::f64x8, c: i8) -> ();
        #[link_name = "llvm.x86.avx512.mask.storeu.ps.512"]
        pub fn avx512_mask_storeu_ps_512(a: *mut i8, b: ::simdty::f32x16, c: i16) -> ();
        #[link_name = "llvm.x86.avx512.mask.storeu.q.512"]
        pub fn avx512_mask_storeu_q_512(a: *mut i8, b: ::simdty::i64x8, c: i8) -> ();
        #[link_name = "llvm.x86.avx512.mask.valign.d.512"]
        pub fn avx512_mask_valign_d_512(a: ::simdty::i32x16, b: ::simdty::i32x16, c: i8, d: ::simdty::i32x16, e: i16) -> ::simdty::i32x16;
        #[link_name = "llvm.x86.avx512.mask.valign.q.512"]
        pub fn avx512_mask_valign_q_512(a: ::simdty::i64x8, b: ::simdty::i64x8, c: i8, d: ::simdty::i64x8, e: i8) -> ::simdty::i64x8;
        #[link_name = "llvm.x86.avx512.mask.vcvtph2ps.512"]
        pub fn avx512_mask_vcvtph2ps_512(a: ::simdty::i16x16, b: ::simdty::f32x16, c: i16, d: i32) -> ::simdty::f32x16;
        #[link_name = "llvm.x86.avx512.mask.vcvtps2ph.512"]
        pub fn avx512_mask_vcvtps2ph_512(a: ::simdty::f32x16, b: i32, c: ::simdty::i16x16, d: i16) -> ::simdty::i16x16;
        #[link_name = "llvm.x86.avx512.mask.vpermt.d.512"]
        pub fn avx512_mask_vpermt_d_512(a: ::simdty::i32x16, b: ::simdty::i32x16, c: ::simdty::i32x16, d: i16) -> ::simdty::i32x16;
        #[link_name = "llvm.x86.avx512.mask.vpermt.pd.512"]
        pub fn avx512_mask_vpermt_pd_512(a: ::simdty::i64x8, b: ::simdty::f64x8, c: ::simdty::f64x8, d: i8) -> ::simdty::f64x8;
        #[link_name = "llvm.x86.avx512.mask.vpermt.ps.512"]
        pub fn avx512_mask_vpermt_ps_512(a: ::simdty::i32x16, b: ::simdty::f32x16, c: ::simdty::f32x16, d: i16) -> ::simdty::f32x16;
        #[link_name = "llvm.x86.avx512.mask.vpermt.q.512"]
        pub fn avx512_mask_vpermt_q_512(a: ::simdty::i64x8, b: ::simdty::i64x8, c: ::simdty::i64x8, d: i8) -> ::simdty::i64x8;
        #[link_name = "llvm.x86.avx512.movntdqa"]
        pub fn avx512_movntdqa(a: *mut i8) -> ::simdty::i64x8;
        #[link_name = "llvm.x86.avx512.pbroadcastd.512"]
        pub fn avx512_pbroadcastd_512(a: ::simdty::i32x4) -> ::simdty::i32x16;
        #[link_name = "llvm.x86.avx512.pbroadcastd.i32.512"]
        pub fn avx512_pbroadcastd_i32_512(a: i32) -> ::simdty::i32x16;
        #[link_name = "llvm.x86.avx512.pbroadcastq.512"]
        pub fn avx512_pbroadcastq_512(a: ::simdty::i64x2) -> ::simdty::i64x8;
        #[link_name = "llvm.x86.avx512.pbroadcastq.i64.512"]
        pub fn avx512_pbroadcastq_i64_512(a: i64) -> ::simdty::i64x8;
        #[link_name = "llvm.x86.avx512.pmovzxbd"]
        pub fn avx512_pmovzxbd(a: ::simdty::i8x16) -> ::simdty::i32x16;
        #[link_name = "llvm.x86.avx512.pmovzxbq"]
        pub fn avx512_pmovzxbq(a: ::simdty::i8x16) -> ::simdty::i64x8;
        #[link_name = "llvm.x86.avx512.pmovzxdq"]
        pub fn avx512_pmovzxdq(a: ::simdty::i32x8) -> ::simdty::i64x8;
        #[link_name = "llvm.x86.avx512.pmovzxwd"]
        pub fn avx512_pmovzxwd(a: ::simdty::i16x16) -> ::simdty::i32x16;
        #[link_name = "llvm.x86.avx512.pmovzxwq"]
        pub fn avx512_pmovzxwq(a: ::simdty::i16x8) -> ::simdty::i64x8;
        #[link_name = "llvm.x86.avx512.psll.dq"]
        pub fn avx512_psll_dq(a: ::simdty::i64x8, b: i32) -> ::simdty::i64x8;
        #[link_name = "llvm.x86.avx512.psll.dq.bs"]
        pub fn avx512_psll_dq_bs(a: ::simdty::i64x8, b: i32) -> ::simdty::i64x8;
        #[link_name = "llvm.x86.avx512.psrl.dq"]
        pub fn avx512_psrl_dq(a: ::simdty::i64x8, b: i32) -> ::simdty::i64x8;
        #[link_name = "llvm.x86.avx512.psrl.dq.bs"]
        pub fn avx512_psrl_dq_bs(a: ::simdty::i64x8, b: i32) -> ::simdty::i64x8;
        #[link_name = "llvm.x86.avx512.rcp14.pd.512"]
        pub fn avx512_rcp14_pd_512(a: ::simdty::f64x8, b: ::simdty::f64x8, c: i8) -> ::simdty::f64x8;
        #[link_name = "llvm.x86.avx512.rcp14.ps.512"]
        pub fn avx512_rcp14_ps_512(a: ::simdty::f32x16, b: ::simdty::f32x16, c: i16) -> ::simdty::f32x16;
        #[link_name = "llvm.x86.avx512.rcp14.sd"]
        pub fn avx512_rcp14_sd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: ::simdty::f64x2, d: i8) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.avx512.rcp14.ss"]
        pub fn avx512_rcp14_ss(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4, d: i8) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.avx512.rcp28.pd"]
        pub fn avx512_rcp28_pd(a: ::simdty::f64x8, b: ::simdty::f64x8, c: i8, d: i32) -> ::simdty::f64x8;
        #[link_name = "llvm.x86.avx512.rcp28.ps"]
        pub fn avx512_rcp28_ps(a: ::simdty::f32x16, b: ::simdty::f32x16, c: i16, d: i32) -> ::simdty::f32x16;
        #[link_name = "llvm.x86.avx512.rcp28.sd"]
        pub fn avx512_rcp28_sd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: ::simdty::f64x2, d: i8, e: i32) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.avx512.rcp28.ss"]
        pub fn avx512_rcp28_ss(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4, d: i8, e: i32) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.avx512.rndscale.sd"]
        pub fn avx512_rndscale_sd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: i32) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.avx512.rndscale.ss"]
        pub fn avx512_rndscale_ss(a: ::simdty::f32x4, b: ::simdty::f32x4, c: i32) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.avx512.rsqrt14.pd.512"]
        pub fn avx512_rsqrt14_pd_512(a: ::simdty::f64x8, b: ::simdty::f64x8, c: i8) -> ::simdty::f64x8;
        #[link_name = "llvm.x86.avx512.rsqrt14.ps.512"]
        pub fn avx512_rsqrt14_ps_512(a: ::simdty::f32x16, b: ::simdty::f32x16, c: i16) -> ::simdty::f32x16;
        #[link_name = "llvm.x86.avx512.rsqrt14.sd"]
        pub fn avx512_rsqrt14_sd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: ::simdty::f64x2, d: i8) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.avx512.rsqrt14.ss"]
        pub fn avx512_rsqrt14_ss(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4, d: i8) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.avx512.rsqrt28.pd"]
        pub fn avx512_rsqrt28_pd(a: ::simdty::f64x8, b: ::simdty::f64x8, c: i8, d: i32) -> ::simdty::f64x8;
        #[link_name = "llvm.x86.avx512.rsqrt28.ps"]
        pub fn avx512_rsqrt28_ps(a: ::simdty::f32x16, b: ::simdty::f32x16, c: i16, d: i32) -> ::simdty::f32x16;
        #[link_name = "llvm.x86.avx512.rsqrt28.sd"]
        pub fn avx512_rsqrt28_sd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: ::simdty::f64x2, d: i8, e: i32) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.avx512.rsqrt28.ss"]
        pub fn avx512_rsqrt28_ss(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4, d: i8, e: i32) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.avx512.scatter.dpd.512"]
        pub fn avx512_scatter_dpd_512(a: *mut i8, b: i8, c: ::simdty::i32x8, d: ::simdty::f64x8, e: i32) -> ();
        #[link_name = "llvm.x86.avx512.scatter.dpi.512"]
        pub fn avx512_scatter_dpi_512(a: *mut i8, b: i16, c: ::simdty::i32x16, d: ::simdty::i32x16, e: i32) -> ();
        #[link_name = "llvm.x86.avx512.scatter.dpq.512"]
        pub fn avx512_scatter_dpq_512(a: *mut i8, b: i8, c: ::simdty::i32x8, d: ::simdty::i64x8, e: i32) -> ();
        #[link_name = "llvm.x86.avx512.scatter.dps.512"]
        pub fn avx512_scatter_dps_512(a: *mut i8, b: i16, c: ::simdty::i32x16, d: ::simdty::f32x16, e: i32) -> ();
        #[link_name = "llvm.x86.avx512.scatter.qpd.512"]
        pub fn avx512_scatter_qpd_512(a: *mut i8, b: i8, c: ::simdty::i64x8, d: ::simdty::f64x8, e: i32) -> ();
        #[link_name = "llvm.x86.avx512.scatter.qpi.512"]
        pub fn avx512_scatter_qpi_512(a: *mut i8, b: i8, c: ::simdty::i64x8, d: ::simdty::i32x8, e: i32) -> ();
        #[link_name = "llvm.x86.avx512.scatter.qpq.512"]
        pub fn avx512_scatter_qpq_512(a: *mut i8, b: i8, c: ::simdty::i64x8, d: ::simdty::i64x8, e: i32) -> ();
        #[link_name = "llvm.x86.avx512.scatter.qps.512"]
        pub fn avx512_scatter_qps_512(a: *mut i8, b: i8, c: ::simdty::i64x8, d: ::simdty::f32x8, e: i32) -> ();
        #[link_name = "llvm.x86.avx512.scatterpf.dpd.512"]
        pub fn avx512_scatterpf_dpd_512(a: i8, b: ::simdty::i32x8, c: *mut i8, d: i32, e: i32) -> ();
        #[link_name = "llvm.x86.avx512.scatterpf.dps.512"]
        pub fn avx512_scatterpf_dps_512(a: i16, b: ::simdty::i32x16, c: *mut i8, d: i32, e: i32) -> ();
        #[link_name = "llvm.x86.avx512.scatterpf.qpd.512"]
        pub fn avx512_scatterpf_qpd_512(a: i8, b: ::simdty::i64x8, c: *mut i8, d: i32, e: i32) -> ();
        #[link_name = "llvm.x86.avx512.scatterpf.qps.512"]
        pub fn avx512_scatterpf_qps_512(a: i8, b: ::simdty::i64x8, c: *mut i8, d: i32, e: i32) -> ();
        #[link_name = "llvm.x86.avx512.sqrt.pd.512"]
        pub fn avx512_sqrt_pd_512(a: ::simdty::f64x8, b: ::simdty::f64x8, c: i8, d: i32) -> ::simdty::f64x8;
        #[link_name = "llvm.x86.avx512.sqrt.ps.512"]
        pub fn avx512_sqrt_ps_512(a: ::simdty::f32x16, b: ::simdty::f32x16, c: i16, d: i32) -> ::simdty::f32x16;
        #[link_name = "llvm.x86.avx512.sqrt.sd"]
        pub fn avx512_sqrt_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.avx512.sqrt.ss"]
        pub fn avx512_sqrt_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.avx512.vbroadcast.sd.512"]
        pub fn avx512_vbroadcast_sd_512(a: *mut i8) -> ::simdty::f64x8;
        #[link_name = "llvm.x86.avx512.vbroadcast.sd.pd.512"]
        pub fn avx512_vbroadcast_sd_pd_512(a: ::simdty::f64x2) -> ::simdty::f64x8;
        #[link_name = "llvm.x86.avx512.vbroadcast.ss.512"]
        pub fn avx512_vbroadcast_ss_512(a: *mut i8) -> ::simdty::f32x16;
        #[link_name = "llvm.x86.avx512.vbroadcast.ss.ps.512"]
        pub fn avx512_vbroadcast_ss_ps_512(a: ::simdty::f32x4) -> ::simdty::f32x16;
        #[link_name = "llvm.x86.avx.addsub.pd.256"]
        pub fn avx_addsub_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4) -> ::simdty::f64x4;
        #[link_name = "llvm.x86.avx.addsub.ps.256"]
        pub fn avx_addsub_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8) -> ::simdty::f32x8;
        #[link_name = "llvm.x86.avx.blend.pd.256"]
        pub fn avx_blend_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4, c: i8) -> ::simdty::f64x4;
        #[link_name = "llvm.x86.avx.blend.ps.256"]
        pub fn avx_blend_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8, c: i8) -> ::simdty::f32x8;
        #[link_name = "llvm.x86.avx.blendv.pd.256"]
        pub fn avx_blendv_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4, c: ::simdty::f64x4) -> ::simdty::f64x4;
        #[link_name = "llvm.x86.avx.blendv.ps.256"]
        pub fn avx_blendv_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8, c: ::simdty::f32x8) -> ::simdty::f32x8;
        #[link_name = "llvm.x86.avx.cmp.pd.256"]
        pub fn avx_cmp_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4, c: i8) -> ::simdty::f64x4;
        #[link_name = "llvm.x86.avx.cmp.ps.256"]
        pub fn avx_cmp_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8, c: i8) -> ::simdty::f32x8;
        #[link_name = "llvm.x86.avx.cvt.pd2.ps.256"]
        pub fn avx_cvt_pd2_ps_256(a: ::simdty::f64x4) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.avx.cvt.pd2dq.256"]
        pub fn avx_cvt_pd2dq_256(a: ::simdty::f64x4) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.avx.cvt.ps2.pd.256"]
        pub fn avx_cvt_ps2_pd_256(a: ::simdty::f32x4) -> ::simdty::f64x4;
        #[link_name = "llvm.x86.avx.cvt.ps2dq.256"]
        pub fn avx_cvt_ps2dq_256(a: ::simdty::f32x8) -> ::simdty::i32x8;
        #[link_name = "llvm.x86.avx.cvtdq2.pd.256"]
        pub fn avx_cvtdq2_pd_256(a: ::simdty::i32x4) -> ::simdty::f64x4;
        #[link_name = "llvm.x86.avx.cvtdq2.ps.256"]
        pub fn avx_cvtdq2_ps_256(a: ::simdty::i32x8) -> ::simdty::f32x8;
        #[link_name = "llvm.x86.avx.cvtt.pd2dq.256"]
        pub fn avx_cvtt_pd2dq_256(a: ::simdty::f64x4) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.avx.cvtt.ps2dq.256"]
        pub fn avx_cvtt_ps2dq_256(a: ::simdty::f32x8) -> ::simdty::i32x8;
        #[link_name = "llvm.x86.avx.dp.ps.256"]
        pub fn avx_dp_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8, c: i8) -> ::simdty::f32x8;
        #[link_name = "llvm.x86.avx.hadd.pd.256"]
        pub fn avx_hadd_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4) -> ::simdty::f64x4;
        #[link_name = "llvm.x86.avx.hadd.ps.256"]
        pub fn avx_hadd_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8) -> ::simdty::f32x8;
        #[link_name = "llvm.x86.avx.hsub.pd.256"]
        pub fn avx_hsub_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4) -> ::simdty::f64x4;
        #[link_name = "llvm.x86.avx.hsub.ps.256"]
        pub fn avx_hsub_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8) -> ::simdty::f32x8;
        #[link_name = "llvm.x86.avx.ldu.dq.256"]
        pub fn avx_ldu_dq_256(a: *mut i8) -> ::simdty::i8x32;
        #[link_name = "llvm.x86.avx.maskload.pd"]
        pub fn avx_maskload_pd(a: *mut i8, b: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.avx.maskload.pd.256"]
        pub fn avx_maskload_pd_256(a: *mut i8, b: ::simdty::f64x4) -> ::simdty::f64x4;
        #[link_name = "llvm.x86.avx.maskload.ps"]
        pub fn avx_maskload_ps(a: *mut i8, b: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.avx.maskload.ps.256"]
        pub fn avx_maskload_ps_256(a: *mut i8, b: ::simdty::f32x8) -> ::simdty::f32x8;
        #[link_name = "llvm.x86.avx.maskstore.pd"]
        pub fn avx_maskstore_pd(a: *mut i8, b: ::simdty::f64x2, c: ::simdty::f64x2) -> ();
        #[link_name = "llvm.x86.avx.maskstore.pd.256"]
        pub fn avx_maskstore_pd_256(a: *mut i8, b: ::simdty::f64x4, c: ::simdty::f64x4) -> ();
        #[link_name = "llvm.x86.avx.maskstore.ps"]
        pub fn avx_maskstore_ps(a: *mut i8, b: ::simdty::f32x4, c: ::simdty::f32x4) -> ();
        #[link_name = "llvm.x86.avx.maskstore.ps.256"]
        pub fn avx_maskstore_ps_256(a: *mut i8, b: ::simdty::f32x8, c: ::simdty::f32x8) -> ();
        #[link_name = "llvm.x86.avx.max.pd.256"]
        pub fn avx_max_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4) -> ::simdty::f64x4;
        #[link_name = "llvm.x86.avx.max.ps.256"]
        pub fn avx_max_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8) -> ::simdty::f32x8;
        #[link_name = "llvm.x86.avx.min.pd.256"]
        pub fn avx_min_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4) -> ::simdty::f64x4;
        #[link_name = "llvm.x86.avx.min.ps.256"]
        pub fn avx_min_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8) -> ::simdty::f32x8;
        #[link_name = "llvm.x86.avx.movmsk.pd.256"]
        pub fn avx_movmsk_pd_256(a: ::simdty::f64x4) -> i32;
        #[link_name = "llvm.x86.avx.movmsk.ps.256"]
        pub fn avx_movmsk_ps_256(a: ::simdty::f32x8) -> i32;
        #[link_name = "llvm.x86.avx.ptestc.256"]
        pub fn avx_ptestc_256(a: ::simdty::i64x4, b: ::simdty::i64x4) -> i32;
        #[link_name = "llvm.x86.avx.ptestnzc.256"]
        pub fn avx_ptestnzc_256(a: ::simdty::i64x4, b: ::simdty::i64x4) -> i32;
        #[link_name = "llvm.x86.avx.ptestz.256"]
        pub fn avx_ptestz_256(a: ::simdty::i64x4, b: ::simdty::i64x4) -> i32;
        #[link_name = "llvm.x86.avx.rcp.ps.256"]
        pub fn avx_rcp_ps_256(a: ::simdty::f32x8) -> ::simdty::f32x8;
        #[link_name = "llvm.x86.avx.round.pd.256"]
        pub fn avx_round_pd_256(a: ::simdty::f64x4, b: i32) -> ::simdty::f64x4;
        #[link_name = "llvm.x86.avx.round.ps.256"]
        pub fn avx_round_ps_256(a: ::simdty::f32x8, b: i32) -> ::simdty::f32x8;
        #[link_name = "llvm.x86.avx.rsqrt.ps.256"]
        pub fn avx_rsqrt_ps_256(a: ::simdty::f32x8) -> ::simdty::f32x8;
        #[link_name = "llvm.x86.avx.sqrt.pd.256"]
        pub fn avx_sqrt_pd_256(a: ::simdty::f64x4) -> ::simdty::f64x4;
        #[link_name = "llvm.x86.avx.sqrt.ps.256"]
        pub fn avx_sqrt_ps_256(a: ::simdty::f32x8) -> ::simdty::f32x8;
        #[link_name = "llvm.x86.avx.storeu.dq.256"]
        pub fn avx_storeu_dq_256(a: *mut i8, b: ::simdty::i8x32) -> ();
        #[link_name = "llvm.x86.avx.storeu.pd.256"]
        pub fn avx_storeu_pd_256(a: *mut i8, b: ::simdty::f64x4) -> ();
        #[link_name = "llvm.x86.avx.storeu.ps.256"]
        pub fn avx_storeu_ps_256(a: *mut i8, b: ::simdty::f32x8) -> ();
        #[link_name = "llvm.x86.avx.vbroadcastf128.pd.256"]
        pub fn avx_vbroadcastf128_pd_256(a: *mut i8) -> ::simdty::f64x4;
        #[link_name = "llvm.x86.avx.vbroadcastf128.ps.256"]
        pub fn avx_vbroadcastf128_ps_256(a: *mut i8) -> ::simdty::f32x8;
        #[link_name = "llvm.x86.avx.vextractf128.pd.256"]
        pub fn avx_vextractf128_pd_256(a: ::simdty::f64x4, b: i8) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.avx.vextractf128.ps.256"]
        pub fn avx_vextractf128_ps_256(a: ::simdty::f32x8, b: i8) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.avx.vextractf128.si.256"]
        pub fn avx_vextractf128_si_256(a: ::simdty::i32x8, b: i8) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.avx.vinsertf128.pd.256"]
        pub fn avx_vinsertf128_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x2, c: i8) -> ::simdty::f64x4;
        #[link_name = "llvm.x86.avx.vinsertf128.ps.256"]
        pub fn avx_vinsertf128_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x4, c: i8) -> ::simdty::f32x8;
        #[link_name = "llvm.x86.avx.vinsertf128.si.256"]
        pub fn avx_vinsertf128_si_256(a: ::simdty::i32x8, b: ::simdty::i32x4, c: i8) -> ::simdty::i32x8;
        #[link_name = "llvm.x86.avx.vperm2f128.pd.256"]
        pub fn avx_vperm2f128_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4, c: i8) -> ::simdty::f64x4;
        #[link_name = "llvm.x86.avx.vperm2f128.ps.256"]
        pub fn avx_vperm2f128_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8, c: i8) -> ::simdty::f32x8;
        #[link_name = "llvm.x86.avx.vperm2f128.si.256"]
        pub fn avx_vperm2f128_si_256(a: ::simdty::i32x8, b: ::simdty::i32x8, c: i8) -> ::simdty::i32x8;
        #[link_name = "llvm.x86.avx.vpermilvar.pd"]
        pub fn avx_vpermilvar_pd(a: ::simdty::f64x2, b: ::simdty::i64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.avx.vpermilvar.pd.256"]
        pub fn avx_vpermilvar_pd_256(a: ::simdty::f64x4, b: ::simdty::i64x4) -> ::simdty::f64x4;
        #[link_name = "llvm.x86.avx.vpermilvar.ps"]
        pub fn avx_vpermilvar_ps(a: ::simdty::f32x4, b: ::simdty::i32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.avx.vpermilvar.ps.256"]
        pub fn avx_vpermilvar_ps_256(a: ::simdty::f32x8, b: ::simdty::i32x8) -> ::simdty::f32x8;
        #[link_name = "llvm.x86.avx.vtestc.pd"]
        pub fn avx_vtestc_pd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> i32;
        #[link_name = "llvm.x86.avx.vtestc.pd.256"]
        pub fn avx_vtestc_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4) -> i32;
        #[link_name = "llvm.x86.avx.vtestc.ps"]
        pub fn avx_vtestc_ps(a: ::simdty::f32x4, b: ::simdty::f32x4) -> i32;
        #[link_name = "llvm.x86.avx.vtestc.ps.256"]
        pub fn avx_vtestc_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8) -> i32;
        #[link_name = "llvm.x86.avx.vtestnzc.pd"]
        pub fn avx_vtestnzc_pd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> i32;
        #[link_name = "llvm.x86.avx.vtestnzc.pd.256"]
        pub fn avx_vtestnzc_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4) -> i32;
        #[link_name = "llvm.x86.avx.vtestnzc.ps"]
        pub fn avx_vtestnzc_ps(a: ::simdty::f32x4, b: ::simdty::f32x4) -> i32;
        #[link_name = "llvm.x86.avx.vtestnzc.ps.256"]
        pub fn avx_vtestnzc_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8) -> i32;
        #[link_name = "llvm.x86.avx.vtestz.pd"]
        pub fn avx_vtestz_pd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> i32;
        #[link_name = "llvm.x86.avx.vtestz.pd.256"]
        pub fn avx_vtestz_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4) -> i32;
        #[link_name = "llvm.x86.avx.vtestz.ps"]
        pub fn avx_vtestz_ps(a: ::simdty::f32x4, b: ::simdty::f32x4) -> i32;
        #[link_name = "llvm.x86.avx.vtestz.ps.256"]
        pub fn avx_vtestz_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8) -> i32;
        #[link_name = "llvm.x86.avx.vzeroall"]
        pub fn avx_vzeroall() -> ();
        #[link_name = "llvm.x86.avx.vzeroupper"]
        pub fn avx_vzeroupper() -> ();
        #[link_name = "llvm.x86.bmi.bextr.32"]
        pub fn bmi_bextr_32(a: i32, b: i32) -> i32;
        #[link_name = "llvm.x86.bmi.bextr.64"]
        pub fn bmi_bextr_64(a: i64, b: i64) -> i64;
        #[link_name = "llvm.x86.bmi.bzhi.32"]
        pub fn bmi_bzhi_32(a: i32, b: i32) -> i32;
        #[link_name = "llvm.x86.bmi.bzhi.64"]
        pub fn bmi_bzhi_64(a: i64, b: i64) -> i64;
        #[link_name = "llvm.x86.bmi.pdep.32"]
        pub fn bmi_pdep_32(a: i32, b: i32) -> i32;
        #[link_name = "llvm.x86.bmi.pdep.64"]
        pub fn bmi_pdep_64(a: i64, b: i64) -> i64;
        #[link_name = "llvm.x86.bmi.pext.32"]
        pub fn bmi_pext_32(a: i32, b: i32) -> i32;
        #[link_name = "llvm.x86.bmi.pext.64"]
        pub fn bmi_pext_64(a: i64, b: i64) -> i64;
        #[link_name = "llvm.x86.fma.mask.vfmadd.pd.512"]
        pub fn fma_mask_vfmadd_pd_512(a: ::simdty::f64x8, b: ::simdty::f64x8, c: ::simdty::f64x8, d: i8, e: i32) -> ::simdty::f64x8;
        #[link_name = "llvm.x86.fma.mask.vfmadd.ps.512"]
        pub fn fma_mask_vfmadd_ps_512(a: ::simdty::f32x16, b: ::simdty::f32x16, c: ::simdty::f32x16, d: i16, e: i32) -> ::simdty::f32x16;
        #[link_name = "llvm.x86.fma.mask.vfmaddsub.pd.512"]
        pub fn fma_mask_vfmaddsub_pd_512(a: ::simdty::f64x8, b: ::simdty::f64x8, c: ::simdty::f64x8, d: i8, e: i32) -> ::simdty::f64x8;
        #[link_name = "llvm.x86.fma.mask.vfmaddsub.ps.512"]
        pub fn fma_mask_vfmaddsub_ps_512(a: ::simdty::f32x16, b: ::simdty::f32x16, c: ::simdty::f32x16, d: i16, e: i32) -> ::simdty::f32x16;
        #[link_name = "llvm.x86.fma.mask.vfmsub.pd.512"]
        pub fn fma_mask_vfmsub_pd_512(a: ::simdty::f64x8, b: ::simdty::f64x8, c: ::simdty::f64x8, d: i8, e: i32) -> ::simdty::f64x8;
        #[link_name = "llvm.x86.fma.mask.vfmsub.ps.512"]
        pub fn fma_mask_vfmsub_ps_512(a: ::simdty::f32x16, b: ::simdty::f32x16, c: ::simdty::f32x16, d: i16, e: i32) -> ::simdty::f32x16;
        #[link_name = "llvm.x86.fma.mask.vfmsubadd.pd.512"]
        pub fn fma_mask_vfmsubadd_pd_512(a: ::simdty::f64x8, b: ::simdty::f64x8, c: ::simdty::f64x8, d: i8, e: i32) -> ::simdty::f64x8;
        #[link_name = "llvm.x86.fma.mask.vfmsubadd.ps.512"]
        pub fn fma_mask_vfmsubadd_ps_512(a: ::simdty::f32x16, b: ::simdty::f32x16, c: ::simdty::f32x16, d: i16, e: i32) -> ::simdty::f32x16;
        #[link_name = "llvm.x86.fma.mask.vfnmadd.pd.512"]
        pub fn fma_mask_vfnmadd_pd_512(a: ::simdty::f64x8, b: ::simdty::f64x8, c: ::simdty::f64x8, d: i8, e: i32) -> ::simdty::f64x8;
        #[link_name = "llvm.x86.fma.mask.vfnmadd.ps.512"]
        pub fn fma_mask_vfnmadd_ps_512(a: ::simdty::f32x16, b: ::simdty::f32x16, c: ::simdty::f32x16, d: i16, e: i32) -> ::simdty::f32x16;
        #[link_name = "llvm.x86.fma.mask.vfnmsub.pd.512"]
        pub fn fma_mask_vfnmsub_pd_512(a: ::simdty::f64x8, b: ::simdty::f64x8, c: ::simdty::f64x8, d: i8, e: i32) -> ::simdty::f64x8;
        #[link_name = "llvm.x86.fma.mask.vfnmsub.ps.512"]
        pub fn fma_mask_vfnmsub_ps_512(a: ::simdty::f32x16, b: ::simdty::f32x16, c: ::simdty::f32x16, d: i16, e: i32) -> ::simdty::f32x16;
        #[link_name = "llvm.x86.fma.vfmadd.pd"]
        pub fn fma_vfmadd_pd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.fma.vfmadd.pd.256"]
        pub fn fma_vfmadd_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4, c: ::simdty::f64x4) -> ::simdty::f64x4;
        #[link_name = "llvm.x86.fma.vfmadd.ps"]
        pub fn fma_vfmadd_ps(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.fma.vfmadd.ps.256"]
        pub fn fma_vfmadd_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8, c: ::simdty::f32x8) -> ::simdty::f32x8;
        #[link_name = "llvm.x86.fma.vfmadd.sd"]
        pub fn fma_vfmadd_sd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.fma.vfmadd.ss"]
        pub fn fma_vfmadd_ss(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.fma.vfmaddsub.pd"]
        pub fn fma_vfmaddsub_pd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.fma.vfmaddsub.pd.256"]
        pub fn fma_vfmaddsub_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4, c: ::simdty::f64x4) -> ::simdty::f64x4;
        #[link_name = "llvm.x86.fma.vfmaddsub.ps"]
        pub fn fma_vfmaddsub_ps(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.fma.vfmaddsub.ps.256"]
        pub fn fma_vfmaddsub_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8, c: ::simdty::f32x8) -> ::simdty::f32x8;
        #[link_name = "llvm.x86.fma.vfmsub.pd"]
        pub fn fma_vfmsub_pd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.fma.vfmsub.pd.256"]
        pub fn fma_vfmsub_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4, c: ::simdty::f64x4) -> ::simdty::f64x4;
        #[link_name = "llvm.x86.fma.vfmsub.ps"]
        pub fn fma_vfmsub_ps(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.fma.vfmsub.ps.256"]
        pub fn fma_vfmsub_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8, c: ::simdty::f32x8) -> ::simdty::f32x8;
        #[link_name = "llvm.x86.fma.vfmsub.sd"]
        pub fn fma_vfmsub_sd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.fma.vfmsub.ss"]
        pub fn fma_vfmsub_ss(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.fma.vfmsubadd.pd"]
        pub fn fma_vfmsubadd_pd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.fma.vfmsubadd.pd.256"]
        pub fn fma_vfmsubadd_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4, c: ::simdty::f64x4) -> ::simdty::f64x4;
        #[link_name = "llvm.x86.fma.vfmsubadd.ps"]
        pub fn fma_vfmsubadd_ps(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.fma.vfmsubadd.ps.256"]
        pub fn fma_vfmsubadd_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8, c: ::simdty::f32x8) -> ::simdty::f32x8;
        #[link_name = "llvm.x86.fma.vfnmadd.pd"]
        pub fn fma_vfnmadd_pd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.fma.vfnmadd.pd.256"]
        pub fn fma_vfnmadd_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4, c: ::simdty::f64x4) -> ::simdty::f64x4;
        #[link_name = "llvm.x86.fma.vfnmadd.ps"]
        pub fn fma_vfnmadd_ps(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.fma.vfnmadd.ps.256"]
        pub fn fma_vfnmadd_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8, c: ::simdty::f32x8) -> ::simdty::f32x8;
        #[link_name = "llvm.x86.fma.vfnmadd.sd"]
        pub fn fma_vfnmadd_sd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.fma.vfnmadd.ss"]
        pub fn fma_vfnmadd_ss(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.fma.vfnmsub.pd"]
        pub fn fma_vfnmsub_pd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.fma.vfnmsub.pd.256"]
        pub fn fma_vfnmsub_pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4, c: ::simdty::f64x4) -> ::simdty::f64x4;
        #[link_name = "llvm.x86.fma.vfnmsub.ps"]
        pub fn fma_vfnmsub_ps(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.fma.vfnmsub.ps.256"]
        pub fn fma_vfnmsub_ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8, c: ::simdty::f32x8) -> ::simdty::f32x8;
        #[link_name = "llvm.x86.fma.vfnmsub.sd"]
        pub fn fma_vfnmsub_sd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.fma.vfnmsub.ss"]
        pub fn fma_vfnmsub_ss(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.int"]
        pub fn int(a: i8) -> ();
        #[link_name = "llvm.x86.mmx.emms"]
        pub fn mmx_emms() -> ();
        #[link_name = "llvm.x86.mmx.femms"]
        pub fn mmx_femms() -> ();
        #[link_name = "llvm.x86.pclmulqdq"]
        pub fn pclmulqdq(a: ::simdty::i64x2, b: ::simdty::i64x2, c: i8) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.rdfsbase.32"]
        pub fn rdfsbase_32() -> i32;
        #[link_name = "llvm.x86.rdfsbase.64"]
        pub fn rdfsbase_64() -> i64;
        #[link_name = "llvm.x86.rdgsbase.32"]
        pub fn rdgsbase_32() -> i32;
        #[link_name = "llvm.x86.rdgsbase.64"]
        pub fn rdgsbase_64() -> i64;
        #[link_name = "llvm.x86.rdpmc"]
        pub fn rdpmc(a: i32) -> i64;
        #[link_name = "llvm.x86.rdtsc"]
        pub fn rdtsc() -> i64;
        #[link_name = "llvm.x86.rdtscp"]
        pub fn rdtscp(a: *mut i8) -> i64;
        #[link_name = "llvm.x86.sha1msg1"]
        pub fn sha1msg1(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.sha1msg2"]
        pub fn sha1msg2(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.sha1nexte"]
        pub fn sha1nexte(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.sha1rnds4"]
        pub fn sha1rnds4(a: ::simdty::i32x4, b: ::simdty::i32x4, c: i8) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.sha256msg1"]
        pub fn sha256msg1(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.sha256msg2"]
        pub fn sha256msg2(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.sha256rnds2"]
        pub fn sha256rnds2(a: ::simdty::i32x4, b: ::simdty::i32x4, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.sse2.add.sd"]
        pub fn sse2_add_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.sse2.clflush"]
        pub fn sse2_clflush(a: *mut i8) -> ();
        #[link_name = "llvm.x86.sse2.cmp.pd"]
        pub fn sse2_cmp_pd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: i8) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.sse2.cmp.sd"]
        pub fn sse2_cmp_sd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: i8) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.sse2.comieq.sd"]
        pub fn sse2_comieq_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> i32;
        #[link_name = "llvm.x86.sse2.comige.sd"]
        pub fn sse2_comige_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> i32;
        #[link_name = "llvm.x86.sse2.comigt.sd"]
        pub fn sse2_comigt_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> i32;
        #[link_name = "llvm.x86.sse2.comile.sd"]
        pub fn sse2_comile_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> i32;
        #[link_name = "llvm.x86.sse2.comilt.sd"]
        pub fn sse2_comilt_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> i32;
        #[link_name = "llvm.x86.sse2.comineq.sd"]
        pub fn sse2_comineq_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> i32;
        #[link_name = "llvm.x86.sse2.cvtdq2pd"]
        pub fn sse2_cvtdq2pd(a: ::simdty::i32x4) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.sse2.cvtdq2ps"]
        pub fn sse2_cvtdq2ps(a: ::simdty::i32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.sse2.cvtpd2dq"]
        pub fn sse2_cvtpd2dq(a: ::simdty::f64x2) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.sse2.cvtpd2ps"]
        pub fn sse2_cvtpd2ps(a: ::simdty::f64x2) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.sse2.cvtps2dq"]
        pub fn sse2_cvtps2dq(a: ::simdty::f32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.sse2.cvtps2pd"]
        pub fn sse2_cvtps2pd(a: ::simdty::f32x4) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.sse2.cvtsd2si"]
        pub fn sse2_cvtsd2si(a: ::simdty::f64x2) -> i32;
        #[link_name = "llvm.x86.sse2.cvtsd2si64"]
        pub fn sse2_cvtsd2si64(a: ::simdty::f64x2) -> i64;
        #[link_name = "llvm.x86.sse2.cvtsd2ss"]
        pub fn sse2_cvtsd2ss(a: ::simdty::f32x4, b: ::simdty::f64x2) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.sse2.cvtsi2sd"]
        pub fn sse2_cvtsi2sd(a: ::simdty::f64x2, b: i32) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.sse2.cvtsi642sd"]
        pub fn sse2_cvtsi642sd(a: ::simdty::f64x2, b: i64) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.sse2.cvtss2sd"]
        pub fn sse2_cvtss2sd(a: ::simdty::f64x2, b: ::simdty::f32x4) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.sse2.cvttpd2dq"]
        pub fn sse2_cvttpd2dq(a: ::simdty::f64x2) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.sse2.cvttps2dq"]
        pub fn sse2_cvttps2dq(a: ::simdty::f32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.sse2.cvttsd2si"]
        pub fn sse2_cvttsd2si(a: ::simdty::f64x2) -> i32;
        #[link_name = "llvm.x86.sse2.cvttsd2si64"]
        pub fn sse2_cvttsd2si64(a: ::simdty::f64x2) -> i64;
        #[link_name = "llvm.x86.sse2.div.sd"]
        pub fn sse2_div_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.sse2.lfence"]
        pub fn sse2_lfence() -> ();
        #[link_name = "llvm.x86.sse2.maskmov.dqu"]
        pub fn sse2_maskmov_dqu(a: ::simdty::i8x16, b: ::simdty::i8x16, c: *mut i8) -> ();
        #[link_name = "llvm.x86.sse2.max.pd"]
        pub fn sse2_max_pd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.sse2.max.sd"]
        pub fn sse2_max_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.sse2.mfence"]
        pub fn sse2_mfence() -> ();
        #[link_name = "llvm.x86.sse2.min.pd"]
        pub fn sse2_min_pd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.sse2.min.sd"]
        pub fn sse2_min_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.sse2.movmsk.pd"]
        pub fn sse2_movmsk_pd(a: ::simdty::f64x2) -> i32;
        #[link_name = "llvm.x86.sse2.mul.sd"]
        pub fn sse2_mul_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.sse2.packssdw.128"]
        pub fn sse2_packssdw_128(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.sse2.packsswb.128"]
        pub fn sse2_packsswb_128(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i8x16;
        #[link_name = "llvm.x86.sse2.packuswb.128"]
        pub fn sse2_packuswb_128(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i8x16;
        #[link_name = "llvm.x86.sse2.padds.b"]
        pub fn sse2_padds_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.x86.sse2.padds.w"]
        pub fn sse2_padds_w(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.sse2.paddus.b"]
        pub fn sse2_paddus_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.x86.sse2.paddus.w"]
        pub fn sse2_paddus_w(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.sse2.pause"]
        pub fn sse2_pause() -> ();
        #[link_name = "llvm.x86.sse2.pavg.b"]
        pub fn sse2_pavg_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.x86.sse2.pavg.w"]
        pub fn sse2_pavg_w(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.sse2.pmadd.wd"]
        pub fn sse2_pmadd_wd(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.sse2.pmaxs.w"]
        pub fn sse2_pmaxs_w(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.sse2.pmaxu.b"]
        pub fn sse2_pmaxu_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.x86.sse2.pmins.w"]
        pub fn sse2_pmins_w(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.sse2.pminu.b"]
        pub fn sse2_pminu_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.x86.sse2.pmovmskb.128"]
        pub fn sse2_pmovmskb_128(a: ::simdty::i8x16) -> i32;
        #[link_name = "llvm.x86.sse2.pmulh.w"]
        pub fn sse2_pmulh_w(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.sse2.pmulhu.w"]
        pub fn sse2_pmulhu_w(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.sse2.pmulu.dq"]
        pub fn sse2_pmulu_dq(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.sse2.psad.bw"]
        pub fn sse2_psad_bw(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.sse2.pshuf.d"]
        pub fn sse2_pshuf_d(a: ::simdty::i32x4, b: i8) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.sse2.pshufh.w"]
        pub fn sse2_pshufh_w(a: ::simdty::i16x8, b: i8) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.sse2.pshufl.w"]
        pub fn sse2_pshufl_w(a: ::simdty::i16x8, b: i8) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.sse2.psll.d"]
        pub fn sse2_psll_d(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.sse2.psll.dq"]
        pub fn sse2_psll_dq(a: ::simdty::i64x2, b: i32) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.sse2.psll.dq.bs"]
        pub fn sse2_psll_dq_bs(a: ::simdty::i64x2, b: i32) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.sse2.psll.q"]
        pub fn sse2_psll_q(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.sse2.psll.w"]
        pub fn sse2_psll_w(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.sse2.pslli.d"]
        pub fn sse2_pslli_d(a: ::simdty::i32x4, b: i32) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.sse2.pslli.q"]
        pub fn sse2_pslli_q(a: ::simdty::i64x2, b: i32) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.sse2.pslli.w"]
        pub fn sse2_pslli_w(a: ::simdty::i16x8, b: i32) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.sse2.psra.d"]
        pub fn sse2_psra_d(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.sse2.psra.w"]
        pub fn sse2_psra_w(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.sse2.psrai.d"]
        pub fn sse2_psrai_d(a: ::simdty::i32x4, b: i32) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.sse2.psrai.w"]
        pub fn sse2_psrai_w(a: ::simdty::i16x8, b: i32) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.sse2.psrl.d"]
        pub fn sse2_psrl_d(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.sse2.psrl.dq"]
        pub fn sse2_psrl_dq(a: ::simdty::i64x2, b: i32) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.sse2.psrl.dq.bs"]
        pub fn sse2_psrl_dq_bs(a: ::simdty::i64x2, b: i32) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.sse2.psrl.q"]
        pub fn sse2_psrl_q(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.sse2.psrl.w"]
        pub fn sse2_psrl_w(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.sse2.psrli.d"]
        pub fn sse2_psrli_d(a: ::simdty::i32x4, b: i32) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.sse2.psrli.q"]
        pub fn sse2_psrli_q(a: ::simdty::i64x2, b: i32) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.sse2.psrli.w"]
        pub fn sse2_psrli_w(a: ::simdty::i16x8, b: i32) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.sse2.psubs.b"]
        pub fn sse2_psubs_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.x86.sse2.psubs.w"]
        pub fn sse2_psubs_w(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.sse2.psubus.b"]
        pub fn sse2_psubus_b(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.x86.sse2.psubus.w"]
        pub fn sse2_psubus_w(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.sse2.sqrt.pd"]
        pub fn sse2_sqrt_pd(a: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.sse2.sqrt.sd"]
        pub fn sse2_sqrt_sd(a: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.sse2.storel.dq"]
        pub fn sse2_storel_dq(a: *mut i8, b: ::simdty::i32x4) -> ();
        #[link_name = "llvm.x86.sse2.storeu.dq"]
        pub fn sse2_storeu_dq(a: *mut i8, b: ::simdty::i8x16) -> ();
        #[link_name = "llvm.x86.sse2.storeu.pd"]
        pub fn sse2_storeu_pd(a: *mut i8, b: ::simdty::f64x2) -> ();
        #[link_name = "llvm.x86.sse2.sub.sd"]
        pub fn sse2_sub_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.sse2.ucomieq.sd"]
        pub fn sse2_ucomieq_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> i32;
        #[link_name = "llvm.x86.sse2.ucomige.sd"]
        pub fn sse2_ucomige_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> i32;
        #[link_name = "llvm.x86.sse2.ucomigt.sd"]
        pub fn sse2_ucomigt_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> i32;
        #[link_name = "llvm.x86.sse2.ucomile.sd"]
        pub fn sse2_ucomile_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> i32;
        #[link_name = "llvm.x86.sse2.ucomilt.sd"]
        pub fn sse2_ucomilt_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> i32;
        #[link_name = "llvm.x86.sse2.ucomineq.sd"]
        pub fn sse2_ucomineq_sd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> i32;
        #[link_name = "llvm.x86.sse3.addsub.pd"]
        pub fn sse3_addsub_pd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.sse3.addsub.ps"]
        pub fn sse3_addsub_ps(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.sse3.hadd.pd"]
        pub fn sse3_hadd_pd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.sse3.hadd.ps"]
        pub fn sse3_hadd_ps(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.sse3.hsub.pd"]
        pub fn sse3_hsub_pd(a: ::simdty::f64x2, b: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.sse3.hsub.ps"]
        pub fn sse3_hsub_ps(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.sse3.ldu.dq"]
        pub fn sse3_ldu_dq(a: *mut i8) -> ::simdty::i8x16;
        #[link_name = "llvm.x86.sse3.monitor"]
        pub fn sse3_monitor(a: *mut i8, b: i32, c: i32) -> ();
        #[link_name = "llvm.x86.sse3.mwait"]
        pub fn sse3_mwait(a: i32, b: i32) -> ();
        #[link_name = "llvm.x86.sse41.blendpd"]
        pub fn sse41_blendpd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: i8) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.sse41.blendps"]
        pub fn sse41_blendps(a: ::simdty::f32x4, b: ::simdty::f32x4, c: i8) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.sse41.blendvpd"]
        pub fn sse41_blendvpd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.sse41.blendvps"]
        pub fn sse41_blendvps(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.sse41.dppd"]
        pub fn sse41_dppd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: i8) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.sse41.dpps"]
        pub fn sse41_dpps(a: ::simdty::f32x4, b: ::simdty::f32x4, c: i8) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.sse41.extractps"]
        pub fn sse41_extractps(a: ::simdty::f32x4, b: i32) -> i32;
        #[link_name = "llvm.x86.sse41.insertps"]
        pub fn sse41_insertps(a: ::simdty::f32x4, b: ::simdty::f32x4, c: i8) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.sse41.movntdqa"]
        pub fn sse41_movntdqa(a: *mut i8) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.sse41.mpsadbw"]
        pub fn sse41_mpsadbw(a: ::simdty::i8x16, b: ::simdty::i8x16, c: i8) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.sse41.packusdw"]
        pub fn sse41_packusdw(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.sse41.pblendvb"]
        pub fn sse41_pblendvb(a: ::simdty::i8x16, b: ::simdty::i8x16, c: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.x86.sse41.pblendw"]
        pub fn sse41_pblendw(a: ::simdty::i16x8, b: ::simdty::i16x8, c: i8) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.sse41.pextrb"]
        pub fn sse41_pextrb(a: ::simdty::i8x16, b: i32) -> i32;
        #[link_name = "llvm.x86.sse41.pextrd"]
        pub fn sse41_pextrd(a: ::simdty::i32x4, b: i32) -> i32;
        #[link_name = "llvm.x86.sse41.pextrq"]
        pub fn sse41_pextrq(a: ::simdty::i64x2, b: i32) -> i64;
        #[link_name = "llvm.x86.sse41.phminposuw"]
        pub fn sse41_phminposuw(a: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.sse41.pmaxsb"]
        pub fn sse41_pmaxsb(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.x86.sse41.pmaxsd"]
        pub fn sse41_pmaxsd(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.sse41.pmaxud"]
        pub fn sse41_pmaxud(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.sse41.pmaxuw"]
        pub fn sse41_pmaxuw(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.sse41.pminsb"]
        pub fn sse41_pminsb(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.x86.sse41.pminsd"]
        pub fn sse41_pminsd(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.sse41.pminud"]
        pub fn sse41_pminud(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.sse41.pminuw"]
        pub fn sse41_pminuw(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.sse41.pmovsxbd"]
        pub fn sse41_pmovsxbd(a: ::simdty::i8x16) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.sse41.pmovsxbq"]
        pub fn sse41_pmovsxbq(a: ::simdty::i8x16) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.sse41.pmovsxbw"]
        pub fn sse41_pmovsxbw(a: ::simdty::i8x16) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.sse41.pmovsxdq"]
        pub fn sse41_pmovsxdq(a: ::simdty::i32x4) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.sse41.pmovsxwd"]
        pub fn sse41_pmovsxwd(a: ::simdty::i16x8) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.sse41.pmovsxwq"]
        pub fn sse41_pmovsxwq(a: ::simdty::i16x8) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.sse41.pmovzxbd"]
        pub fn sse41_pmovzxbd(a: ::simdty::i8x16) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.sse41.pmovzxbq"]
        pub fn sse41_pmovzxbq(a: ::simdty::i8x16) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.sse41.pmovzxbw"]
        pub fn sse41_pmovzxbw(a: ::simdty::i8x16) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.sse41.pmovzxdq"]
        pub fn sse41_pmovzxdq(a: ::simdty::i32x4) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.sse41.pmovzxwd"]
        pub fn sse41_pmovzxwd(a: ::simdty::i16x8) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.sse41.pmovzxwq"]
        pub fn sse41_pmovzxwq(a: ::simdty::i16x8) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.sse41.pmuldq"]
        pub fn sse41_pmuldq(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.sse41.ptestc"]
        pub fn sse41_ptestc(a: ::simdty::i64x2, b: ::simdty::i64x2) -> i32;
        #[link_name = "llvm.x86.sse41.ptestnzc"]
        pub fn sse41_ptestnzc(a: ::simdty::i64x2, b: ::simdty::i64x2) -> i32;
        #[link_name = "llvm.x86.sse41.ptestz"]
        pub fn sse41_ptestz(a: ::simdty::i64x2, b: ::simdty::i64x2) -> i32;
        #[link_name = "llvm.x86.sse41.round.pd"]
        pub fn sse41_round_pd(a: ::simdty::f64x2, b: i32) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.sse41.round.ps"]
        pub fn sse41_round_ps(a: ::simdty::f32x4, b: i32) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.sse41.round.sd"]
        pub fn sse41_round_sd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: i32) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.sse41.round.ss"]
        pub fn sse41_round_ss(a: ::simdty::f32x4, b: ::simdty::f32x4, c: i32) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.sse42.crc32.32.16"]
        pub fn sse42_crc32_32_16(a: i32, b: i16) -> i32;
        #[link_name = "llvm.x86.sse42.crc32.32.32"]
        pub fn sse42_crc32_32_32(a: i32, b: i32) -> i32;
        #[link_name = "llvm.x86.sse42.crc32.32.8"]
        pub fn sse42_crc32_32_8(a: i32, b: i8) -> i32;
        #[link_name = "llvm.x86.sse42.crc32.64.64"]
        pub fn sse42_crc32_64_64(a: i64, b: i64) -> i64;
        #[link_name = "llvm.x86.sse42.pcmpestri128"]
        pub fn sse42_pcmpestri128(a: ::simdty::i8x16, b: i32, c: ::simdty::i8x16, d: i32, e: i8) -> i32;
        #[link_name = "llvm.x86.sse42.pcmpestria128"]
        pub fn sse42_pcmpestria128(a: ::simdty::i8x16, b: i32, c: ::simdty::i8x16, d: i32, e: i8) -> i32;
        #[link_name = "llvm.x86.sse42.pcmpestric128"]
        pub fn sse42_pcmpestric128(a: ::simdty::i8x16, b: i32, c: ::simdty::i8x16, d: i32, e: i8) -> i32;
        #[link_name = "llvm.x86.sse42.pcmpestrio128"]
        pub fn sse42_pcmpestrio128(a: ::simdty::i8x16, b: i32, c: ::simdty::i8x16, d: i32, e: i8) -> i32;
        #[link_name = "llvm.x86.sse42.pcmpestris128"]
        pub fn sse42_pcmpestris128(a: ::simdty::i8x16, b: i32, c: ::simdty::i8x16, d: i32, e: i8) -> i32;
        #[link_name = "llvm.x86.sse42.pcmpestriz128"]
        pub fn sse42_pcmpestriz128(a: ::simdty::i8x16, b: i32, c: ::simdty::i8x16, d: i32, e: i8) -> i32;
        #[link_name = "llvm.x86.sse42.pcmpestrm128"]
        pub fn sse42_pcmpestrm128(a: ::simdty::i8x16, b: i32, c: ::simdty::i8x16, d: i32, e: i8) -> ::simdty::i8x16;
        #[link_name = "llvm.x86.sse42.pcmpistri128"]
        pub fn sse42_pcmpistri128(a: ::simdty::i8x16, b: ::simdty::i8x16, c: i8) -> i32;
        #[link_name = "llvm.x86.sse42.pcmpistria128"]
        pub fn sse42_pcmpistria128(a: ::simdty::i8x16, b: ::simdty::i8x16, c: i8) -> i32;
        #[link_name = "llvm.x86.sse42.pcmpistric128"]
        pub fn sse42_pcmpistric128(a: ::simdty::i8x16, b: ::simdty::i8x16, c: i8) -> i32;
        #[link_name = "llvm.x86.sse42.pcmpistrio128"]
        pub fn sse42_pcmpistrio128(a: ::simdty::i8x16, b: ::simdty::i8x16, c: i8) -> i32;
        #[link_name = "llvm.x86.sse42.pcmpistris128"]
        pub fn sse42_pcmpistris128(a: ::simdty::i8x16, b: ::simdty::i8x16, c: i8) -> i32;
        #[link_name = "llvm.x86.sse42.pcmpistriz128"]
        pub fn sse42_pcmpistriz128(a: ::simdty::i8x16, b: ::simdty::i8x16, c: i8) -> i32;
        #[link_name = "llvm.x86.sse42.pcmpistrm128"]
        pub fn sse42_pcmpistrm128(a: ::simdty::i8x16, b: ::simdty::i8x16, c: i8) -> ::simdty::i8x16;
        #[link_name = "llvm.x86.sse4a.extrq"]
        pub fn sse4a_extrq(a: ::simdty::i64x2, b: ::simdty::i8x16) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.sse4a.extrqi"]
        pub fn sse4a_extrqi(a: ::simdty::i64x2, b: i8, c: i8) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.sse4a.insertq"]
        pub fn sse4a_insertq(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.sse4a.insertqi"]
        pub fn sse4a_insertqi(a: ::simdty::i64x2, b: ::simdty::i64x2, c: i8, d: i8) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.sse4a.movnt.sd"]
        pub fn sse4a_movnt_sd(a: *mut i8, b: ::simdty::f64x2) -> ();
        #[link_name = "llvm.x86.sse4a.movnt.ss"]
        pub fn sse4a_movnt_ss(a: *mut i8, b: ::simdty::f32x4) -> ();
        #[link_name = "llvm.x86.sse.add.ss"]
        pub fn sse_add_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.sse.cmp.ps"]
        pub fn sse_cmp_ps(a: ::simdty::f32x4, b: ::simdty::f32x4, c: i8) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.sse.cmp.ss"]
        pub fn sse_cmp_ss(a: ::simdty::f32x4, b: ::simdty::f32x4, c: i8) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.sse.comieq.ss"]
        pub fn sse_comieq_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> i32;
        #[link_name = "llvm.x86.sse.comige.ss"]
        pub fn sse_comige_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> i32;
        #[link_name = "llvm.x86.sse.comigt.ss"]
        pub fn sse_comigt_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> i32;
        #[link_name = "llvm.x86.sse.comile.ss"]
        pub fn sse_comile_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> i32;
        #[link_name = "llvm.x86.sse.comilt.ss"]
        pub fn sse_comilt_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> i32;
        #[link_name = "llvm.x86.sse.comineq.ss"]
        pub fn sse_comineq_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> i32;
        #[link_name = "llvm.x86.sse.cvtsi2ss"]
        pub fn sse_cvtsi2ss(a: ::simdty::f32x4, b: i32) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.sse.cvtsi642ss"]
        pub fn sse_cvtsi642ss(a: ::simdty::f32x4, b: i64) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.sse.cvtss2si"]
        pub fn sse_cvtss2si(a: ::simdty::f32x4) -> i32;
        #[link_name = "llvm.x86.sse.cvtss2si64"]
        pub fn sse_cvtss2si64(a: ::simdty::f32x4) -> i64;
        #[link_name = "llvm.x86.sse.cvttss2si"]
        pub fn sse_cvttss2si(a: ::simdty::f32x4) -> i32;
        #[link_name = "llvm.x86.sse.cvttss2si64"]
        pub fn sse_cvttss2si64(a: ::simdty::f32x4) -> i64;
        #[link_name = "llvm.x86.sse.div.ss"]
        pub fn sse_div_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.sse.ldmxcsr"]
        pub fn sse_ldmxcsr(a: *mut i8) -> ();
        #[link_name = "llvm.x86.sse.max.ps"]
        pub fn sse_max_ps(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.sse.max.ss"]
        pub fn sse_max_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.sse.min.ps"]
        pub fn sse_min_ps(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.sse.min.ss"]
        pub fn sse_min_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.sse.movmsk.ps"]
        pub fn sse_movmsk_ps(a: ::simdty::f32x4) -> i32;
        #[link_name = "llvm.x86.sse.mul.ss"]
        pub fn sse_mul_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.sse.rcp.ps"]
        pub fn sse_rcp_ps(a: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.sse.rcp.ss"]
        pub fn sse_rcp_ss(a: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.sse.rsqrt.ps"]
        pub fn sse_rsqrt_ps(a: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.sse.rsqrt.ss"]
        pub fn sse_rsqrt_ss(a: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.sse.sfence"]
        pub fn sse_sfence() -> ();
        #[link_name = "llvm.x86.sse.sqrt.ps"]
        pub fn sse_sqrt_ps(a: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.sse.sqrt.ss"]
        pub fn sse_sqrt_ss(a: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.sse.stmxcsr"]
        pub fn sse_stmxcsr(a: *mut i8) -> ();
        #[link_name = "llvm.x86.sse.storeu.ps"]
        pub fn sse_storeu_ps(a: *mut i8, b: ::simdty::f32x4) -> ();
        #[link_name = "llvm.x86.sse.sub.ss"]
        pub fn sse_sub_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.sse.ucomieq.ss"]
        pub fn sse_ucomieq_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> i32;
        #[link_name = "llvm.x86.sse.ucomige.ss"]
        pub fn sse_ucomige_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> i32;
        #[link_name = "llvm.x86.sse.ucomigt.ss"]
        pub fn sse_ucomigt_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> i32;
        #[link_name = "llvm.x86.sse.ucomile.ss"]
        pub fn sse_ucomile_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> i32;
        #[link_name = "llvm.x86.sse.ucomilt.ss"]
        pub fn sse_ucomilt_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> i32;
        #[link_name = "llvm.x86.sse.ucomineq.ss"]
        pub fn sse_ucomineq_ss(a: ::simdty::f32x4, b: ::simdty::f32x4) -> i32;
        #[link_name = "llvm.x86.ssse3.pabs.b.128"]
        pub fn ssse3_pabs_b_128(a: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.x86.ssse3.pabs.d.128"]
        pub fn ssse3_pabs_d_128(a: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.ssse3.pabs.w.128"]
        pub fn ssse3_pabs_w_128(a: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.ssse3.phadd.d.128"]
        pub fn ssse3_phadd_d_128(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.ssse3.phadd.sw.128"]
        pub fn ssse3_phadd_sw_128(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.ssse3.phadd.w.128"]
        pub fn ssse3_phadd_w_128(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.ssse3.phsub.d.128"]
        pub fn ssse3_phsub_d_128(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.ssse3.phsub.sw.128"]
        pub fn ssse3_phsub_sw_128(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.ssse3.phsub.w.128"]
        pub fn ssse3_phsub_w_128(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.ssse3.pmadd.ub.sw.128"]
        pub fn ssse3_pmadd_ub_sw_128(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.ssse3.pmul.hr.sw.128"]
        pub fn ssse3_pmul_hr_sw_128(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.ssse3.pshuf.b.128"]
        pub fn ssse3_pshuf_b_128(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.x86.ssse3.psign.b.128"]
        pub fn ssse3_psign_b_128(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.x86.ssse3.psign.d.128"]
        pub fn ssse3_psign_d_128(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.ssse3.psign.w.128"]
        pub fn ssse3_psign_w_128(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.subborrow.u32"]
        pub fn subborrow_u32(a: i8, b: i32, c: i32, d: *mut i8) -> i8;
        #[link_name = "llvm.x86.subborrow.u64"]
        pub fn subborrow_u64(a: i8, b: i64, c: i64, d: *mut i8) -> i8;
        #[link_name = "llvm.x86.tbm.bextri.u32"]
        pub fn tbm_bextri_u32(a: i32, b: i32) -> i32;
        #[link_name = "llvm.x86.tbm.bextri.u64"]
        pub fn tbm_bextri_u64(a: i64, b: i64) -> i64;
        #[link_name = "llvm.x86.vcvtph2ps.128"]
        pub fn vcvtph2ps_128(a: ::simdty::i16x8) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.vcvtph2ps.256"]
        pub fn vcvtph2ps_256(a: ::simdty::i16x8) -> ::simdty::f32x8;
        #[link_name = "llvm.x86.vcvtps2ph.128"]
        pub fn vcvtps2ph_128(a: ::simdty::f32x4, b: i32) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.vcvtps2ph.256"]
        pub fn vcvtps2ph_256(a: ::simdty::f32x8, b: i32) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.wrfsbase.32"]
        pub fn wrfsbase_32(a: i32) -> ();
        #[link_name = "llvm.x86.wrfsbase.64"]
        pub fn wrfsbase_64(a: i64) -> ();
        #[link_name = "llvm.x86.wrgsbase.32"]
        pub fn wrgsbase_32(a: i32) -> ();
        #[link_name = "llvm.x86.wrgsbase.64"]
        pub fn wrgsbase_64(a: i64) -> ();
        #[link_name = "llvm.x86.xabort"]
        pub fn xabort(a: i8) -> ();
        #[link_name = "llvm.x86.xbegin"]
        pub fn xbegin() -> i32;
        #[link_name = "llvm.x86.xend"]
        pub fn xend() -> ();
        #[link_name = "llvm.x86.xop.vfrcz.pd"]
        pub fn xop_vfrcz_pd(a: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.xop.vfrcz.pd.256"]
        pub fn xop_vfrcz_pd_256(a: ::simdty::f64x4) -> ::simdty::f64x4;
        #[link_name = "llvm.x86.xop.vfrcz.ps"]
        pub fn xop_vfrcz_ps(a: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.xop.vfrcz.ps.256"]
        pub fn xop_vfrcz_ps_256(a: ::simdty::f32x8) -> ::simdty::f32x8;
        #[link_name = "llvm.x86.xop.vfrcz.sd"]
        pub fn xop_vfrcz_sd(a: ::simdty::f64x2) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.xop.vfrcz.ss"]
        pub fn xop_vfrcz_ss(a: ::simdty::f32x4) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.xop.vpcmov"]
        pub fn xop_vpcmov(a: ::simdty::i64x2, b: ::simdty::i64x2, c: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.xop.vpcmov.256"]
        pub fn xop_vpcmov_256(a: ::simdty::i64x4, b: ::simdty::i64x4, c: ::simdty::i64x4) -> ::simdty::i64x4;
        #[link_name = "llvm.x86.xop.vpcomb"]
        pub fn xop_vpcomb(a: ::simdty::i8x16, b: ::simdty::i8x16, c: i8) -> ::simdty::i8x16;
        #[link_name = "llvm.x86.xop.vpcomd"]
        pub fn xop_vpcomd(a: ::simdty::i32x4, b: ::simdty::i32x4, c: i8) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.xop.vpcomq"]
        pub fn xop_vpcomq(a: ::simdty::i64x2, b: ::simdty::i64x2, c: i8) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.xop.vpcomub"]
        pub fn xop_vpcomub(a: ::simdty::i8x16, b: ::simdty::i8x16, c: i8) -> ::simdty::i8x16;
        #[link_name = "llvm.x86.xop.vpcomud"]
        pub fn xop_vpcomud(a: ::simdty::i32x4, b: ::simdty::i32x4, c: i8) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.xop.vpcomuq"]
        pub fn xop_vpcomuq(a: ::simdty::i64x2, b: ::simdty::i64x2, c: i8) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.xop.vpcomuw"]
        pub fn xop_vpcomuw(a: ::simdty::i16x8, b: ::simdty::i16x8, c: i8) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.xop.vpcomw"]
        pub fn xop_vpcomw(a: ::simdty::i16x8, b: ::simdty::i16x8, c: i8) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.xop.vpermil2pd"]
        pub fn xop_vpermil2pd(a: ::simdty::f64x2, b: ::simdty::f64x2, c: ::simdty::f64x2, d: i8) -> ::simdty::f64x2;
        #[link_name = "llvm.x86.xop.vpermil2pd.256"]
        pub fn xop_vpermil2pd_256(a: ::simdty::f64x4, b: ::simdty::f64x4, c: ::simdty::f64x4, d: i8) -> ::simdty::f64x4;
        #[link_name = "llvm.x86.xop.vpermil2ps"]
        pub fn xop_vpermil2ps(a: ::simdty::f32x4, b: ::simdty::f32x4, c: ::simdty::f32x4, d: i8) -> ::simdty::f32x4;
        #[link_name = "llvm.x86.xop.vpermil2ps.256"]
        pub fn xop_vpermil2ps_256(a: ::simdty::f32x8, b: ::simdty::f32x8, c: ::simdty::f32x8, d: i8) -> ::simdty::f32x8;
        #[link_name = "llvm.x86.xop.vphaddbd"]
        pub fn xop_vphaddbd(a: ::simdty::i8x16) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.xop.vphaddbq"]
        pub fn xop_vphaddbq(a: ::simdty::i8x16) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.xop.vphaddbw"]
        pub fn xop_vphaddbw(a: ::simdty::i8x16) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.xop.vphadddq"]
        pub fn xop_vphadddq(a: ::simdty::i32x4) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.xop.vphaddubd"]
        pub fn xop_vphaddubd(a: ::simdty::i8x16) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.xop.vphaddubq"]
        pub fn xop_vphaddubq(a: ::simdty::i8x16) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.xop.vphaddubw"]
        pub fn xop_vphaddubw(a: ::simdty::i8x16) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.xop.vphaddudq"]
        pub fn xop_vphaddudq(a: ::simdty::i32x4) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.xop.vphadduwd"]
        pub fn xop_vphadduwd(a: ::simdty::i16x8) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.xop.vphadduwq"]
        pub fn xop_vphadduwq(a: ::simdty::i16x8) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.xop.vphaddwd"]
        pub fn xop_vphaddwd(a: ::simdty::i16x8) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.xop.vphaddwq"]
        pub fn xop_vphaddwq(a: ::simdty::i16x8) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.xop.vphsubbw"]
        pub fn xop_vphsubbw(a: ::simdty::i8x16) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.xop.vphsubdq"]
        pub fn xop_vphsubdq(a: ::simdty::i32x4) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.xop.vphsubwd"]
        pub fn xop_vphsubwd(a: ::simdty::i16x8) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.xop.vpmacsdd"]
        pub fn xop_vpmacsdd(a: ::simdty::i32x4, b: ::simdty::i32x4, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.xop.vpmacsdqh"]
        pub fn xop_vpmacsdqh(a: ::simdty::i32x4, b: ::simdty::i32x4, c: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.xop.vpmacsdql"]
        pub fn xop_vpmacsdql(a: ::simdty::i32x4, b: ::simdty::i32x4, c: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.xop.vpmacssdd"]
        pub fn xop_vpmacssdd(a: ::simdty::i32x4, b: ::simdty::i32x4, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.xop.vpmacssdqh"]
        pub fn xop_vpmacssdqh(a: ::simdty::i32x4, b: ::simdty::i32x4, c: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.xop.vpmacssdql"]
        pub fn xop_vpmacssdql(a: ::simdty::i32x4, b: ::simdty::i32x4, c: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.xop.vpmacsswd"]
        pub fn xop_vpmacsswd(a: ::simdty::i16x8, b: ::simdty::i16x8, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.xop.vpmacssww"]
        pub fn xop_vpmacssww(a: ::simdty::i16x8, b: ::simdty::i16x8, c: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.xop.vpmacswd"]
        pub fn xop_vpmacswd(a: ::simdty::i16x8, b: ::simdty::i16x8, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.xop.vpmacsww"]
        pub fn xop_vpmacsww(a: ::simdty::i16x8, b: ::simdty::i16x8, c: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.xop.vpmadcsswd"]
        pub fn xop_vpmadcsswd(a: ::simdty::i16x8, b: ::simdty::i16x8, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.xop.vpmadcswd"]
        pub fn xop_vpmadcswd(a: ::simdty::i16x8, b: ::simdty::i16x8, c: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.xop.vpperm"]
        pub fn xop_vpperm(a: ::simdty::i8x16, b: ::simdty::i8x16, c: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.x86.xop.vprotb"]
        pub fn xop_vprotb(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.x86.xop.vprotbi"]
        pub fn xop_vprotbi(a: ::simdty::i8x16, b: i8) -> ::simdty::i8x16;
        #[link_name = "llvm.x86.xop.vprotd"]
        pub fn xop_vprotd(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.xop.vprotdi"]
        pub fn xop_vprotdi(a: ::simdty::i32x4, b: i8) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.xop.vprotq"]
        pub fn xop_vprotq(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.xop.vprotqi"]
        pub fn xop_vprotqi(a: ::simdty::i64x2, b: i8) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.xop.vprotw"]
        pub fn xop_vprotw(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.xop.vprotwi"]
        pub fn xop_vprotwi(a: ::simdty::i16x8, b: i8) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.xop.vpshab"]
        pub fn xop_vpshab(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.x86.xop.vpshad"]
        pub fn xop_vpshad(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.xop.vpshaq"]
        pub fn xop_vpshaq(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.xop.vpshaw"]
        pub fn xop_vpshaw(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.xop.vpshlb"]
        pub fn xop_vpshlb(a: ::simdty::i8x16, b: ::simdty::i8x16) -> ::simdty::i8x16;
        #[link_name = "llvm.x86.xop.vpshld"]
        pub fn xop_vpshld(a: ::simdty::i32x4, b: ::simdty::i32x4) -> ::simdty::i32x4;
        #[link_name = "llvm.x86.xop.vpshlq"]
        pub fn xop_vpshlq(a: ::simdty::i64x2, b: ::simdty::i64x2) -> ::simdty::i64x2;
        #[link_name = "llvm.x86.xop.vpshlw"]
        pub fn xop_vpshlw(a: ::simdty::i16x8, b: ::simdty::i16x8) -> ::simdty::i16x8;
        #[link_name = "llvm.x86.xtest"]
        pub fn xtest() -> i32;
    }
}
/// LLVM intrinsics for the xcore architecture.
pub mod xcore {
    extern {
        #[link_name = "llvm.xcore.bitrev"]
        pub fn bitrev(a: i32) -> i32;
        #[link_name = "llvm.xcore.checkevent"]
        pub fn checkevent(a: *mut i8) -> *mut i8;
        #[link_name = "llvm.xcore.chkct"]
        pub fn chkct(a: *mut i8, b: i32) -> ();
        #[link_name = "llvm.xcore.clre"]
        pub fn clre() -> ();
        #[link_name = "llvm.xcore.clrpt"]
        pub fn clrpt(a: *mut i8) -> ();
        #[link_name = "llvm.xcore.clrsr"]
        pub fn clrsr(a: i32) -> ();
        #[link_name = "llvm.xcore.crc32"]
        pub fn crc32(a: i32, b: i32, c: i32) -> i32;
        #[link_name = "llvm.xcore.edu"]
        pub fn edu(a: *mut i8) -> ();
        #[link_name = "llvm.xcore.eeu"]
        pub fn eeu(a: *mut i8) -> ();
        #[link_name = "llvm.xcore.endin"]
        pub fn endin(a: *mut i8) -> i32;
        #[link_name = "llvm.xcore.freer"]
        pub fn freer(a: *mut i8) -> ();
        #[link_name = "llvm.xcore.geted"]
        pub fn geted() -> i32;
        #[link_name = "llvm.xcore.getet"]
        pub fn getet() -> i32;
        #[link_name = "llvm.xcore.getid"]
        pub fn getid() -> i32;
        #[link_name = "llvm.xcore.getps"]
        pub fn getps(a: i32) -> i32;
        #[link_name = "llvm.xcore.getr"]
        pub fn getr(a: i32) -> *mut i8;
        #[link_name = "llvm.xcore.getst"]
        pub fn getst(a: *mut i8) -> *mut i8;
        #[link_name = "llvm.xcore.getts"]
        pub fn getts(a: *mut i8) -> i32;
        #[link_name = "llvm.xcore.in"]
        pub fn in_(a: *mut i8) -> i32;
        #[link_name = "llvm.xcore.inct"]
        pub fn inct(a: *mut i8) -> i32;
        #[link_name = "llvm.xcore.initcp"]
        pub fn initcp(a: *mut i8, b: *mut i8) -> ();
        #[link_name = "llvm.xcore.initdp"]
        pub fn initdp(a: *mut i8, b: *mut i8) -> ();
        #[link_name = "llvm.xcore.initlr"]
        pub fn initlr(a: *mut i8, b: *mut i8) -> ();
        #[link_name = "llvm.xcore.initpc"]
        pub fn initpc(a: *mut i8, b: *mut i8) -> ();
        #[link_name = "llvm.xcore.initsp"]
        pub fn initsp(a: *mut i8, b: *mut i8) -> ();
        #[link_name = "llvm.xcore.inshr"]
        pub fn inshr(a: *mut i8, b: i32) -> i32;
        #[link_name = "llvm.xcore.int"]
        pub fn int(a: *mut i8) -> i32;
        #[link_name = "llvm.xcore.mjoin"]
        pub fn mjoin(a: *mut i8) -> ();
        #[link_name = "llvm.xcore.msync"]
        pub fn msync(a: *mut i8) -> ();
        #[link_name = "llvm.xcore.out"]
        pub fn out(a: *mut i8, b: i32) -> ();
        #[link_name = "llvm.xcore.outct"]
        pub fn outct(a: *mut i8, b: i32) -> ();
        #[link_name = "llvm.xcore.outshr"]
        pub fn outshr(a: *mut i8, b: i32) -> i32;
        #[link_name = "llvm.xcore.outt"]
        pub fn outt(a: *mut i8, b: i32) -> ();
        #[link_name = "llvm.xcore.peek"]
        pub fn peek(a: *mut i8) -> i32;
        #[link_name = "llvm.xcore.setc"]
        pub fn setc(a: *mut i8, b: i32) -> ();
        #[link_name = "llvm.xcore.setclk"]
        pub fn setclk(a: *mut i8, b: *mut i8) -> ();
        #[link_name = "llvm.xcore.setd"]
        pub fn setd(a: *mut i8, b: i32) -> ();
        #[link_name = "llvm.xcore.setev"]
        pub fn setev(a: *mut i8, b: *mut i8) -> ();
        #[link_name = "llvm.xcore.setps"]
        pub fn setps(a: i32, b: i32) -> ();
        #[link_name = "llvm.xcore.setpsc"]
        pub fn setpsc(a: *mut i8, b: i32) -> ();
        #[link_name = "llvm.xcore.setpt"]
        pub fn setpt(a: *mut i8, b: i32) -> ();
        #[link_name = "llvm.xcore.setrdy"]
        pub fn setrdy(a: *mut i8, b: *mut i8) -> ();
        #[link_name = "llvm.xcore.setsr"]
        pub fn setsr(a: i32) -> ();
        #[link_name = "llvm.xcore.settw"]
        pub fn settw(a: *mut i8, b: i32) -> ();
        #[link_name = "llvm.xcore.setv"]
        pub fn setv(a: *mut i8, b: *mut i8) -> ();
        #[link_name = "llvm.xcore.sext"]
        pub fn sext(a: i32, b: i32) -> i32;
        #[link_name = "llvm.xcore.ssync"]
        pub fn ssync() -> ();
        #[link_name = "llvm.xcore.syncr"]
        pub fn syncr(a: *mut i8) -> ();
        #[link_name = "llvm.xcore.testct"]
        pub fn testct(a: *mut i8) -> i32;
        #[link_name = "llvm.xcore.testwct"]
        pub fn testwct(a: *mut i8) -> i32;
        #[link_name = "llvm.xcore.waitevent"]
        pub fn waitevent() -> *mut i8;
        #[link_name = "llvm.xcore.zext"]
        pub fn zext(a: i32, b: i32) -> i32;
    }
}
