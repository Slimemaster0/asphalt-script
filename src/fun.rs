use crate::print::*;
use crate::memory::*;
use crate::int::new_int;

pub fn fun(input: &str, stack: &mut Vec<Item>, ptr_stack: &mut Vec<Pointer>) {
    let keyword: Vec<&str> = input.split("(").collect();
    if keyword.len() != 1 {
        match keyword[0].to_owned().as_str() {
            "printf" => printf(input),
            "int" => new_int(input, stack, ptr_stack),
            "printvar" => print_var(input, stack, ptr_stack),           

            _ => eprintln!("\x1b[31mERR:\x1b[0m {}", input),
        }
    }
}
