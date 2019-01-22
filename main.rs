fn read_string() -> Vec<char> {
    use std::io::Read;

    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    let mut ret: Vec<char> = s.chars().collect();
    ret.push('\0');
    ret
}

use std::fs::File;
use std::io::Write;

fn main() {
    let input = read_string();
    let mut idx: usize = 0;
    let ofile_name = "tmp.s";
    let mut file = File::create(ofile_name).unwrap();
    file.write_all(b".intel_syntax noprefix\n").unwrap();
    file.write_all(b".global _main\n\n").unwrap();
    file.write_all(b"_main:\n").unwrap();
    file.write_all(format!("\tmov rax, {}\n", read_num(&input, &mut idx)).as_bytes()).unwrap();
    while input[idx] != '\0' {
        if input[idx] == '+' {
            idx += 1;
            let n = read_num(&input, &mut idx);
            file.write_all(format!("\tadd rax, {}\n", n).as_bytes()).unwrap();
        } else if input[idx] == '-' {
            idx += 1;
            let n = read_num(&input, &mut idx);
            file.write_all(format!("\tsub rax, {}\n", n).as_bytes()).unwrap();
        } else {
            println!("expected op but got {} (at {})", input[idx], idx);
        }
    }
    file.write_all(b"\tret\n").unwrap();
}

fn read_num(buf: &Vec<char>, idx: &mut usize) -> usize {
    let mut ret = 0;
    while let Some(n) = buf[*idx].to_digit(10) {
        ret = ret * 10 + n as usize;
        *idx += 1;
    }
    ret
}
