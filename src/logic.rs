// vim:fileencoding=utf-8:foldmethod=marker
use crate::memory::*;
use crate::function::check_args;

// {{{ and
pub fn and(args: Vec<Value>) -> Value {
    check_args(&args, 2, vec!["bool".to_string(), "bool".to_string()], "and");
    
    let mut arg1 = false;
    let mut arg2 = false;

    // {{{ copy arguments
    match args[0] {
        Value::Bool(v) => arg1 = v,
        _ => {}
    }
    match args[1] {
        Value::Bool(v) => arg2 = v,
        _ => {}
    }
    // }}}
    return Value::Bool(arg1 && arg2);
} // }}}

// {{{ or
pub fn or(args: Vec<Value>) -> Value { 
    check_args(&args, 2, vec!["bool".to_string(), "bool".to_string()], "or");
    
    let mut arg1 = false;
    let mut arg2 = false;

    // {{{ copy arguments
    match args[0] {
        Value::Bool(v) => arg1 = v,
        _ => {}
    }
    match args[1] {
        Value::Bool(v) => arg2 = v,
        _ => {}
    }
    // }}}
    return Value::Bool(arg1 || arg2);
} // }}}

// {{{ not
pub fn not(args: Vec<Value>) -> Value {
    check_args(&args, 1, vec!["bool".to_string()], "!");
    
    let mut arg1 = false;
    // {{{ copy arguments
    match args[0] {
        Value::Bool(v) => arg1 = v,
        _ => {}
    }
    // }}}
    return Value::Bool(!arg1);
} // }}}
