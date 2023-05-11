// vim:fileencoding=utf-8:foldmethod=marker

use std::process::exit;
use crate::errcodes::*;

#[derive(Clone)]
pub enum Value {
    Null,
    Int(i32),
    String(String),
}

#[derive(Clone)]
pub struct Item {
    pub value: Value,
    pub name: String,
}

pub fn read_pointer(stack: &Vec<Item>, name: &str) -> usize { // {{{
    for i in 0..stack.len() {
        if stack[i].name == name.to_string() {
            return i;
            }
        }
    eprintln!("\x1b[31mERR:\x1b[0m Cannot find value '{}'!", name);
    exit(69420);
} // }}}

pub fn new_var(args: Vec<Value>, stack: &mut Vec<Item>) { // {{{
    if args.len() != 2 {
        eprintln!("\x1b31mErr: 'new' takes in 2 arguments");
        exit(BAD_ARGC);
    }

    match &args[0] {
        Value::String(i) => {
            let item: Item = Item { value: args[1].to_owned(), name: i.to_owned() };
            stack.push(item);
        }
        _ => {
            eprintln!("\x1b[31mErr:\x1b[0m The first argument for 'new' must be a 'String'");
            exit(WRONG_ARGT);
        }
    }
} // }}}
