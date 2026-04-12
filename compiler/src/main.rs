use std::{env, fs};
mod utils {
    pub mod fileutils;
    pub mod errorutils;
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        utils::errorutils::error_print(utils::errorutils::ErrorCodes::ErrorInvalidCommandLineArguments);
    }
    let program_name = &args[1];
    let code_check_program = utils::fileutils::check_program(program_name);
    if code_check_program != 0 {
        match code_check_program {
            1 => utils::errorutils::error_print(utils::errorutils::ErrorCodes::ErrorInvalidFileExtension),
            2 => utils::errorutils::error_print(utils::errorutils::ErrorCodes::ErrorNoSuchFile),
            _ => utils::errorutils::error_print(utils::errorutils::ErrorCodes::ErrorUnexpected),
        }
    }
    let contents: String = utils::fileutils::get_file_contents(program_name);
    println!("{}", contents);
}
