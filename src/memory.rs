use std::process::exit;

#[derive(Clone)]
pub enum Item {
    Int(i32),
    String(String),
}

pub struct Pointer {
    pub item: usize,
    pub name: String,
}

pub fn read_pointer(ptr_stack: &Vec<Pointer>, name: &str) -> usize {
    let mut i: usize = 0;
    while i < ptr_stack.len() {
        if ptr_stack[i].name == name.to_string() {
            return i;
            }
        i += 1;
        }
    eprintln!("\x1b[31mERR:\x1b[0m Cannot find value '{}'!", name);
    exit(69420);
}

pub fn new_var(string: &str, mem: &mut Vec<Item>, ptr_stack: &mut Vec<Pointer>) {
    let mut new_str = String::from(string);
    new_str.pop();

    let tmp_str1: Vec<&str> = new_str.split("(").collect();
    let content: Vec<&str> = tmp_str1[1].split("(").collect();

    let args: Vec<&str> = content[0].split(",").collect();

    let var_type = args[1].trim();
    let var_value = args[2].trim();

    match var_type {
        "int" => {
            let value: i32 = var_value.parse().unwrap();
            mem.push(Item::Int(value));
            },

        "str" | "string" => {
                let value: Vec<&str> = var_value.split("\"").collect();
                mem.push(Item::String(value[1].to_string()));
            }

        _ => {
            eprintln!("\x1b[31mERR:\x1b[0m unknown type: {}", var_type);
            exit(0x20)
        }
    }
    

    ptr_stack.push(Pointer { item: mem.len() -1, name: args[0].to_string() })
}
