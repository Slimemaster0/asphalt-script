use crate::memory::*;

pub struct Int {
    pub value: i32,
}

impl Clone for Int {
    fn clone(&self) -> Self { todo!() }
}
impl Copy for Int {}

pub fn new_int(string: &str, mem: &mut Vec<Item>, ptr_stack: &mut Vec<Pointer>) {
    let mut new_str = String::from(string);
    new_str.pop();

    let tmp_str1: Vec<&str> = new_str.split("(").collect();
    let content: Vec<&str> = tmp_str1[1].split("(").collect();

    let args: Vec<&str> = content[0].split(",").collect();
    let tmp_str2: Vec<&str> = args[1].split(" ").collect();
    let value: i32 = tmp_str2[tmp_str2.len() -1].parse().unwrap();
    
    mem.push(Item::Int(Int { value: value }));
    ptr_stack.push(Pointer { item: mem.len() -1, name: args[0].to_string() })
}
