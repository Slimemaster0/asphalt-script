// vim:fileencoding=utf-8:foldmethod=marker
use crate::memory::*;
use crate::format::*;
use crate::errcodes::*;
use crate::lexer::fun;
use std::process::exit;

pub struct Function {
    pub name: String,
    pub arguments: Vec<String>,
    pub arg_names: Vec<String>,
    pub arg_mut: Vec<bool>,
    pub code: String,
}

// {{{ Check function args
pub fn check_args(args: &Vec<Value>, argc: usize, argt: Vec<String>, name: &str) {
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
// }}}

// {{{ Function executor
pub fn function_executor(func: &Function, args: Vec<Value>, functions: &Vec<Function>) -> Value {
    check_args(&args, func.arguments.len(), func.arguments.clone(), &func.name);
    let code: Vec<&str> = func.code.split(";").collect();

    let mut stack: Vec<Item> = Vec::new();
    
    for i in 0..func.arguments.len() {
        stack.push(Item { 
            value: args[i].clone(),
            name: func.arg_names[i].clone(),
            mutability: func.arg_mut[i].clone()
        })
    }
    
    let mut i: u64 = 0;
    while code.len() as u64 > i {
        if code[i as usize].trim().len() > 0 {
            if code[i as usize].trim().chars().nth(0).expect("No char at 0 - main") == '#' {
                i+=1;
                continue;
            }
        }

        let ret = fun(code[i as usize], &mut stack, &mut i, functions);
        i+=1;
        match ret {
            Value::Null => continue,
            _ => return ret
        }
    }
    return Value::Null;
}
// }}} Function executor

// {{{ Function finder
pub fn fun_finder(functions: &Vec<Function>, name: &str) -> usize {
    for i in 0..functions.len() {
        if functions[i].name == name.trim().to_string() {
            return i;
            }
        }
    eprintln!("\x1b[31mERR:\x1b[0m Cannot find value '{}'!", name);
    exit(ITEM_NOT_EXIST);
}
