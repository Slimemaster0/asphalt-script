use crate::memory::Value;

pub fn test_parse_args(args: Vec<Value>) {
    for i in 0..args.len() {
        match &args[i] {
            Value::Int(v) => println!("Int: {}", v),
            Value::String(v) => println!("String: {}", v),
            Value::Null => println!("Null")
        }
    }
}
