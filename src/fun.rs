// vim:fileencoding=utf-8:foldmethod=marker

use std::process::exit;

use crate::print::*;
use crate::memory::*;

pub fn fun(input: &str, stack: &mut Vec<Item>) -> Value {
    let keyword: Vec<&str> = input.split("(").collect();
    if keyword.len() != 1 {
        match keyword[0].to_owned().as_str().trim() {
            "printf" => printf(input),
            "new" => new_var(input, stack),
            "printvar" => print_var(input, stack),

            _ => eprintln!("\x1b[31mERR:\x1b[0m {}", input),
        }
    }
    return Value::Null;
}

fn parse_args(str: &str, stack: &mut Vec<Item>) -> Vec<Value> { // {{{

    if str.len() == 0 { return Vec::new(); };

    let mut string = String::from(str);
    string.pop();


    while string.chars().nth(0).expect("No char at 0") != '(' {
        string = remove_first(&string.as_str())
            .expect("Could not remove first char")
            .to_string();
    }

    string = remove_first(&string.as_str())
        .expect("Could not remove first char")
        .to_string();

    let mut args_str: Vec<String> = string // Split the string
        .split(",")
        .map(|s| s.to_string()).collect();

    for i in 0..args_str.len() { // Trim each argument
        args_str[i] = args_str[i].trim().to_string();
    }

    let mut args: Vec<Value> = Vec::new();
    
    for i in 0..args_str.len() { // {{{ parse each arguments
                                 
        if args_str[i].contains("(*)") { // function
            let r = &mut *stack;
            args.push(fun(&args_str[i], r));
            continue;

        } else if args_str[i].contains("\"") { // String
                                               
            let content: Vec<&str> = args_str[i].split("\"").collect();
            args.push(Value::String(content[1].to_string()));
            continue;

        }  else if args_str[i] // Floating point numbers
                .chars()
                .nth(args_str[i].len())
                .expect("add -1 to float checking") == 'F' {

          //  args.push();
            eprintln!("Float is not implemented yet");
            exit(10);
            
        }

        let int = args_str[i].parse::<i32>(); // int
        match int {
            Ok(int) => args.push(Value::Int(int)),
            Err(e) => {
                eprintln!("\x1b[31mERR:\x1b[0m {}", e);
                exit(0x666);
            }
        }

    } // }}}

    return args;
}

fn remove_first(s: &str) -> Option<&str> {
    s.chars().next().map(|c| &s[c.len_utf8()..])
} // }}}
