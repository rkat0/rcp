fn read_string() -> Vec<char> {
    use std::io::Read;

    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    let mut ret: Vec<char> = s.chars().collect();
    ret.push('\0');
    ret
}

#[derive(Debug)]
enum Token {
    Add,
    Sub,
    Num(usize),
    Eof
}

impl Token {
    fn num(&self) -> Option<usize> {
        match self {
            Token::Num(n) => Some(*n),
            _ => None
        }
    }
}

use std::fs::File;
use std::io::Write;
use std::process::exit;

fn main() {
    let input = read_string();
    let ofile_name = "tmp.s";
    let toks = tokenize(&input);
    let mut file = File::create(ofile_name).unwrap();
    file.write_all(b".intel_syntax noprefix\n").unwrap();
    file.write_all(b".global _main\n\n").unwrap();
    file.write_all(b"_main:\n").unwrap();
    file.write_all(format!("\tmov rax, {}\n", toks[0].num().unwrap()).as_bytes()).unwrap();
    let mut i = 1;
    while i < toks.len() {
        match toks[i] {
            Token::Add => {
                i += 1;
                file.write_all(format!("\tadd rax, {}\n", toks[i].num().unwrap()).as_bytes()).unwrap();
                i += 1;
            },
            Token::Sub => {
                i += 1;
                file.write_all(format!("\tsub rax, {}\n", toks[i].num().unwrap()).as_bytes()).unwrap();
                i += 1;
            },
            Token::Eof => break,
            _ => {
                println!("expected op (+ or -) but got token '{:?}'", toks[i]);
                exit(1);
            }
        }
    }
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
            '0'..='9' => toks.push(Token::Num(read_num(buf, &mut idx))),
            _ => { println!("unknown character '{}'", buf[idx].escape_debug()); exit(1); }
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
