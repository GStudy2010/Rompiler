use std::fs;
pub fn check_program(program: &str) -> i32 {
    if !(program.ends_with(".masm") && program.len() > 3 && !program.starts_with(".")) {
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
pub fn get_file_contents(program: &str) -> Vec<String> {
    fs::read_to_string(program)
        .expect("Failed to read file")
        .lines()
        .map(String::from)
        .collect()
}

