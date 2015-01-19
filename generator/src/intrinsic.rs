use std::fmt;
use std::num::Int;
use std::str::FromStr;

#[derive(Show, PartialEq, Eq, Clone)]
pub enum LLVMType {
    Anon,
    Int(Option<usize>),
    Float(Option<usize>),
    FixedPoint(usize),
    Ptr(Box<LLVMType>),

    Vector(Option<(usize, Box<LLVMType>)>),
    Metadata,
    Vararg,
    Descriptor,
    X86mmx,
    Mips(Box<LLVMType>)
}

fn int(x: usize) -> LLVMType { LLVMType::Int(Some(x)) }
fn float(x: usize) -> LLVMType { LLVMType::Float(Some(x)) }
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
        if s.starts_with("anonymous_") { return Some(LLVMType::Anon) }

        if s.starts_with("mips_") && s.ends_with("_ty") {
            return parse_internals(&s["mips_".len()..s.len()-"_ty".len()])
                .map(|t| LLVMType::Mips(Box::new(t)))
        }
        else if !(s.starts_with("llvm_") && s.ends_with("_ty")) { return None }

        parse_internals(&s["llvm_".len()..s.len() - "_ty".len()])
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
    pub llvm_name: String,
    pub target_prefix: String,
    pub params: Vec<LLVMType>,
    pub ret: Vec<LLVMType>,
}

impl FromStr for Intrinsic {
    fn from_str(s: &str) -> Option<Intrinsic> {
        let main = regex!(r"(?ism)def\s*(?P<name>int_[a-z_0-9]*)\s*\{.*?$(?P<contents>[^}]*)\}\s*");
        let arch = regex!(r"^int_([^_]*)");
        let gcc_name = regex!(r#"GCCBuiltinName = "([^"]*)""#); // "]))
        let llvm_name = regex!(r#"LLVMName = "([^"]*)""#); // "]))
        let target_prefix = regex!(r#"TargetPrefix = "([^"]*)""#); // "]))
        let params = regex!(r#"ParamTypes = \[([^\]]*)\]"#); // "
        let ret = regex!(r#"RetTypes = \[([^\]]*)\]"#); // "

        let caps = try_opt!(main.captures(s));
        let name = caps.name("name").unwrap();
        let arch = arch.captures(name).unwrap().at(1).unwrap().parse();
        let contents = caps.name("contents").unwrap();

        let gcc_name = gcc_name.captures(contents).map(|c| c.at(1).unwrap().to_string());
        let llvm_name = try_opt!(llvm_name.captures(contents)).at(1).unwrap();
        let target_prefix = try_opt!(target_prefix.captures(contents)).at(1).unwrap();
        let params = try_opt!(params.captures(contents)).at(1).unwrap();
        let params =
            try_opt!(params.split(',')
                     .map(|s| s.trim()).filter(|s| !s.is_empty())
                     .map(|s| s.parse()).collect());
        let ret = try_opt!(ret.captures(contents)).at(1).unwrap();
        let ret =
            try_opt!(ret.split(',')
                     .map(|s| s.trim()).filter(|s| !s.is_empty())
                     .map(|s| s.parse()).collect());
        Some(Intrinsic {
            name: name.to_string(),
            arch: arch,
            gcc_name: gcc_name,
            llvm_name: llvm_name.to_string(),
            target_prefix: target_prefix.to_string(),
            params: params,
            ret: ret,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{LLVMType, Intrinsic};

    #[test]
    fn llvm_type_parse() {
        assert_eq!("llvm_v8i16_ty".parse(),
                   Some(LLVMType::Vector(Some((8, Box::new(LLVMType::Int(Some(16))))))));
    }

    #[test]
    fn llvm_intrinsic_parse() {
        let s = r#"def int_x86_xop_vpshlw {	// GCCBuiltin SDPatternOperator Intrinsic
  string GCCBuiltinName = "__builtin_ia32_vpshlw";
  string Intrinsic:name = "";
  string LLVMName = "";
  string TargetPrefix = "";
  list<LLVMType> RetTypes = [llvm_v8i16_ty];
  list<LLVMType> ParamTypes = [llvm_v8i16_ty, llvm_v8i16_ty];
  list<IntrinsicProperty> Properties = [IntrNoMem];
  bit isTarget = 0;
  string NAME = ?;
}"#;
        let int = s.parse::<Intrinsic>();
        assert_eq!(int, Some(Intrinsic {
            name: "int_x86_xop_vpshlw".to_string(),
            gcc_name: "__builtin_ia32_vpshlw".to_string(),
            llvm_name: "".to_string(),
            target_prefix: "".to_string(),
            ret: vec![LLVMType::Vector(Some((8, Box::new(LLVMType::Int(Some(16))))))],
            params: vec![
                LLVMType::Vector(Some((8, Box::new(LLVMType::Int(Some(16)))))),
                LLVMType::Vector(Some((8, Box::new(LLVMType::Int(Some(16))))))
                    ]
        }));
    }

}
