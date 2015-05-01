use std::fmt;
use std::str::FromStr;

use ast;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MatchStyle {
    Direct, Extend, Truncate
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LLVMType {
    Int(Option<u32>),
    Float(Option<u32>),
    FixedPoint(u32),
    Ptr(Option<Box<LLVMType>>),

    Vector(Option<(u32, Box<LLVMType>)>),
    Metadata,
    Vararg,
    Descriptor,
    X86mmx,
    Mips(Box<LLVMType>),

    MatchedType(u32, MatchStyle),
}


enum TypeKind {
    Generic, Matched(u32), Concrete,
}

fn int(x: u32) -> LLVMType { LLVMType::Int(Some(x)) }
fn float(x: u32) -> LLVMType { LLVMType::Float(Some(x)) }
fn ptr(ty: LLVMType) -> LLVMType { LLVMType::Ptr(Some(Box::new(ty))) }

fn parse_internals(s: &str) -> Result<LLVMType, ()> {
    match s {
        "float" => return Ok(float(32)),
        "double" => return Ok(float(64)),
        "anyvector" => return Ok(LLVMType::Vector(None)),
        "anyfloat" => return Ok(LLVMType::Float(None)),
        "anyint" => return Ok(LLVMType::Int(None)),
        "anyptr" => return Ok(LLVMType::Ptr(None)),
        "ptr" => return Ok(ptr(int(8))),
        "ptrptr" => return Ok(ptr(ptr(int(8)))),
        "anyi64ptr" => return Ok(ptr(int(64))),
        "metadata" => return Ok(LLVMType::Metadata),
        "vararg" => return Ok(LLVMType::Vararg),
        "descriptor" => return Ok(LLVMType::Descriptor),
        "x86mmx" => return Ok(LLVMType::X86mmx),
        "ptrx86mmx" => return Ok(ptr(LLVMType::X86mmx)),
        _ => {}
    }
    if s.starts_with("i") {
        s[1..].parse().map(int).map_err(|_| ())
    } else if s.starts_with("f") {
        s[1..].parse().map(float).map_err(|_| ())
    } else if s.starts_with("q") {
        s[1..].parse().map(LLVMType::FixedPoint).map_err(|_| ())
    } else if s.starts_with("v") {
        let v_len_idx = 1 + s[1..].chars().take_while(|d| d.is_digit(10)).count();
        s[1..v_len_idx].parse().map_err(|_| ()).and_then(|n| {
            parse_internals(&s[v_len_idx..]).map(|t| LLVMType::Vector(Some((n, Box::new(t)))))
        })
    } else {
        println!("unrecognised {}", s);
        Err(())
    }
}

impl FromStr for LLVMType {
    type Err = ();
    fn from_str(s: &str) -> Result<LLVMType, ()> {
        if s.starts_with("mips_") && s.ends_with("_ty") {
            return parse_internals(&s["mips_".len()..s.len()-"_ty".len()])
                .map(|t| LLVMType::Mips(Box::new(t)))
        }
        parse_internals(&s["llvm_".len()..s.len() - "_ty".len()])
            .or_else(|_| parse_internals(s))
    }
}

impl LLVMType {
    fn from_ast(t: &ast::Type) -> Option<LLVMType> {
        if t.args.is_empty() {
            t.name.parse().ok()
        } else {
            let style = match &*t.name {
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

    fn kind(&self) -> TypeKind {
        match *self {
            LLVMType::MatchedType(n, _) => TypeKind::Matched(n),
            LLVMType::Vector(None) | LLVMType::Int(None) |
            LLVMType::Float(None) | LLVMType::Ptr(None)
                => TypeKind::Generic,
            LLVMType::Vector(Some((_, ref ty))) |
                LLVMType::Ptr(Some(ref ty)) | LLVMType::Mips(ref ty) => ty.kind(),

            _ => TypeKind::Concrete
        }
    }

    fn string(&self, dot: bool) -> String {
        let dot = if dot {"."} else {""};
        match *self {
            LLVMType::Int(Some(n)) => format!("{}i{}", dot, n),
            LLVMType::Float(Some(n)) => format!("{}f{}", dot, n),
            LLVMType::Vector(Some((n, ref ty))) => format!("{}v{}{}", dot, n, ty.string(false)),
            LLVMType::Ptr(Some(ref ty)) => format!("{}p0{}", dot, ty.string(false)),
            _ => panic!("unsupported string {:?}", self),
        }
    }

    fn choices(&self, generics: &[(usize, bool)],
               r: &[LLVMType], p: &[LLVMType]) -> (bool, Vec<LLVMType>) {

        fn vectorify(tys: &[LLVMType], include_one: bool) -> Vec<LLVMType> {
            let count = 1 + if include_one {1} else {0};
            let mut ret = Vec::with_capacity(count * tys.len());

            for ty in tys.iter() {
                let width = match *ty {
                    LLVMType::Int(Some(n)) | LLVMType::Float(Some(n)) => n,
                    _ => panic!("invalid vectorification {:?}", ty)
                };
                ret.push(LLVMType::Vector(Some((128 / width, Box::new(ty.clone())))));
                if include_one {
                    ret.push(ty.clone())
                }
            }
            return ret
            /*let lens = [2, 4, 8];
            let mut ret = Vec::with_capacity((lens.len() + 1) * tys.len());
            ret.extend(tys.iter().cloned());

            for &len in lens.iter() {
                for ty in tys.iter() {
                    ret.push(LLVMType::Vector(Some((len, Box::new(ty.clone())))))
                }
            }
            ret*/
        }

        match *self {
            LLVMType::MatchedType(n, style) => {
                if style != MatchStyle::Direct { return (false, vec![]) }

                let (idx, use_r) = generics[n as usize];
                (false, vec![if use_r {&r[idx]} else {&p[idx]}.clone()])
            }
            LLVMType::Int(None) => (true, vectorify(&[int(8), int(16), int(32), int(64)], true)),
            LLVMType::Float(None) => (true, vectorify(&[float(32), float(64)], true)),
            LLVMType::Vector(None) => {
                (true, vectorify(&[int(8), int(16), int(32), int(64), float(32), float(64)], false))
            }
            LLVMType::Vector(Some((n, ref ty))) => {
                let (generic, choices) = ty.choices(generics, r, p);

                (generic,
                 choices.into_iter()
                 .map(|ty| LLVMType::Vector(Some((n, Box::new(ty)))))
                 .collect())
            }
            LLVMType::Ptr(None) => {
                (true, vec![ptr(int(8))])
            }
            LLVMType::Ptr(Some(ref ty)) => {
                let (generic, choices) = ty.choices(generics, r, p);
                (generic, choices.into_iter().map(ptr).collect())
            }
            _ => (false, vec![self.clone()]),
        }
    }
}

impl LLVMType {
    pub fn to_concrete_rust_string(&self) -> Option<String> {
        match *self {
            LLVMType::Int(Some(1)) => Some("bool".to_string()),
            LLVMType::Int(Some(n)) => Some(format!("i{}", n)),
            LLVMType::Float(Some(n)) => Some(format!("f{}", n)),
            LLVMType::Ptr(Some(ref ty))
                => ty.to_concrete_rust_string().map(|s| format!("*mut {}", s)),
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

#[derive(PartialEq, Eq, Hash, Copy, Debug, PartialOrd, Ord, Clone)]
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
    type Err = ();
    fn from_str(s: &str) -> Result<Arch, ()> {
        Ok(match s {
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
            _ => return Err(())
        })
    }
}
impl fmt::Display for Arch {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result { fmt::Display::fmt(self.as_str(), fmt) }
}

#[derive(Debug, PartialEq, Eq)]
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
        let arch = arch.captures(&d.name).unwrap().at(1).unwrap().parse();

        let mut gcc_name = None;
        let mut llvm_name = None;
        let mut ret = vec![];
        let mut params = vec![];
        for sup in d.inherits.iter() {
            match &*sup.name {
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
            arch: arch.ok(),
            name: d.name.clone(),
            gcc_name: gcc_name,
            llvm_name: llvm_name,
            ret: ret,
            params: params,
        })
    }


    pub fn signatures(&self) -> Vec<(String, String)> {
        use std::iter::repeat;
        let mut generics = vec![];

        let ret_iter = self.ret.iter().zip(repeat(true).enumerate());
        let param_iter = self.params.iter().zip(repeat(false).enumerate());
        for (ty, pos) in ret_iter.chain(param_iter) {
            match ty.kind() {
                TypeKind::Generic => generics.push(pos),
                TypeKind::Matched(n) => assert!(generics.len() >= n as usize),
                TypeKind::Concrete => {}
            }
        }
        let mut used_ret = self.ret.clone();
        let mut used_params = self.params.clone();

        let mut sigs = vec![];
        choose_types(&mut sigs, &generics,
                     0, &self.ret,
                     0, &self.params,
                     &mut vec![],
                     &mut used_ret, &mut used_params);

        return sigs;


        fn choose_types(sigs: &mut Vec<(String, String)>,
                        generics: &[(usize, bool)],
                        ri: usize, ret: &[LLVMType],
                        pi: usize, params: &[LLVMType],
                        args: &mut Vec<String>,
                        used_ret: &mut [LLVMType], used_params: &mut [LLVMType]) {
            match ret.get(ri) {
                Some(rty) => {
                    let (generic, choices) = rty.choices(generics,
                                                         used_ret, used_params);
                    for choice in choices.into_iter() {
                        if generic {args.push(choice.string(true))};
                        used_ret[ri] = choice;
                        choose_types(sigs, generics,
                                     ri + 1, ret,
                                     pi, params,
                                     args,
                                     used_ret, used_params);
                        if generic {args.pop();};
                    }
                    return
                }
                None => match params.get(pi) {
                    Some(pty) => {
                        let (generic, choices) = pty.choices(generics,
                                                             used_ret, used_params);
                        for choice in choices.into_iter() {
                            if generic {args.push(choice.string(true))};
                            used_params[pi] = choice;
                            choose_types(sigs, generics,
                                         ri, ret,
                                         pi + 1, params,
                                         args,
                                         used_ret, used_params);
                            if generic {args.pop();}
                        }
                        return
                    }
                    None => {}
                }
            }

            let params = used_params.iter()
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
            let params = match params {
                Some(p) => p.connect(", "),
                None => return
            };

            let ret = match &*used_ret {
                [] => "()".to_string(),
                [ref ret] => match ret.to_concrete_rust_string() {
                    Some(r) => r,
                    None => return
                },
                _ => return
            };

            let sig = format!("({}) -> {}", params, ret);
            sigs.push((args.concat(), sig));
        }
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
