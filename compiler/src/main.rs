use std::{env, fs};
fn error_print(s: &str, code: i32) {
    println!("{}", s);
    std::process::exit(code);
}
fn check_program(program: &str) -> i32 {
    if !(program.ends_with(".ri") && program.len() > 3 && !program.starts_with(".")) {
        1
    } else {
        let is_file = fs::exists(program).expect("Can't check this file");
        if !is_file {
            2
        } else {
            0
        }
    }
}
fn get_file_contents(program: &str) -> String {
    fs::read_to_string(program).expect("Can't check the file")
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        error_print("Invalid amount of command line arguments: ./rompiler file.ri", 1);
    }
    let program_name = &args[1];
    let code_check_program = check_program(program_name);
    if code_check_program != 0 {
        match code_check_program {
            1 => error_print("Invalid file extension", 1),
            2 => error_print("No such file", 1),
            _ => error_print("No way", 1),
        }
    }
    let contents: String = get_file_contents(program_name);
}
