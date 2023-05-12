#[allow(unused)]
use crate::format::*;
use crate::memory::Value;

pub fn test_parse_args(args: Vec<Value>) {
    for i in 0..args.len() {
        match &args[i] {
            Value::Int(v) => println!("Int: {}", v),
            Value::String(v) => println!("String: {}", v),
            Value::Bool(v) => println!("Bool: {}", v),
            Value::Error(code, error) => println!("Error code: {code}\nError message: {error}"),

            Value::Null => println!("Null")
        }
    }
}
