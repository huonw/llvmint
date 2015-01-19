#![feature(plugin)]
#![allow(unstable)]

extern crate regex;
#[plugin] #[no_link] extern crate regex_macros;

use std::collections::{BTreeMap, btree_map};

macro_rules! try_opt {
    ($e: expr) => {  match $e { Some(x) => x, None => return None } }
}

mod intrinsic;
fn main() {
    let s = std::io::stdin().read_to_string().unwrap();
    let defs = regex!(r"(?sm)def\sint_.*?^\}");

    let mut modules = BTreeMap::new();
    for capture in defs.captures_iter(&s[]) {
        let found = capture.at(0).unwrap();
        let intr = match found.parse::<intrinsic::Intrinsic>() {
            None => panic!("failed to parse {}", found),
            Some(intr) => intr
        };

        (match modules.entry(intr.arch) {
            btree_map::Entry::Occupied(o) => o.into_mut(),
            btree_map::Entry::Vacant(v) => v.insert(vec![])
        }).push(intr)
    }


    println!("\
#![feature(simd, simd_ffi, link_llvm_intrinsics)]
#![allow(non_snake_case)]

//! Bindings to (almost) all LLVM intrinsics.
//!
//! Intrinsics are categorised into modules by the architecture on
//! which they are supported (however, see [Platform
//! support](#platform-support) for a caveat), with certain intrinsics
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
//! Many of these intrinsics have corresponding intrinsics exposed by
//! GCC/Clang in C/C++, these names are listed in
//! [`gcc_names`](gcc_names/index.html).
//!
//! # Platform support
//!
//! An intrinsic being available in a certain module (or at the top
//! level) does not guarantee that it is supported on all CPUs of that
//! architecture (resp. all CPUs), e.g. the `x86::avx512_...`
//! intrinsics are only supported on the very latest x86-64 CPUs, not
//! on older x86 processors.
//!
//! Using an intrinsic in a configuration that is not supported will
//! likely cause LLVM assertions or general badness along those lines.

extern crate simdty;
");

    let mut gcc_reexports = BTreeMap::new();
    for (module, items) in modules.iter() {
        let (indent, close, strip) = match *module {
            None => {
                println!("extern {{");
                ("    ",
                 "}",
                 "int_".len())
            }
            Some(arch) => {
                println!("\
/// LLVM intrinsics for the {arch} architecture.
pub mod {arch} {{
    extern {{", arch=arch);
                ("        ",
                 "    }\n}",
                 "int_".len() + arch.as_str().len() + 1)
            }
        };

        for intr in items.iter() {
            let params = intr.params.iter()
                .enumerate()
                .map(|(i, ty)| {
                    ty.to_concrete_rust_string()
                        .map(|s| {
                            if s == "..." {
                                s
                            } else {
                                format!("{}: {}", (b'a' + i as u8) as char, s)
                            }
                        })
                })
                .collect::<Option<Vec<_>>>();
            let p = match params {
                Some(p) => p.connect(", "),
                None => continue
            };
            let ret = match &intr.ret[] {
                [] => "()".to_string(),
                [ref ret] => match ret.to_concrete_rust_string() {
                    Some(r) => r,
                    None => continue
                },
                _ => continue
            };
            let link_name = format!("llvm.{}", intr.name["int_".len()..].replace("_", "."));
            let fn_name = avoid_keywords(&intr.name[strip..]);
            let mut docs = format!("The `{}` intrinsic", link_name);
            if let Some(ref name) = intr.gcc_name {
                docs.push_str("; known as `");
                docs.push_str(&name[]);
                docs.push_str("` in GCC");

                (match gcc_reexports.entry(&name[]) {
                    btree_map::Entry::Occupied(o) => o.into_mut(),
                    btree_map::Entry::Vacant(v) => v.insert(vec![])
                }).push((*module, fn_name))
            }
            docs.push_str(".");

            println!("\
{indent}/// {docs}
{indent}#[link_name = \"{link_name}\"]
{indent}pub fn {fn_name}({params}) -> {ret};",
                     indent = indent,
                     docs = docs,
                     link_name = link_name,
                     fn_name = fn_name,
                     params = p,
                     ret = ret);
        }

        println!("{}", close);
    }

    println!("\
/// Listing of the corresponding name(s) of many GCC intrinsics, for reference/search purposes.
///
/// <dl>");
    for (gcc_name, locations) in gcc_reexports.iter() {
        println!("/// <dt><strong><code>{}</code></strong></dt>", gcc_name);
        for &(module, fn_name) in locations.iter() {
            println!("\
/// <dd><a href=\"../{module}{url_sep}fn.{fn_name}.html\"><code>{module}{mod_sep}{fn_name}</code></a></dd>",
                     module = module.map(|a| a.as_str()).unwrap_or(""),
                     url_sep = if module.is_some() {"/"} else {""},
                     mod_sep = if module.is_some() {"::"} else {""},
                     fn_name = fn_name)
        }
    }
println!("\
/// </dl>
pub mod gcc_names {{}}");
}

fn avoid_keywords(s: &str) -> &str {
    match s {
        "in" => "in_",
        _ => s
    }
}
