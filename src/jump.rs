// vim:fileencoding=utf-8:foldmethod=marker

use crate::memory::{Item, Value};
use crate::format::*;
use crate::errcodes::*;
use std::process::exit;

// {{{ Relative jump
pub fn jump(args: Vec<Value>, line_num: &mut u64) {
    if args.len() != 1 {
        eprintln!("{RED}Err:{RESET_FORMAT} jmp takes in {BOLD}1{RESET_FORMAT} argument!");
        exit(BAD_ARGC);
    }
    match args[0] {
        Value::Int(i) => {
            *line_num = (*line_num as i128 + i as i128) as u64;
            return;
        },
        
        _ => {
            eprintln!("{RED}Err:{RESET_FORMAT} jmp takes in a value of type Int");
            exit(WRONG_ARGT);
        }
    }
}
// }}}

// {{{ Condition and relative jump
pub fn jumpif(args: Vec<Value>, line_num: &mut u64) {
    if args.len() != 2 {
        eprintln!("{RED}Err:{RESET_FORMAT} jmpif takes in {BOLD}2{RESET_FORMAT} arguments!");
        exit(BAD_ARGC);
    }
    match &args[0] {
        Value::Bool(b) => { 
            if b.to_owned() {
                match args[1] {
                    Value::Int(i) => {
                        *line_num = (*line_num as i128 + i as i128) as u64;
                        return;
                    }
                    
                    _ => {
                        eprintln!("{RED}Err:{RESET_FORMAT} The second argument for jmpif must be of type 'Int'!");
                        exit(WRONG_ARGT);
                    }
                }
            } else {
                match args[1] {
                    Value::Int(_) => {
                        return;
                    }

                    // {{{ The user entered the wrong data type
                    _ => {
                        eprintln!("{RED}Err:{RESET_FORMAT} The second argument for jumpif must be of type 'Int'!");
                        exit(WRONG_ARGT);
                    }
                }
            }
        }
        
        _ => {
            eprintln!("{RED}Err:{RESET_FORMAT} The first argument for jumpif must be of type 'Bool'!");
            exit(WRONG_ARGT);
        }
    }
    // }}}
}
// }}}

// {{{ jump to a location
pub fn jumpto(args: Vec<Value>, line_num: &mut u64) {
    if args.len() != 1 {
        eprintln!("{RED}Err:{RESET_FORMAT} jmpto takes in {BOLD}1{RESET_FORMAT} argument!");
        exit(BAD_ARGC);
    }
    match args[0] {
        Value::Int(i) => {
            if !i.is_negative() {
                *line_num = i as u64;
                return;
            } else {
                eprintln!("{RED}Err:{RESET_FORMAT} jmpto only accepts positive numbers!");
            }
        },
        
        _ => {
            eprintln!("{RED}Err:{RESET_FORMAT} jmpto takes in a value of type Int");
            exit(WRONG_ARGT);
        }
    }
}
// }}}
