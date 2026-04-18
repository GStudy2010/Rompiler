pub enum ErrorCodes {
    ErrorNoSuchFile,
    ErrorInvalidFileExtension,
    ErrorUnexpected,
    ErrorInvalidCommandLineArguments,
    ErrorInvalidCharLiteralInLexication,
    ErrorParsingError,
}
pub fn error_print(code: ErrorCodes, additional_info: Option<&String>) {
    match code {
        ErrorCodes::ErrorNoSuchFile => { println!("No such file"); println!("{:?}", additional_info); std::process::exit(1)}
        ErrorCodes::ErrorInvalidFileExtension => { println!("Invalid file extension in compilation"); println!("{:?}", additional_info); std::process::exit(2)}
        ErrorCodes::ErrorUnexpected => { println!("Unexpected error in execution of rompiler"); println!("{:?}", additional_info); std::process::exit(3)}
        ErrorCodes::ErrorInvalidCommandLineArguments => { println!("Invalid command line arguments"); println!("{:?}", additional_info);std::process::exit(4)}
        ErrorCodes::ErrorInvalidCharLiteralInLexication => { println!("Invalid character in lexicating process"); println!("{:?}", additional_info); std::process::exit(5)}
        ErrorCodes::ErrorParsingError => { println!("Error while parsing"); println!("{:?}", additional_info); std::process::exit(1)}
    }
}
