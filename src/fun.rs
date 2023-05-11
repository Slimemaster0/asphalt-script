// vim:fileencoding=utf-8:foldmethod=marker

use std::process::exit;

use crate::print::*;
use crate::memory::*;
use crate::test::*;
use crate::errcodes::*;
use crate::process::*;

pub fn fun(input: &str, stack: &mut Vec<Item>) -> Value {
    let keyword: Vec<&str> = input.split("(").collect();
    if keyword.len() != 1 {
        match keyword[0].to_owned().as_str().trim() {
            "printf" => printf(parse_args(input, stack)),
            "new" => new_var(parse_args(input, stack), stack),
            "test_parse_args" => test_parse_args(parse_args(input, stack)),
            "die" => die(parse_args(input, stack)),

            _ => eprintln!("\x1b[31mERR:\x1b[0m {}", input),
        }
    }
    return Value::Null;
}

pub fn parse_args(str: &str, stack: &mut Vec<Item>) -> Vec<Value> { // {{{

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

            let raw_content: Vec<&str> = args_str[i].split("\"").collect();

            let content_split: Vec<&str> = raw_content[1].split("\\n").collect();
            let mut content = String::new();

            for i in 0..content_split.len() {
                content.push_str(content_split[i]);
                if i != content_split.len() -1 {
                    content.push_str("\n");
                }
            }

            args.push(Value::String(content));
            continue;

        }  else if args_str[i].chars().nth(args_str[i].len() - 1).expect("add -1 to float checking") == 'F' { // Floating point number

            //  args.push();
            eprintln!("Float is not implemented yet");
            exit(NOT_IMPL);

        } else if args_str[i].contains("&") {
            let valname: Vec<&str> = args_str[i].as_str().split("&").collect();
            let ref val = stack[read_pointer(stack, valname[1])];

            args.push(val.value.to_owned());

            continue;   
        } else if args_str[i] == "yup" {
            args.push(Value::Bool(true));
            continue;
        } else if args_str[i] == "nope" {
            args.push(Value::Bool(false));
            continue;
        }

        let int = args_str[i].parse::<i32>(); // int
        match int {
            Ok(int) => args.push(Value::Int(int)),
            Err(e) => {
                eprintln!("\x1b[31mErr:\x1b[0m {}", e);
                exit(FAILED_PARSE);
            }
        }

    } // }}}

    return args;
}

fn remove_first(s: &str) -> Option<&str> {
    s.chars().next().map(|c| &s[c.len_utf8()..])
} // }}}
