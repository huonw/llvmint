use std::io::File;
use std::collections::HashMap;
use std::{cmp, mem};

#[derive(Show, Clone, PartialEq, Eq)]
enum Token {
    Ident(String),
    Int(u32),
    String(String),
    Spikey(Vec<Token>),
    Square(Vec<Token>),
    Braces(Vec<Token>),
    Parens(Vec<Token>),
    Colon,
    Comma,
    Semicolon,
    Equals,
    Bang,
}

fn tokenize(mut s: &str) -> Vec<Token> {
    let mut stack = vec![];
    let mut current = vec![];
    while !s.is_empty() && s.char_at(0).is_whitespace() {
        // will fail with non-ascii whitespace
        s = &s[1..]
    }

    while !s.is_empty() {
        let length = match s.char_at(0) {
            ':' => { current.push(Token::Colon); 1 },
            ',' => { current.push(Token::Comma); 1 },
            ';' => { current.push(Token::Semicolon); 1 },
            '=' => { current.push(Token::Equals); 1 },
            '!' => { current.push(Token::Bang); 1 }
            '"' /*"*/=> {
                let string = regex!("^\"(.*?)\"").captures(s).expect("bad string").at(1)
                    .unwrap();
                current.push(Token::String(string.to_string()));
                string.len() + 2
            }
            '0'...'9' => {
                let num = regex!("^[0-9]+").captures(s).expect("bad num").at(0).unwrap();
                current.push(Token::Int(num.parse().unwrap()));
                num.len()
            }
            '<' | '(' | '{' | '[' => {
                let old = mem::replace(&mut current, vec![]);
                stack.push(old);
                1
            }
            '>' => {
                let new = mem::replace(&mut current, stack.pop().expect("empty > stack?"));
                current.push(Token::Spikey(new));
                1
            }
            '}' => {
                let new = mem::replace(&mut current, stack.pop().expect("empty } stack?"));
                current.push(Token::Braces(new));
                1
            }
            ']' => {
                let new = mem::replace(&mut current, stack.pop().expect("empty ] stack?"));
                current.push(Token::Square(new));
                1
            }
            ')' => {
                let new = mem::replace(&mut current, stack.pop().expect("empty ) stack?"));
                current.push(Token::Parens(new));
                1
            }
            // comment
            '/' => {
                s.chars().take_while(|c| *c != '\n').count()
            }
            c if c.is_whitespace() => 0,
            _ => {
                let ident = match regex!("[A-Za-z0-9_]+").captures(s) {
                    Some(i) => i.at(0).unwrap(),
                    None => panic!("invalid token stream, {}...", &s[..cmp::max(10, s.len())])
                };
                current.push(Token::Ident(ident.to_string()));
                ident.len()
            }
        };
        s = &s[length..];
        while !s.is_empty() && s.char_at(0).is_whitespace() {
            // will fail with non-ascii whitespace
            s = &s[1..]
        }
    }

    assert!(stack.is_empty());
    current
}

#[derive(Clone, Show)]
pub enum Val {
    List(Vec<Val>),
    Strconcat(Vec<Val>),
    Type(Type),
    String(String),
    Int(u32),
}
#[derive(Clone, Show)]
pub struct Type {
    pub name: String,
    pub args: Vec<Val>
}

#[derive(Clone, Show)]
pub struct Class {
    pub name: String,
    pub args: Vec<(Type, String, Option<Val>)>,
    pub inherits: Vec<Type>
}

#[derive(Clone, Show)]
pub struct Def {
    pub name: String,
    pub inherits: Vec<Type>,
}
#[derive(Clone, Show)]
pub struct Let {
    pub items: Vec<Item>,
}

#[derive(Clone, Show)]
pub struct Include {
    items: Vec<Item>,
}

#[derive(Clone, Show)]
pub enum Item {
    Class(Class),
    Multiclass(Class),
    Def(Def),
    Defm(Def),
    Let(Let),
    Include(Include),
}

pub fn flatten_separate(items: Vec<Item>) -> (Vec<Class>, Vec<Def>) {
    let mut classes = vec![];
    let mut defs = vec![];
    for it in items.into_iter() {
        match it {
            Item::Let(Let { items }) | Item::Include(Include { items }) => {
                let (c, d) = flatten_separate(items);
                classes.extend(c.into_iter());
                defs.extend(d.into_iter());
            }
            Item::Defm(_) | Item::Multiclass(_) => {}
            Item::Class(c) => classes.push(c),
            Item::Def(d) => defs.push(d),
        }
    }
    (classes, defs)
}

pub fn classes_by_name(classes: &[Class]) -> HashMap<&str, &Class> {
    let mut map = HashMap::new();
    for c in classes.iter() {
        assert!(map.insert(&c.name[], c).is_none());
    }
    map
}

pub fn resolve_classes(defs: &mut [Def], classes: &HashMap<&str, &Class>) {
    for d in defs.iter_mut() {
        let empty = HashMap::new();
        let mut new = vec![];
        for sup in d.inherits.iter() {
            substitute_type(&mut new, sup, classes, &empty)
        }
        d.inherits = new;
    }

    fn substitute_type(types: &mut Vec<Type>, ty: &Type, classes: &HashMap<&str, &Class>,
                       args: &HashMap<&String, &Val>) {
        use std::iter::repeat;

        let class = match classes.get(&ty.name[]) {
            None => {types.push(ty.clone()); return}
            Some(c) => c,
        };
        let this_args = ty.args.iter().map(Some).chain(repeat(None));

        let real_args = this_args.zip(class.args.iter()).map(|(left, right)| {
            left.map(|v| substitute(v, args))
                .or_else(|| right.2.clone())
                .expect("arg with no val")
        }).collect::<Vec<_>>();
        {
            let arg_dict = real_args.iter().zip(class.args.iter()).map(|(val, right)| {
                (&right.1, val)
            }).collect();
            for sup in class.inherits.iter() {
                substitute_type(types, sup, classes, &arg_dict)
            }
        }

        types.push(Type {
            name: ty.name.clone(),
            args: real_args.clone(),
        });
    }

    fn substitute(val: &Val, rules: &HashMap<&String, &Val>) -> Val {
        match *val {
            Val::Type(ref ty) => match rules.get(&ty.name) {
                Some(new) => {assert!(ty.args.is_empty()); (*new).clone()},
                None => val.clone()
            },
            Val::List(ref vals) => {
                Val::List(vals.iter().map(|v| substitute(v, rules)).collect())
            }
            Val::Strconcat(ref vals) => {
                let new = vals.iter().map(|v| substitute(v, rules)).collect::<Vec<_>>();
                if new.iter().all(|s| match *s { Val::String(_) => true, _ => false }) {
                    Val::String(new.iter().map(|s| match *s {
                        Val::String(ref s) => &s[], _ => unreachable!() }).collect())
                } else {
                    Val::Strconcat(new)
                }
            }

            _ => val.clone()
        }
    }
}

struct Parser<'a, I: Iterator<Item = Token>> {
    tokens: I,
    root: &'a Path
}

fn expect(t: Token, expected: Token) {
    if t != expected {
        panic!("expected {:?}, found {:?}", expected, t)
    }
}
fn expect_ident(tok: Token) -> String {
    match tok {
        Token::Ident(s) => s,
        _ => panic!("expected ident, found {:?}", tok)
    }
}

impl<'a, I: Iterator<Item = Token>> Parser<'a, I> {
    fn subparser<'b, J: Iterator<Item = Token>>(&'b self, iter: J) -> Parser<'b,J> {
        Parser {
            tokens: iter,
            root: self.root
        }
    }

    fn token(&mut self) -> Token { self.tokens.next().expect("found EOF") }
    fn expect_token(&mut self, expected: Token) {
        expect(self.token(), expected);
    }
    fn expect_ident_or_eof(&mut self) -> Option<String> {
        self.tokens.next().map(expect_ident)
    }
    fn expect_ident(&mut self) -> String {
        self.expect_ident_or_eof().expect("expected ident, found EOF")
    }

    fn parse_items_to_eof(&mut self) -> Vec<Item> {
        let mut ret = vec![];
        while let Some(item) = self.parse_item_or_eof() {
            ret.push(item);
        }
        ret
    }

    fn parse_item_or_eof(&mut self) -> Option<Item> {
        self.expect_ident_or_eof().map(|s| {
            match &s[] {
                "def" => Item::Def(self.parse_def()),
                "defm" => Item::Defm(self.parse_def()),
                "let" => Item::Let(self.parse_let()),
                "class" => Item::Class(self.parse_class()),
                "multiclass" => Item::Multiclass(self.parse_class()),
                "include" => Item::Include(self.parse_include()),
                _ => panic!("unexpected keyword {}", s)
            }
        })
    }

    fn parse_def(&mut self) -> Def {
        let name = self.expect_ident();
        self.expect_token(Token::Colon);
        let inherits = self.parse_inherits();
        Def { name: name, inherits: inherits }
    }

    fn parse_let(&mut self) -> Let {
        let _name = self.expect_ident();
        self.expect_token(Token::Equals);
        match self.token() {
            Token::Square(_) | Token::String(_) | Token::Int(_) => {}
            tok => panic!("expected [...], string or int, found {:?}", tok)
        }
        self.expect_token(Token::Ident("in".to_string()));
        match self.token() {
            Token::Braces(contents) => {
                let mut subparser = self.subparser(contents.into_iter());
                let items = subparser.parse_items_to_eof();
                Let { items: items }
            }
            tok => panic!("expected {{...}}, found {:?}", tok)
        }
    }

    fn parse_class(&mut self) -> Class {
        let name = self.expect_ident();
        let (args, tok) = match self.token() {
            Token::Spikey(contents) => {
                let mut args = vec![];
                {
                    let mut subparser = self.subparser(contents.into_iter());
                    while let Some((ty, tok)) = subparser.try_parse_type_or_eof(None) {
                        let next = tok.unwrap_or_else(|| subparser.token());
                        let name = expect_ident(next);
                        let (next, val) = match subparser.tokens.next() {
                            Some(Token::Equals) => {
                                let (val, tok) = subparser.try_parse_val_or_eof()
                                    .expect("expected val, found EOF");

                                (tok.or_else(|| subparser.tokens.next()), Some(val))
                            }
                            tok => (tok, None),
                        };
                        args.push((ty, name, val));
                        match next {
                            Some(tok) => expect(tok, Token::Comma),
                            None => break
                        }
                    }
                }
                (args, self.token())
            }
            tok => (vec![], tok),
        };
        let inherits = match tok {
            Token::Semicolon | Token::Braces(_) => vec![],
            Token::Colon => self.parse_inherits(),
            tok => panic!("expected : ; or {{...}}, found {:?}", tok)
        };
        Class {
            name: name,
            args: args,
            inherits: inherits
        }
    }

    fn parse_include(&mut self) -> Include {
        let path = match self.token() {
            Token::String(s) => s,
            tok => panic!("expected string, found {:?}", tok)
        };

        if true {
            // ignore includes for now
            return Include { items: vec![] }
        }
        let file = self.root.join(path);
        let s = File::open(&file).read_to_string().unwrap();
        Include { items: parse(&s[], self.root) }
    }

    fn parse_inherits(&mut self) -> Vec<Type> {
        let mut ret = vec![];

        loop {
            let (ty, tok) = self.try_parse_type_or_eof(None).expect("expected type, found EOF");
            ret.push(ty);

            match tok.unwrap_or_else(|| self.token()) {
                Token::Comma => {},
                Token::Semicolon | Token::Braces(_) => break,
                tok => panic!("expected , ; or {{...}}, found {:?}", tok)
            }
        }
        ret
    }

    fn try_parse_type_or_eof(&mut self, first: Option<Token>) -> Option<(Type, Option<Token>)> {
        first.map(expect_ident).or_else(|| self.expect_ident_or_eof()).map(|name| {
            match self.tokens.next() {
                Some(Token::Spikey(contents)) => {
                    let mut subparser = self.subparser(contents.into_iter());
                    let vals = subparser.parse_vals_until_eof();
                    (Type {
                        name: name,
                        args: vals,
                    }, None)
                }
                tok => (Type { name: name, args: vec![] }, tok),
            }
        })
    }
    fn parse_vals_until_eof(&mut self) -> Vec<Val> {
        let mut ret = vec![];
        while let Some((val, tok)) = self.try_parse_val_or_eof() {
            ret.push(val);
            match tok.or_else(|| self.tokens.next()) {
                Some(tok) => expect(tok, Token::Comma),
                None => break
            }
        }
        ret
    }
    fn try_parse_val_or_eof(&mut self) -> Option<(Val, Option<Token>)> {
        self.tokens.next().map(|tok| {
            match tok {
                Token::Square(contents) => {
                    let mut subparser = self.subparser(contents.into_iter());
                    let vals = subparser.parse_vals_until_eof();
                    (Val::List(vals), None)
                }
                Token::Int(n) => {
                    (Val::Int(n), None)
                }
                Token::String(s) => {
                    (Val::String(s), None)
                }
                Token::Bang => {
                    self.expect_token(Token::Ident("strconcat".to_string()));
                    match self.token() {
                        Token::Parens(contents) => {
                            let mut subparser = self.subparser(contents.into_iter());
                            let vals = subparser.parse_vals_until_eof();
                            (Val::Strconcat(vals), None)
                        }
                        tok => panic!("expected (...), found {:?}", tok)
                    }
                }
                _ => {
                    let (ty, tok) = self.try_parse_type_or_eof(Some(tok))
                        .expect("expected type, found EOF");
                    (Val::Type(ty), tok)
                }
            }
        })
    }
}

pub fn parse(s: &str, root: &Path) -> Vec<Item> {
    let mut p = Parser {
        tokens: tokenize(s).into_iter(),
        root: root
    };
    p.parse_items_to_eof()
}
