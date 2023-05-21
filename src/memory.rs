// vim:fileencoding=utf-8:foldmethod=marker

use std::process::exit;
use crate::errcodes::*;
use crate::format::*;

#[derive(Clone)]
pub enum Value {
    Null,
    Int(i32),
    String(String),
    Bool(bool),
    Error(i32, String)
}

#[derive(Clone)]
pub struct Item {
    pub value: Value,
    pub name: String,
    pub mutability: bool,
}

pub fn read_pointer(stack: &Vec<Item>, name: &str) -> usize { // {{{
    for i in 0..stack.len() {
        if stack[i].name == name.to_string() {
            return i;
            }
        }
    eprintln!("\x1b[31mERR:\x1b[0m Cannot find value '{}'!", name);
    exit(ITEM_NOT_EXIST);
} // }}}

pub fn new_var(args: Vec<Value>, stack: &mut Vec<Item>) { // {{{
    if args.len() != 3 {
        eprintln!("\x1b31mErr:\x1b0m 'new' takes in 3 arguments");
        exit(BAD_ARGC);
    }

    match &args[0] {
        Value::String(i) => {
            match &args[1] {
                Value::Bool(b) => {
                    let item: Item = Item { value: args[2].to_owned(), name: i.to_owned(), mutability: b.to_owned() };
                    stack.push(item);
                }
                
                _ => {
                    eprintln!("{RED}Err:{RESET_FORMAT} The second argument must be of type 'bool'");
                    exit(WRONG_ARGT);
                }
            }
        }
        _ => {
            eprintln!("\x1b[31mErr:\x1b[0m The first argument for 'new' must be a 'String'");
            exit(WRONG_ARGT);
        }
    }
} // }}}

pub fn del_var(args: Vec<Value>, stack: &mut Vec<Item>) { // {{{
    if args.len() != 1 {
        eprintln!("{RED}Err:{RESET_FORMAT} 'del' takes in 1 argument");
        exit(BAD_ARGC);
    }

    match &args[0] {
        Value::String(i) => {
            stack.remove(read_pointer(&stack.clone(), i.as_str()));
        }

        _ => {
            eprintln!("\x1b[31mErr:\x1b[0m The first argument for 'del' must be a 'String'");
            exit(WRONG_ARGT);
        }

    }
} // }}}

//fn mut_var(args: Vec<Value>, stack: &mut Vec<Item>) {
//    if args.len() != 2 {
//    }
//}
