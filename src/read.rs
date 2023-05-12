use std::fs::File;
use std::io::Read;
use std::process::exit;
use crate::memory::Value;
use crate::errcodes::*;
use crate::format::*;

pub fn read_to_string(args: Vec<Value>) -> Value {
    if args.len() != 1 {
        eprintln!("{RED}Err:{RESET_FORMAT} readf takes in {BOLD}1{RESET_FORMAT} argument");
        exit(BAD_ARGC);
    }    
    
    match &args[0] {
        Value::String(v) => {
            let file = File::open(v.as_str());
            match file {
                Ok(mut f) => {
                    let mut contents = String::new();
                    let result = f.read_to_string(&mut contents);

                    match result {
                        Ok(_) => return Value::String(contents),
                        Err(e) => panic!("{RED}Err:{RESET_FORMAT} {e}")
                    }
                }

                Err(e) => return Value::Error(FAILD_TO_READ, e.to_string())
            }
        }
        _ => {
            eprintln!("{RED}Err:{RESET_FORMAT} readf expects a string");
            exit(WRONG_ARGT)
        }
    }
}
