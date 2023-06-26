// vim:fileencoding=utf-8:foldmethod=marker
// {{{ Modules
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
mod comp;
mod logic;
mod function;
// }}}

// {{{ Use
use crate::memory::*;
use crate::lexer::fun;
use crate::function::{ Function, function_executor, fun_finder };
use crate::errcodes::*;
use crate::format::*;

use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::process::exit;
// }}}

// {{{ Main function
fn main() {
    let args: Vec<String> = env::args().collect();

    let mut file = File::open(args[1].as_str()).expect("\x1b[31mERR:\x1b[0m Cannot open file!");
    let mut script = String::new();

    file.read_to_string(&mut script).expect("\x1b[31mERR:\x1b[0m Cannot read file!");

    init(&script);
}
// }}}

// {{{ init
pub fn init(file: &String) {
    let mut functions: Vec<Function> = Vec::new();

    if  !file.contains("fun!") &&
        !file.contains("boiler") &&
        !file.contains("{") &&
        !file.contains("}") {
        functions = vec![Function { name: String::from("boiler"), arguments: Vec::new(), arg_names: Vec::new(), arg_mut: Vec::new(), code: file.to_owned()}];
    } else {
        eprintln!("{RED}ERR:{RESET_FORMAT} function are not implemented yet!");
        exit(NOT_IMPL);
    }
    
    let boilerplate_function_id = fun_finder(&functions, "boiler");

    function_executor(&functions[boilerplate_function_id], Vec::new(), &functions);
}
// }}}
