use crate::memory::*;

pub fn new_str(string: &str, mem: &mut Vec<Item>, ptr_stack: &mut Vec<Pointer>) {
    let mut new_str = String::from(string);
    new_str.pop();

    let tmp_str1: Vec<&str> = new_str.split("(").collect();
    let content: Vec<&str> = tmp_str1[1].split("(").collect();

    let args: Vec<&str> = content[0].split(",").collect();
    let tmp_str2: Vec<&str> = args[1].split(" ").collect();
    let value: i32 = tmp_str2[tmp_str2.len() -1].parse().unwrap();
    
    mem.push(Item::Int(value));
    ptr_stack.push(Pointer { item: mem.len() -1, name: args[0].to_string() })
}
