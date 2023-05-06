use std::process::exit;


#[derive(Copy, Clone)]
pub enum Item {
    Int(i32)
}

pub struct Pointer {
    pub item: usize,
    pub name: String,
}

pub fn deref_pointer(ptr_stack: &Vec<Pointer>, name: &str, stack: &Vec<Item>) -> Item {
    let mut i: usize = 0;
    while i < ptr_stack.len() {
        if ptr_stack[i].name == name.to_string() {
            return  stack[ptr_stack[i].item];
            }
        i += 1;
        }
    eprintln!("\x1b[31mERR:\x1b[0m Cannot find value '{}'!", name);
    exit(69420);
}
