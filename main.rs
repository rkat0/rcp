fn read_string() -> String {
    use std::io::Read;

    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    s
}

use std::fs::File;
use std::io::Write;

fn main() {
    let input = read_string();
    println!("input: {}", input);
    let n = input.parse::<i64>().unwrap();
    let ofile_name = "tmp.s";
    let mut file = File::create(ofile_name).unwrap();
    file.write_all(b".intel_syntax noprefix\n").unwrap();
    file.write_all(b".global _main\n\n").unwrap();
    file.write_all(b"_main:\n").unwrap();
    file.write_all(format!("\tmov rax, {}\n", n).as_bytes()).unwrap();
    file.write_all(b"\tret\n").unwrap();
}
