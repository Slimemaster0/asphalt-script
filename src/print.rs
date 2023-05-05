pub fn printf(string: &str) {
    let tmp_str1: Vec<&str> = string.split("(").collect();
    let content: Vec<&str> = tmp_str1[1].split("(").collect();

    let string: Vec<&str> = content[0].split("\"").collect();
    println!("{}", string[1]);
}
