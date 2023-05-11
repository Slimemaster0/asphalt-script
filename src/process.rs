use std::process::exit;

use crate::errcodes::*;
use crate::memory::Value;
use crate::format::*;

pub fn die(args: Vec<Value>) {
    if args.len() != 1 {
        eprintln!("{YELLOW}W:{RESET_FORMAT} 'die' takes one argument of type 'int'");
        exit(BAD_ARGC);
    }
    
    match args[0] {
        Value::Int(v) => exit(v),
        _ => {
            eprintln!("{YELLOW}W:{RESET_FORMAT} 'die' takes one argument of type 'int'");
            exit(WRONG_ARGT);
        }
    }
}
