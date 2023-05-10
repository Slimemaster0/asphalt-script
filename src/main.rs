// Modules
mod print;
mod fun;
mod memory;
mod test;

// Use
use crate::memory::*;
use crate::fun::fun;

use std::fs::File;
use std::io::prelude::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut file = File::open(args[1].as_str()).expect("\x1b[31mERR:\x1b[0m Cannot open file!");
    let mut script = String::new();

    file.read_to_string(&mut script).expect("\x1b[31mERR:\x1b[0m Cannot read file!");

    let code: Vec<&str> = script.split("\n").collect();

    let mut stack: Vec<Item> = Vec::new();
    
    for line in code.iter() {
        fun(line, &mut stack);
    }
}
