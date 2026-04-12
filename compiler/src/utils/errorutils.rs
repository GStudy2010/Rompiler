pub enum ErrorCodes {
    ErrorNoSuchFile,
    ErrorInvalidFileExtension,
    ErrorUnexpected,
    ErrorInvalidCommandLineArguments,
}
pub fn error_print(code: ErrorCodes) {
    match code {
        ErrorCodes::ErrorNoSuchFile => { println!("No such file"); std::process::exit(1)}
        ErrorCodes::ErrorInvalidFileExtension => { println!("Invalid file extension in compilation"); std::process::exit(2)}
        ErrorCodes::ErrorUnexpected => { println!("Unexpected error in execution of rompiler"); std::process::exit(3)}
        ErrorCodes::ErrorInvalidCommandLineArguments => { println!("Invalid command line arguments"); std::process::exit(4)}
    }
}
