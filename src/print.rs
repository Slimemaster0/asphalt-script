use crate::memory::*;

pub fn printf(args: Vec<Value>) {
    for i in 0..args.len() {
        match &args[i] {
            Value::Int(v) => print!("{}", v),
            Value::String(v) => print!("{}", v),
            Value::Null => eprintln!("Tried to print a 'Null'")
        }
    }
}


pub fn print_var(string: &str, stack: &mut Vec<Item>) {
    let tmp_str1: Vec<&str> = string.split("(").collect();
    let content: Vec<&str> = tmp_str1[1].split(")").collect();

    let ref var = stack[read_pointer(&stack, &content[0])];

    match &var.value {
        Value::Int(i) => print!("{}", i.to_owned()),
        Value::String(i) => print!("{}", i.to_owned()),



        Value::Null => {}
    }

}
