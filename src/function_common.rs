use crate::memory::*;
use crate::format::*;
use crate::errcodes::*;
use std::process::exit;

pub fn check_args(args: &Vec<Value>, argc: usize, argt: Vec<&str>, name: &str) {
    if argc != args.len() {
        eprintln!("{RED}ERR:{RESET_FORMAT} {name} takes in {BOLD}{argc}{RESET_FORMAT} arguments");
        exit(BAD_ARGC);
    }
    for i in 0..args.len() {
        match args[i] {
            Value::Int(_) => { if argt[i] != "int" { exit(WRONG_ARGT) } }
            Value::String(_) => { if argt[i] != "str" { exit(WRONG_ARGT) } }
            Value::Bool(_) => { if argt[i] != "bool" { exit(WRONG_ARGT) } }
            Value::Char(_) => { if argt[i] != "char" { exit(WRONG_ARGT) } }

            Value::Error(_, _) => { if argt[i] != "error" { exit(WRONG_ARGT) } }
            Value::Null => { if argt[i] != "null" { exit(WRONG_ARGT) } }
        }
    }
}
