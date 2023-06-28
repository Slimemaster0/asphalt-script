// vim:fileencoding=utf-8:foldmethod=marker
// {{{ Modules
mod print;
mod lexer;
mod memory;
mod test;
mod errcodes;
mod process;
mod format;
mod read;
mod binops;
mod jump;
mod comp;
mod logic;
mod function;
// }}}

// {{{ Use
use crate::memory::*;
use crate::lexer::{ split_muti_points, remove_first };
use crate::function::{ Function, function_executor, fun_finder };
use crate::errcodes::*;
use crate::format::*;

use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::process::exit;
use std::thread;
// }}}

// {{{ Main function
fn main() {
    let args: Vec<String> = env::args().collect();

    let mut file = File::open(args[1].as_str()).expect("\x1b[31mERR:\x1b[0m Cannot open file!");
    let mut script = String::new();

    file.read_to_string(&mut script).expect("\x1b[31mERR:\x1b[0m Cannot read file!");

    init(&script);
}
// }}}

// {{{ init function - Break the script into functions and run the main function
pub fn init(file: &String) {
    let mut functions: Vec<Function> = Vec::new();

    // {{{ Parse functions
    // {{{ Backwards compatibility for scripts without functions
    if  !file.contains("fun!") &&
        !file.contains("boiler") &&
        !file.contains("{") &&
        !file.contains("}") {
        functions = vec![Function { name: String::from("boiler"), arguments: Vec::new(), arg_names: Vec::new(), arg_mut: Vec::new(), code: file.to_owned()}];
        // }}}
        // {{{ Parse functions
    } else {
        // {{{ Split the string
        let mut split: Vec<&str> = file.split("\nfun! ").collect();
        if split[0].find("fun! ") == Some(0) { // if there is no \n at the start of the first function.
            for _ in 0..5 {
                split[0] = remove_first(split[0]).expect("{RED}Err:{RESET_FORMAT} failed to find the first function! (Not to be confused with the boiler function)");
            }
        }
        // }}}
        // {{{ Split the substrings into functions
        for i in 0..split.len() {
            // {{{ Name of the function
            let name: String;
            {
                let name_spit: Vec<&str> = split[i].split("(").collect();
                name = name_spit[0].to_string();
            }
            // }}}
            // {{{ Extract the code
            let code: String;
            {
                let fn_start = split[i].find("{").expect("{RED}Err:{RESET_FORMAT} Cannot find start of the function \"{name[0]}\"!");
                let fn_end = split[i].rfind("}").expect("{RED}Err:{RESET_FORMAT} Cannot find end of the function \"{name[0]}\"!");
                let code_vec: Vec<String> = split_muti_points(&split[i].to_string(), &vec![fn_start + 1, fn_end]);
                code = code_vec[1].to_string();
            }
            // }}}
            // {{{ Extract the arguments
            let mut args: Vec<String> = Vec::new();
            let mut args_names: Vec<String> = Vec::new();
            let mut args_mut: Vec<bool> = Vec::new();
            {
                let args_start = split[i].find("(").expect("{RED}Err:{RESET_FORMAT} Cannot find start of arguments for \"{name}\"!");
                let args_end = split[i].find(")").expect("{RED}Err:{RESET_FORMAT} Cannot find end of arguments for \"{Name}\"!");
                let arg_str: Vec<String> = split_muti_points(&split[i].to_string(), &vec![args_start + 1, args_end]);
                let arg_str_split: Vec<&str> = arg_str[1].split(",").collect();

                for j in 0..arg_str_split.len() {
                    if arg_str_split[j].len() == 0 { continue; }
                    let arg_split: Vec<&str> = arg_str_split[j].split(":").collect();
                    args_names.push(arg_split[0].to_string());
                    match arg_split[1].find(" mut ") {
                        Some(_) => {
                            args_mut.push(true);
                            let arg_type_split: Vec<&str> = arg_split[1].split(" mut ").collect();
                            args.push(arg_type_split[1].trim().to_string())
                        }
                        None => {
                            args_mut.push(false);
                            args.push(arg_split[1].trim().to_string())
                        }
                    }
                        
                }
            }
            
            functions.push(Function {
                name: name,
                arguments: args,
                arg_names: args_names,
                arg_mut: args_mut,
                code: code.to_owned()
            });
            // }}}
        }
        // }}}
    }
    // }}}
    // }}}
    
    // Execute the "boiler" function
    let boilerplate_function_id = fun_finder(&functions, "boiler");
    function_executor(&functions[boilerplate_function_id], Vec::new(), &functions);
}
// }}}
