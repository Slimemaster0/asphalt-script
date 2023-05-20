use std::eprintln;

use crate::memory::Value;
use crate::errcodes::*;
use crate::format::*;
use std::process::exit;

pub fn binops(args: Vec<Value>, operator: char) -> Value {
    if args.len() != 2 {
        eprintln!("{RED}Err:{RESET_FORMAT} Binary operations takes in {BOLD}2{RESET_FORMAT} arguments");
        exit(BAD_ARGC);
    }
    match args[0] {
        Value::Int(i1) => {
            match args[1] {
                Value::Int(i2) => {
                    match operator {
                        '+' => return Value::Int(i1 + i2),
                        '-' => return Value::Int(i1 - i2),
                        '*' => return Value::Int(i1 * i2),
                        '/' => return Value::Int(i1 / i2),
                        
                        _ => {
                            eprintln!("{RED}Err:{RESET_FORMAT} the operator most be '+', '-', '*' or '/'");
                            exit(WRONG_ARGT);
                        }
                    }
                }
                _ => {
                    eprintln!("{RED}Err:{RESET_FORMAT} Binary operations expects 2 numbers of any type");
                    exit(WRONG_ARGT);
                }
            }
        }

        _ => {
            eprintln!("{RED}Err:{RESET_FORMAT} Binary operations expects 2 numbers of any type");
            exit(WRONG_ARGT);
        }
    }
}
