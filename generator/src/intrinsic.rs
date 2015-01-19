use std::fmt;
use std::num::Int;
use std::str::FromStr;

use ast;

#[derive(Show, PartialEq, Eq, Clone)]
pub enum MatchStyle {
    Direct, Extend, Truncate
}

#[derive(Show, PartialEq, Eq, Clone)]
pub enum LLVMType {
    Anon,
    Int(Option<u32>),
    Float(Option<u32>),
    FixedPoint(u32),
    Ptr(Box<LLVMType>),

    Vector(Option<(u32, Box<LLVMType>)>),
    Metadata,
    Vararg,
    Descriptor,
    X86mmx,
    Mips(Box<LLVMType>),

    MatchedType(u32, MatchStyle),
}

fn int(x: u32) -> LLVMType { LLVMType::Int(Some(x)) }
fn float(x: u32) -> LLVMType { LLVMType::Float(Some(x)) }
fn ptr(ty: LLVMType) -> LLVMType { LLVMType::Ptr(Box::new(ty)) }

fn parse_internals(s: &str) -> Option<LLVMType> {
    match s {
        "float" => return Some(float(32)),
        "double" => return Some(float(64)),
        "anyvector" => return Some(LLVMType::Vector(None)),
        "anyfloat" => return Some(LLVMType::Float(None)),
        "anyint" => return Some(LLVMType::Int(None)),
        "anyptr" | "ptr" => return Some(ptr(int(8))),
        "ptrptr" => return Some(ptr(ptr(int(8)))),
        "anyi64ptr" => return Some(ptr(int(64))),
        "metadata" => return Some(LLVMType::Metadata),
        "vararg" => return Some(LLVMType::Vararg),
        "descriptor" => return Some(LLVMType::Descriptor),
        "x86mmx" => return Some(LLVMType::X86mmx),
        "ptrx86mmx" => return Some(ptr(LLVMType::X86mmx)),
        _ => {}
    }
    if s.starts_with("i") {
        s[1..].parse().map(int)
    } else if s.starts_with("f") {
        s[1..].parse().map(float)
    } else if s.starts_with("q") {
        s[1..].parse().map(LLVMType::FixedPoint)
    } else if s.starts_with("v") {
        let v_len_idx = 1 + s[1..].chars().take_while(|d| d.is_digit(10)).count();
        s[1..v_len_idx].parse().and_then(|n| {
            parse_internals(&s[v_len_idx..]).map(|t| LLVMType::Vector(Some((n, Box::new(t)))))
        })
    } else {
        println!("unrecognised {}", s);
        None
    }
}

impl FromStr for LLVMType {
    fn from_str(s: &str) -> Option<LLVMType> {
        if s.starts_with("mips_") && s.ends_with("_ty") {
            return parse_internals(&s["mips_".len()..s.len()-"_ty".len()])
                .map(|t| LLVMType::Mips(Box::new(t)))
        }
        parse_internals(&s["llvm_".len()..s.len() - "_ty".len()])
            .or_else(|| parse_internals(s))
    }
}

impl LLVMType {
    fn from_ast(t: &ast::Type) -> Option<LLVMType> {
        if t.args.is_empty() {
            t.name.parse()
        } else {
            let style = match &t.name[] {
                "LLVMMatchType" => MatchStyle::Direct,
                "LLVMExtendedType" => MatchStyle::Extend,
                "LLVMTruncatedType" => MatchStyle::Truncate,
                "LLVMAnyPointerType" => {
                    return match t.args[0] {
                        ast::Val::Type(ref t) => LLVMType::from_ast(t).map(ptr),
                        _ => None
                    }
                }
                _ => return None
            };
            let n  = match t.args[0] {
                ast::Val::Int(n) => n,
                _ => return None,
            };
            Some(LLVMType::MatchedType(n, style))
        }
    }
}

impl LLVMType {
    pub fn to_concrete_rust_string(&self) -> Option<String> {
        match *self {
            LLVMType::Int(Some(1)) => Some("bool".to_string()),
            LLVMType::Int(Some(n)) => Some(format!("i{}", n)),
            LLVMType::Float(Some(n)) => Some(format!("f{}", n)),
            LLVMType::Ptr(ref ty) => ty.to_concrete_rust_string().map(|s| format!("*mut {}", s)),
            LLVMType::Vararg => Some("...".to_string()),
            LLVMType::Mips(ref ty) => ty.to_concrete_rust_string(),

            LLVMType::Vector(Some((n, ref ty))) => {
                let (name, size) = match **ty {
                    LLVMType::Int(Some(n)) if n == 8 || n == 16 || n == 32 || n == 64 => ("i", n),
                    LLVMType::Float(Some(n)) if n == 32 || n == 64 => ("f", n),
                    _ => return None
                };
                if size.count_ones() == 1 && 2 <= size && size <= 64 {
                    Some(format!("::simdty::{}{}x{}", name, size, n))
                } else {
                    None
                }
            }

            _ => None
        }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Show, PartialOrd, Ord)]
pub enum Arch {
    AMDGPU,
    Aarch64,
    Arm,
    Cuda,
    Hexagon,
    Mips,
    Nvvm,
    Ppc,
    Ptx,
    R600,
    X86,
    Xcore,
}
impl Arch {
    pub fn as_str(&self) -> &'static str {
        match *self {
            Arch::AMDGPU => "AMDGPU",
            Arch::Aarch64 => "aarch64",
            Arch::Arm => "arm",
            Arch::Cuda => "cuda",
            Arch::Hexagon => "hexagon",
            Arch::Mips => "mips",
            Arch::Nvvm => "nvvm",
            Arch::Ppc => "ppc",
            Arch::Ptx => "ptx",
            Arch::R600 => "r600",
            Arch::X86 => "x86",
            Arch::Xcore => "xcore",
        }
    }
}
impl FromStr for Arch {
    fn from_str(s: &str) -> Option<Arch> {
        Some(match s {
            "AMDGPU" => Arch::AMDGPU,
            "aarch64" => Arch::Aarch64,
            "arm" => Arch::Arm,
            "cuda" => Arch::Cuda,
            "hexagon" => Arch::Hexagon,
            "mips" => Arch::Mips,
            "nvvm" => Arch::Nvvm,
            "ppc" => Arch::Ppc,
            "ptx" => Arch::Ptx,
            "r600" => Arch::R600,
            "x86" => Arch::X86,
            "xcore" => Arch::Xcore,
            _ => return None
        })
    }
}
impl fmt::String for Arch {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result { fmt::String::fmt(self.as_str(), fmt) }
}

#[derive(Show, PartialEq, Eq)]
pub struct Intrinsic {
    pub arch: Option<Arch>,
    pub name: String,
    pub gcc_name: Option<String>,
    pub llvm_name: Option<String>,
    pub params: Vec<LLVMType>,
    pub ret: Vec<LLVMType>,
}

impl Intrinsic {
    pub fn from_ast(d: &ast::Def) -> Option<Intrinsic> {
        if !d.name.starts_with("int_") { return None }
        let arch = regex!(r"^int_([^_]*)");
        let arch = arch.captures(&d.name[]).unwrap().at(1).unwrap().parse();

        let mut gcc_name = None;
        let mut llvm_name = None;
        let mut ret = vec![];
        let mut params = vec![];
        for sup in d.inherits.iter() {
            match &sup.name[] {
                "GCCBuiltin" => {
                    match sup.args[0] {
                        ast::Val::String(ref s) => {
                            if !s.is_empty() {
                                gcc_name = Some(s.clone())
                            }
                        }
                        _ => return None
                    }
                }
                "Intrinsic" => {
                    match sup.args[0] {
                        ast::Val::List(ref ret_) => {
                            ret = try_opt!(ret_.iter()
                                .map(|v| match *v {
                                    ast::Val::Type(ref t) => LLVMType::from_ast(t),
                                    _ => None
                                })
                                .collect::<Option<_>>())
                        }
                        _ => return None
                    }
                    match sup.args[1] {
                        ast::Val::List(ref params_) => {
                            params = try_opt!(params_.iter()
                                .map(|v| match *v {
                                    ast::Val::Type(ref t) => LLVMType::from_ast(t),
                                    _ => None
                                })
                                .collect::<Option<_>>())
                        }
                        _ => return None
                    }
                    match sup.args[3] {
                        ast::Val::String(ref s) => {
                            if !s.is_empty() { llvm_name = Some(s.clone()) }
                        }
                        _ => return None
                    }
                }
                _ => {}
            }
        }

        Some(Intrinsic {
            arch: arch,
            name: d.name.clone(),
            gcc_name: gcc_name,
            llvm_name: llvm_name,
            ret: ret,
            params: params,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{LLVMType, Intrinsic, Arch};

    #[test]
    fn llvm_type_parse() {
        assert_eq!("llvm_v8i16_ty".parse(),
                   Some(LLVMType::Vector(Some((8, Box::new(LLVMType::Int(Some(16))))))));
    }


}
