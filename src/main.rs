// Modules
mod print;
mod lexer;
mod memory;
mod test;
mod errcodes;
mod process;
mod format;
mod read;
mod binops;
mod jump;

// Use
use crate::memory::*;
use crate::lexer::fun;

use std::fs::File;
use std::io::prelude::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut file = File::open(args[1].as_str()).expect("\x1b[31mERR:\x1b[0m Cannot open file!");
    let mut script = String::new();

    file.read_to_string(&mut script).expect("\x1b[31mERR:\x1b[0m Cannot read file!");

    let code: Vec<&str> = script.split(";").collect();

    let mut stack: Vec<Item> = Vec::new();
    
    let mut i: u64 = 0;
    while code.len() as u64 > i {
        if code[i as usize].trim().len() > 0 {
            if code[i as usize].trim().chars().nth(0).expect("No char at 0 - main") == '#' {
                i+=1;
                continue;
            }
        }

        fun(code[i as usize], &mut stack, &mut i);
        i+=1;
    }
}
