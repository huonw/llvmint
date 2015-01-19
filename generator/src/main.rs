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

extern crate simdty;
");
    for (module, items) in modules.iter() {
        let (indent, close, strip) = match *module {
            None => {
                println!("extern {{");
                ("    ", "}", "int_".len())
            }
            Some(arch) => {
                println!("\
/// LLVM intrinsics for the {arch} architecture.
pub mod {arch} {{
    extern {{", arch=arch);
                ("        ", "    }\n}", "int_".len() + arch.as_str().len() + 1)
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

            println!("\
{indent}#[link_name = \"llvm.{link_name}\"]
{indent}pub fn {fn_name}({params}) -> {ret};",
                     indent = indent,
                     link_name = intr.name["int_".len()..].replace("_", "."),
                     fn_name = avoid_keywords(&intr.name[strip..]),
                     params = p,
                     ret = ret);
        }

        println!("{}", close);
    }
}

fn avoid_keywords(s: &str) -> &str {
    match s {
        "in" => "in_",
        _ => s
    }
}
