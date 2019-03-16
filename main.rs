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
    Num(usize),
    Eof
}

#[derive(PartialEq, Eq, Debug)]
enum Astty {
    Num(usize),
    Add,
    Sub,
    Mul,
    Div,
    Mod,
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

use std::fs::File;
use std::io::Write;

fn main() {
    let input = read_string();
    let ofile_name = "tmp.s";
    let toks = tokenize(&input);
    let ast = Box::new(parse(&toks));

    println!("{:?}", ast);

    let mut file = File::create(ofile_name).unwrap();
    file.write_all(b".intel_syntax noprefix\n").unwrap();
    file.write_all(b".global _main\n\n").unwrap();
    file.write_all(b"_main:\n").unwrap();
    gen(&ast, &mut file);
    file.write_all(b"\tpop rax\n").unwrap();
    file.write_all(b"\tret\n").unwrap();
}

fn tokenize(buf: &Vec<char>) -> Vec<Token> {
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
            '0'..='9' => toks.push(Token::Num(read_num(buf, &mut idx))),
            _ => { panic!("unknown character '{}'", buf[idx].escape_debug()); }
        }
        skip_whitespace(buf, &mut idx);
    }
    toks.push(Token::Eof);
    toks
}

fn skip_whitespace(buf: &Vec<char>, idx: &mut usize) {
    while *idx < buf.len() && buf[*idx].is_whitespace() {
        *idx += 1;
    }
}

fn read_num(buf: &Vec<char>, idx: &mut usize) -> usize {
    let mut ret = 0;
    while let Some(n) = buf[*idx].to_digit(10) {
        ret = ret * 10 + n as usize;
        *idx += 1;
    }
    ret
}

fn parse(toks: &Vec<Token>) -> Ast {
    let mut idx = 0;
    let ast = add(&toks, &mut idx);
    if toks.len() - 1 > idx {
        panic!("got unexpected token {:?} at {}", toks[idx], idx);
    }
    ast
}

fn add(toks: &Vec<Token>, idx: &mut usize) -> Ast {
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

fn mul(toks: &Vec<Token>, idx: &mut usize) -> Ast {
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

fn term(toks: &Vec<Token>, idx: &mut usize) -> Ast {
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
        _ => { panic!("expected '(' or Num but got {:?}", toks[*idx]); }
    }
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
        Astty::Eof => {
            return;
        }
    }
    file.write_all(b"\tpush rax\n").unwrap();
}
