// vim:fileencoding=utf-8:foldmethod=marker
use crate::memory::*;
use crate::format::*;
use crate::errcodes::*;
use std::process::exit;

// {{{ intcmp
pub fn intcmp(args: Vec<Value>) -> Value {
    if args.len() != 3 {
        eprintln!("{RED}Err:{RESET_FORMAT} intcmp takes in {BOLD}3{RESET_FORMAT} arguments! (Int, String (\"==\", \"<\", \">\", \"<=\", \">=\"), int)");
        exit(BAD_ARGC);
    }
    match &args[0] {
        Value::Int(i1) => {
            match &args[1] {
                Value::String(cmp) => {
                    match &args[2] {
                        Value::Int(i2) => {
                            match cmp.as_str() { 
// {{{ All the different posibilities

                                "==" => {
                                    if i1 == i2 {
                                        return Value::Bool(true);
                                    } else {
                                        return Value::Bool(false);
                                    }
                                }
                                "!=" => {
                                    if i1 == i2{
                                        return Value::Bool(false);
                                    } else {
                                        return Value::Bool(true)
                                    }
                                }
                                "<" => {
                                    if i1 < i2 {
                                        return Value::Bool(true);
                                    } else {
                                        return Value::Bool(false);
                                    }
                                }
                                ">" => {
                                    if i1 > i2 {
                                        return Value::Bool(true);
                                    } else {
                                        return Value::Bool(false);
                                    }
                                }
                                "<=" => {
                                    if i1 <= i2 {
                                        return Value::Bool(true);
                                    } else {
                                        return Value::Bool(false);
                                    }
                                }
                                ">=" => {
                                    if i1 >= i2 {
                                        return Value::Bool(true);
                                    } else {
                                        return Value::Bool(false);
                                    }
                                } 

// }}}
                                _ => {
                                    eprintln!("{RED}Err:{RESET_FORMAT} The string must be \"==\", \"!=\" \"<\", \">\", \"<=\", \">=\"");
                                    exit(FAILED_PARSE);
                                }
                            }
                        }
                        
                        _ => {
                            eprintln!("{RED}Err:{RESET_FORMAT} The 3rd argument must be an integer");
                            exit(WRONG_ARGT);
                        }
                    }
                }

                _ => {
                    eprintln!("{RED}Err:{RESET_FORMAT} The second argument must be a string");
                    exit(WRONG_ARGT);
                }
            }
        }
        
        _ => {
            eprintln!("{RED}Err:{RESET_FORMAT} The first argument must be an integer");
            exit(WRONG_ARGT);
        }
    }
}
// }}}

// {{{ strcmp
pub fn strcmp(args: Vec<Value>) -> Value {
    if args.len() != 2 {
        eprintln!("{RED}Err:{RESET_FORMAT} strcmp takes in {BOLD}2{RESET_FORMAT} arguments!");
        exit(BAD_ARGC);
    }

    match &args[0] {
        Value::String(s1) => {
            match &args[1] {
                Value::String(s2) => {
                    if s1 == s2 {
                        return Value::Bool(true);
                    } else {
                        return Value::Bool(false);
                    }
                }
                _ => {
                    eprintln!("{RED}Err:{RESET_FORMAT} strcmp takes in 2 strings");
                    exit(WRONG_ARGT)
                }
            }
        }
        _ => {
            eprintln!("{RED}Err:{RESET_FORMAT} strcmp takes in 2 strings");
            exit(WRONG_ARGT);
        }
    }
}
// }}}
