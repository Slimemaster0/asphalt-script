// vim:fileencoding=utf-8:foldmethod=marker
// {{{ use
use std::process::exit;
use std::env;

use crate::print::*;
use crate::memory::*;
use crate::test::*;
use crate::errcodes::*;
use crate::process::*;
use crate::format::*;
use crate::read::read_to_string;
use crate::binops::binops;
use crate::jump::*;
use crate::comp::*;
use crate::logic::*;
// }}}

pub fn fun(input: &str, stack: &mut Vec<Item>, line_num: &mut u64) -> Value {
    if input.contains("(") && input.contains(")") {
    let keyword: Vec<&str> = input.split("(").collect();
        match keyword[0].to_owned().as_str().trim() {
            "printf" => printf(parse_args(input, stack, line_num)),
            "test_parse_args" => test_parse_args(parse_args(input, stack, line_num)),
            "die" => die(parse_args(input, stack, line_num)),
            "readf" => return read_to_string(parse_args(input, stack, line_num)),
            "return" => return parse_args(input, stack, line_num)[0].clone(),
// {{{ Memory
            "del" => del_var(parse_args(input, stack, line_num), stack),
            "new" => new_var(parse_args(input, stack, line_num), stack),
            "mut" => mut_var(parse_args(input, stack, line_num), stack),
// }}}
// {{{ binary operations
            "add" => return binops(parse_args(input, stack, line_num), '+'),
            "sub" => return binops(parse_args(input, stack, line_num), '-'),
            "mul" => return binops(parse_args(input, stack, line_num), '*'),
            "div" => return binops(parse_args(input, stack, line_num), '/'),
            // }}}
// {{{ jump
            "jmp" => jump(parse_args(input, stack, line_num), line_num),
            "jmpif" => jumpif(parse_args(input, stack, line_num), line_num),
            "jmpto" => jumpto(parse_args(input, stack, line_num), line_num),
// }}}
// {{{ logic gates
            "&&" => return and(parse_args(input, stack, line_num)),
            "||" => return or(parse_args(input, stack, line_num)),
            "!" => return not(parse_args(input, stack, line_num)),
// }}}
// {{{ comp
            "intcmp" => return intcmp(parse_args(input, stack, line_num)),
            "strcmp" => return strcmp(parse_args(input, stack, line_num)),
// }}}

            _ => eprintln!("\x1b[31mERR:\x1b[0m {}", input),
        }
    }
    return Value::Null;
}

pub fn parse_args(str: &str, stack: &mut Vec<Item>, line_num: &mut u64) -> Vec<Value> { // {{{

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


    match env::var("DEBUG_PARSER") {
        Ok(v) => {
            if v == "1" {
                eprintln!("Arguments string: {string}");
            }
        }
        Err(_) => {}
    }

    // {{{ Split the argument string
    let mut is_str: bool = false;
    let mut parens: usize = 0;
    let mut split_points: Vec<usize> = Vec::new();
    let chars: Vec<char> = string.chars().collect();

    for i in 0..chars.len() {
        if chars[i] == '"' {
            if is_str {
                is_str = false;
            } else {
                is_str = true;
            }
            continue;
        }

        if chars[i] == '(' {
            parens += 1;
        }
        if chars[i] == ')' {
            parens -= 1;
        }

        if parens == 0 && is_str == false && chars[i] == ',' {
            split_points.push(i);
        }
    }

    let mut args_str: Vec<String> = split_muti_points(&string, &split_points);

    for i in 0..args_str.len() {
        if args_str[i].chars().nth(0).expect("{RED}Err:{RESET_FORMAT} No char at 0") == ',' {
            let rm_first = remove_first(args_str[i].as_str()).expect("{RED}Err:{RESET_FORMAT} Could not remove first!");
            args_str[i] = rm_first.trim().to_string();
            continue;
        } else {
            args_str[i] = args_str[i].trim().to_string();
        }
    }

    // }}}

    match env::var("DEBUG_PARSER") {
        Ok(v) => {
            if v == "1" {
                for i in 0..args_str.len() {
                    eprintln!("Arg{}: {}", i, args_str[i]);
                }
            }
        }
        Err(_) => {}
    }


    let mut args: Vec<Value> = Vec::new();

    for i in 0..args_str.len() { // {{{ parse each argument

        if args_str[i].contains("(") && args_str[i].chars().nth(args_str[i].len() -1).expect("No char at {args_str[i] -1}") == ')' { // function
            args.push(fun(&args_str[i], stack, line_num));
            continue;

        } else if args_str[i].contains("\"") { // {{{ String

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
// }}}
        } else if args_str[i].contains("'") && args_str[i].chars().nth(args_str[i].len() -1).expect("No char at {args_str[i] -1}") == '\'' { // char
            if args_str[i].contains("'\\") { // Escape codes
                match args_str[i].trim().chars().nth(2).expect("No char at 2") {
                    'n' => args.push(Value::Char('\n')),
                    't' => args.push(Value::Char('\t')),
                    '0' => args.push(Value::Char('\0')),
                    '\\' => args.push(Value::Char('\\')),
                    
                    _ => {
                        eprintln!("{RED}ERR:{RESET_FORMAT} Unknown escape code: \'\\{}\'", args_str[i].trim().chars().nth(2).expect("No char at 2"));
                        exit(INVALID_ESCAPE_CODE);
                    }
                }
            } else {
                args.push(Value::Char(args_str[i].trim().chars().nth(1).expect("No char at 1")));
            }
            continue;
            
        }  else if args_str[i].chars().nth(args_str[i].len() - 1).expect("add -1 to float checking") == 'F' { // Floating point number

            //  args.push();
            eprintln!("Float is not implemented yet");
            exit(NOT_IMPL);

        } else if args_str[i].chars().nth(0).expect("No char at 0") == '&' {
            let valname: Vec<&str> = args_str[i].as_str().split("&").collect();
            let ref val = stack[read_pointer(stack, valname[1])];

            args.push(val.value.to_owned());

            continue;   

        } else if args_str[i] == "yup" { // {{{ Boolean
            args.push(Value::Bool(true));
            continue;
        } else if args_str[i] == "nope" {
            args.push(Value::Bool(false));
            continue;

            // }}}
        } else if args_str[i].chars().nth(0).expect("No char at 0") == '$' { // {{{ environment variables and program arguments
            let name = remove_first(args_str[i].as_str()).expect("{RED}Err:{RESET_FORMAT} Could not remove first character");
            
            if name.contains("arg") { // Reading an argument
                let number_str: Vec<&str> = name.split("arg").collect();
                let number = number_str[1].parse::<usize>().expect("{RED}Err:{RESET_FORMAT} argument are read as $arg{BOLD}n{RESET_FORMAT}");

                let argv: Vec<String> = env::args().collect();
                args.push(Value::String(argv[number +1].clone()));
                continue;
            } else if name.contains("os") {
                args.push(Value::String(env::consts::OS.to_string()));
                continue;
            } else if args_str[i] .contains("version") {
                args.push(Value::String(env!("CARGO_PKG_VERSION").to_string()));
                continue;
            }

            match env::var(name) { // Reading an environment variable
                Ok(v) => args.push(Value::String(v)),
                Err(v) => panic!("{RED}Err:{RESET_FORMAT} {v}")
            }

            continue;
// }}}
        } else if args_str[i].chars().nth(0).expect("No char at 0") == '¤' {
            match args_str[i].as_str() {

                _ => {
                    eprintln!("{RED}Err:{RESET_FORMAT} Unknown variable: {}", args_str[i]);
                    exit(ITEM_NOT_EXIST);
                }
            }
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

fn split_muti_points(str: &String, points: &Vec<usize>) -> Vec<String> { // {{{
    let mut return_val: Vec<String> = Vec::new();

    if points.len() == 0 {
        return_val.push(str.to_owned());
        return return_val;
    } else {
        return_val.push(str.split_at(points[0]).0.to_string());
    }

    for i in 0..points.len() -1 {
        return_val.push(str.split_at(points[i]).1.split_at(points[i+1] - points[i]).0.to_string());
    }

    return_val.push(str.split_at(points[points.len() -1]).1.to_string());


    return return_val;
} // }}}
