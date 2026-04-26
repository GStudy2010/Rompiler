use std::{env, fs};
mod parser {
    pub mod main_parser;
}
mod lexer {
    pub mod main_lexer;
}
mod utils {
    pub mod fileutils;
    pub mod errorutils;
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        utils::errorutils::error_print(utils::errorutils::ErrorCodes::ErrorInvalidCommandLineArguments, None);
    }
    let program_name = &args[1];
    let code_check_program = utils::fileutils::check_program(program_name);
    if code_check_program != 0 {
        match code_check_program {
            1 => utils::errorutils::error_print(utils::errorutils::ErrorCodes::ErrorInvalidFileExtension, None),
            2 => utils::errorutils::error_print(utils::errorutils::ErrorCodes::ErrorNoSuchFile, None),
            _ => utils::errorutils::error_print(utils::errorutils::ErrorCodes::ErrorUnexpected, None),
        }
    }
    let contents: String = utils::fileutils::get_file_contents(program_name);
    println!("{}", contents);
    let mut lexer = lexer::main_lexer::Lexer::new(contents.clone());
    lexer.lex();
    lexer.print();
    let mut parser = parser::main_parser::Parser::new(lexer.lex_tokens);
    parser.parse();
    parser.print();
}
