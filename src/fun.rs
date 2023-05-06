use crate::print::*;
use crate::memory::*;

pub fn fun(input: &str, stack: &mut Vec<Item>, ptr_stack: &mut Vec<Pointer>) {
    let keyword: Vec<&str> = input.split("(").collect();
    if keyword.len() != 1 {
        match keyword[0].to_owned().as_str().trim() {
            "printf" => printf(input),
            "new" => new_var(input, stack, ptr_stack),
            "printvar" => print_var(input, stack, ptr_stack),           

            _ => eprintln!("\x1b[31mERR:\x1b[0m {}", input),
        }
    }
}
