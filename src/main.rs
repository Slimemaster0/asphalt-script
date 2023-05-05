// Modules
mod print;

// Use
use crate::print::printf;
use std::fs::File;
use std::io::prelude::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut file = File::open(args[1].as_str()).expect("\x1b[31mERR:\x1b[0m Cannot open file!");
    let mut script = String::new();

    file.read_to_string(&mut script).expect("\x1b[31mERR:\x1b[0m Cannot read file!");

    let code: Vec<&str> = script.split("\n").collect();
    
    for line in code.iter() {
        let keyword: Vec<&str> = line.split("(").collect();
        if keyword.len() != 1 {
            match keyword[0].to_owned().as_str() {
                "printf" => printf(line),
                _ => eprintln!("\x1b[31mERR:\x1b[0m {}", line),
            }
        }
    }
}
