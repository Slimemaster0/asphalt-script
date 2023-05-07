// vim:fileencoding=utf-8:foldmethod=marker

use std::process::exit;

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

pub fn new_var(string: &str, mem: &mut Vec<Item>) { // {{{
    let mut new_str = String::from(string);
    new_str.pop();

    let tmp_str1: Vec<&str> = new_str.split("(").collect();
    let content: Vec<&str> = tmp_str1[1].split("(").collect();

    let args: Vec<&str> = content[0].split(",").collect();

    let var_type = args[1].trim();
    let var_value = args[2].trim();

    match var_type { // {{{ create a variable of the type specified
        "int" => {
            let value: i32 = var_value.parse().unwrap();
            let name: Vec<&str> = args[0].split("\"").collect();
            mem.push(Item { name: name[1].to_string(), value: Value::Int(value) } );
            },

        "str" | "string" => {
                let value: Vec<&str> = var_value.split("\"").collect();
                let name: Vec<&str> = args[0].split("\"").collect();

                mem.push(Item { name: name[1].to_string(), value: Value::String(value[1].to_string())});
            }

        _ => {
            eprintln!("\x1b[31mERR:\x1b[0m unknown type: {}", var_type);
            exit(0x20)
        }
    } // }}}
    

} // }}}
