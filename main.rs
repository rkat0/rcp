fn read_string() -> Vec<char> {
    use std::io::Read;

    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    let mut ret: Vec<char> = s.chars().collect();
    ret.push('\0');
    ret
}

#[derive(PartialEq, Eq, Debug)]
enum Token {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    ParL,
    ParR,
    Assign,
    Semicolon,
    Num(usize),
    Id(String),
    Eof
}

#[derive(PartialEq, Eq, Debug)]
enum Astty {
    Num(usize),
    Id(String),
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Assign,
    Eof
}

#[derive(Debug)]
struct Ast {
    ty: Astty,
    args: Vec<Box<Ast>>
}

impl Ast {
    fn new0(ty: Astty) -> Ast {
        Ast {ty: ty, args: Vec::new()}
    }

    // fn new1(ty: Astty, arg: Ast) -> Ast {
    //     Ast {ty: ty, args: vec![Box::new(arg)]}
    // }

    fn new2(ty: Astty, arg1: Ast, arg2: Ast) -> Ast {
        Ast {ty: ty, args: vec![Box::new(arg1), Box::new(arg2)]}
    }
}

type Code = Vec<Box<Ast>>;

use std::fs::File;
use std::io::Write;

fn main() {
    let input = read_string();
    let ofile_name = "tmp.s";
    let toks = tokenize(&input);
    let code = parse(&toks);

    let mut file = File::create(ofile_name).unwrap();
    file.write_all(b".intel_syntax noprefix\n").unwrap();
    file.write_all(b".global _main\n\n").unwrap();
    file.write_all(b"_main:\n").unwrap();
    gen_code(&code, &mut file);
    file.write_all(b"\tpop rax\n").unwrap();
    file.write_all(b"\tret\n").unwrap();
}

fn tokenize(buf: &[char]) -> Vec<Token> {
    let mut idx = 0;
    let mut toks = Vec::new();
    skip_whitespace(buf, &mut idx);
    while buf[idx] != '\0' {
        match buf[idx] {
            '+' => { toks.push(Token::Add); idx += 1; },
            '-' => { toks.push(Token::Sub); idx += 1; },
            '*' => { toks.push(Token::Mul); idx += 1; },
            '/' => { toks.push(Token::Div); idx += 1; },
            '%' => { toks.push(Token::Mod); idx += 1; },
            '(' => { toks.push(Token::ParL); idx += 1; },
            ')' => { toks.push(Token::ParR); idx += 1; },
            '=' => { toks.push(Token::Assign); idx += 1; },
            ';' => { toks.push(Token::Semicolon); idx += 1; },
            '0'..='9' => toks.push(Token::Num(read_num(buf, &mut idx))),
            'a'..='z' | 'A'..='Z' | '_' => toks.push(Token::Id(read_id(buf, &mut idx))),
            _ => { panic!("unknown character '{}'", buf[idx].escape_debug()); }
        }
        skip_whitespace(buf, &mut idx);
    }
    toks.push(Token::Eof);
    toks
}

fn skip_whitespace(buf: &[char], idx: &mut usize) {
    while *idx < buf.len() && buf[*idx].is_whitespace() {
        *idx += 1;
    }
}

fn read_num(buf: &[char], idx: &mut usize) -> usize {
    let mut ret = 0;
    while let Some(n) = buf[*idx].to_digit(10) {
        ret = ret * 10 + n as usize;
        *idx += 1;
    }
    ret
}

fn read_id(buf: &[char], idx: &mut usize) -> String {
    let s = *idx;
    while buf[*idx].is_ascii_alphanumeric() || buf[*idx] == '_' {
        *idx += 1;
    }
    buf[s..*idx].iter().collect()
}

fn parse(toks: &[Token]) -> Code {
    let mut idx = 0;
    let mut code = Vec::new();
    while idx < toks.len() - 1 {
        code.push(Box::new(assign(&toks, &mut idx)));
    }
    code
}

fn assign(toks: &[Token], idx: &mut usize) -> Ast {
    let mut node = add(toks, idx);
    loop {
        match toks[*idx] {
            Token::Assign => { *idx += 1; node = Ast::new2(Astty::Assign, node, assign(toks, idx)); },
            Token::Semicolon => { *idx += 1; return node; },
            ref t => { panic!("';' did not found. Found {:?}", t); }
        }
    }
}

fn add(toks: &[Token], idx: &mut usize) -> Ast {
    if toks.len() == *idx {
        return Ast::new0(Astty::Eof);
    }
    let mut node = mul(toks, idx);
    loop {
        match toks[*idx] {
            Token::Add => { *idx += 1; node = Ast::new2(Astty::Add, node, mul(toks, idx)); },
            Token::Sub => { *idx += 1; node = Ast::new2(Astty::Sub, node, mul(toks, idx)); },
            _ => { return node; }
        }
    }
}

fn mul(toks: &[Token], idx: &mut usize) -> Ast {
    if toks.len() == *idx {
        return Ast::new0(Astty::Eof);
    }
    let mut node  = term(toks, idx);
    loop {
        match toks[*idx] {
            Token::Mul => { *idx += 1; node = Ast::new2(Astty::Mul, node, term(toks, idx)) },
            Token::Div => { *idx += 1; node = Ast::new2(Astty::Div, node, term(toks, idx)) },
            Token::Mod => { *idx += 1; node = Ast::new2(Astty::Mod, node, term(toks, idx)) },
            _ => { return node; }
        }
    }
}

fn term(toks: &[Token], idx: &mut usize) -> Ast {
    if toks.len() == *idx {
        return Ast::new0(Astty::Eof);
    }
    match toks[*idx] {
        Token::ParL => {
            *idx += 1;
            let node = add(toks, idx);
            if toks[*idx] == Token::ParR {
                *idx += 1;
                node
            } else {
                panic!("Right parenthesis ')' was not found. Found {:?}", toks[*idx]);
            }
        },
        Token::Num(n) => {
            *idx += 1;
            Ast::new0(Astty::Num(n))
        },
        _ => { return Ast::new0(Astty::Eof); }
    }
}

fn gen_code(code: &Code, file: &mut File) {
    let ast = code.last().unwrap();
    gen(ast, file);
}

fn gen(ast: &Box<Ast>, file: &mut File) {
    for subt in ast.args.iter() {
        gen(subt, file);
    }
    if ast.args.len() == 2 {
        file.write_all(b"\tpop rdi\n").unwrap();
        file.write_all(b"\tpop rax\n").unwrap();
    }
    match ast.ty {
        Astty::Num(n) => {
            file.write_all(format!("\tpush {}\n", n).as_bytes()).unwrap();
            return;
        },
        Astty::Id(ref arg) => {
            panic!("unimplemented");
        },
        Astty::Add => {
            file.write_all(b"\tadd rax, rdi\n").unwrap();
        },
        Astty::Sub => {
            file.write_all(b"\tsub rax, rdi\n").unwrap();
        },
        Astty::Mul => {
            file.write_all(b"\tmul rdi\n").unwrap();
        },
        Astty::Div => {
            file.write_all(b"\tmov rdx, 0\n").unwrap();
            file.write_all(b"\tdiv rdi\n").unwrap();
        },
        Astty::Mod => {
            panic!("unimplemented");
        },
        Astty::Assign => {
            return;
            // panic!("unimplemented")
        },
        Astty::Eof => {
            return;
        }
    }
    file.write_all(b"\tpush rax\n").unwrap();
}
