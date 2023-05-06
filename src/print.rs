use crate::memory::*;

pub fn printf(string: &str) {
    let tmp_str1: Vec<&str> = string.split("(").collect();
    let content: Vec<&str> = tmp_str1[1].split("(").collect();

    let args: Vec<&str> = content[0].split("\"").collect();
    let output_split: Vec<&str> = args[1].split("\\n").collect();
    let mut output = String::new();

    for i in 0..output_split.len() {
        output.push_str(output_split[i]);
        if i != output_split.len() -1 {
            output.push_str("\n");
        }
    }

    print!("{}", output);
}


pub fn print_var(string: &str, stack: &mut Vec<Item>, ptr_stack: &mut Vec<Pointer>) {
    let tmp_str1: Vec<&str> = string.split("(").collect();
    let content: Vec<&str> = tmp_str1[1].split(")").collect();

    let var = deref_pointer(&ptr_stack, &content[0], stack);

    match var {
        Item::Int(i) => print!("{}", i),
    }

}
