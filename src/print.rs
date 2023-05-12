use crate::memory::*;

pub fn printf(args: Vec<Value>) {
    for i in 0..args.len() {
        match &args[i] {
            Value::Int(v) => print!("{}", v),
            Value::String(v) => print!("{}", v),
            Value::Bool(v) => {
                    match v {
                        true => print!("yup"),
                        false => print!("nope")
                    }
                }
            Value::Error(_, v) => println!("{}", v),

            Value::Null => eprintln!("Tried to print a 'Null'")
        }
    }
}
