#![feature(plugin, slice_patterns, str_char)]
#![plugin(regex_macros)]
extern crate regex;

use std::io;
use std::io::prelude::*;
use std::collections::{BTreeMap, btree_map};
use std::path::Path;

macro_rules! try_opt {
    ($e: expr) => {  match $e { Some(x) => x, None => { return None } } }
}

mod intrinsic;
mod ast;
fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();

    let ast = ast::parse(&s, Path::new(""));
    let (classes, mut defs) = ast::flatten_separate(ast);
    let class_names = ast::classes_by_name(&classes);
    ast::resolve_classes(&mut defs, &class_names);

    let mut modules = BTreeMap::new();
    for d in defs.iter() {
        let intr = match intrinsic::Intrinsic::from_ast(d) {
            None if !d.name.starts_with("int_") => continue,
            None => panic!("failed to parse: {:?}", d),
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
            let sigs = intr.signatures();

            for &(ref suffix, ref sig) in sigs.iter() {

                let mut link_name = intr.llvm_name.clone()
                    .unwrap_or_else(|| format!("llvm.{}",
                                               intr.name["int_".len()..].replace("_", ".")));
                link_name.push_str(&suffix);

                let raw_name = format!("{}{}", &intr.name[strip..], suffix.replace(".", "_"));
                let fn_name = avoid_keywords(&raw_name);
                let mut docs = format!("The `{}` intrinsic", link_name);
                if let Some(ref name) = intr.gcc_name {
                    docs.push_str("; known as `");
                    docs.push_str(&name);
                    docs.push_str("` in GCC");

                    (match gcc_reexports.entry(&*name) {
                        btree_map::Entry::Occupied(o) => o.into_mut(),
                        btree_map::Entry::Vacant(v) => v.insert(vec![])
                    }).push((*module, fn_name.to_string()))
                }
                docs.push_str(".");

                println!("\
{indent}/// {docs}
{indent}#[link_name = \"{link_name}\"]
{indent}pub fn {fn_name}{sig};",
                         indent = indent,
                         docs = docs,
                         link_name = link_name,
                         fn_name = fn_name,
                         sig = sig);
            }
        }

        println!("{}", close);
    }

    println!("\
/// Listing of the corresponding name(s) of many GCC intrinsics, for reference/search purposes.
///
/// <dl>");
    for (gcc_name, locations) in gcc_reexports.iter() {
        println!("/// <dt><strong><code>{}</code></strong></dt>", gcc_name);
        for &(module, ref fn_name) in locations.iter() {
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
