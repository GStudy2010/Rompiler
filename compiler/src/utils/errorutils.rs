pub enum ErrorCodes {
    ErrorNoSuchFile,
    ErrorInvalidFileExtension,
    ErrorUnexpected,
    ErrorInvalidCommandLineArguments,
    ErrorInvalidCharLiteralInLexication,
    ErrorParsingError,
    ErrorNoEntryPoint,
    ErrorInvalidExpretion,
    ErrorAsertionFailed,
    ErrorInvalidInLifetimeStatment,
}
pub fn error_print(code: ErrorCodes, additional_info: Option<&String>) -> ! {
    match code {
        ErrorCodes::ErrorNoSuchFile                     => { println!("No such file"); println!("{:?}", additional_info); std::process::exit(1)}
        ErrorCodes::ErrorInvalidFileExtension           => { println!("Invalid file extension in compilation"); println!("{:?}", additional_info); std::process::exit(2)}
        ErrorCodes::ErrorUnexpected                     => { println!("Unexpected error in execution of rompiler"); println!("{:?}", additional_info); std::process::exit(3)}
        ErrorCodes::ErrorInvalidCommandLineArguments    => { println!("Invalid command line arguments"); println!("{:?}", additional_info);std::process::exit(4)}
        ErrorCodes::ErrorInvalidCharLiteralInLexication => { println!("Invalid character in lexicating process"); println!("{:?}", additional_info); std::process::exit(5)}
        ErrorCodes::ErrorParsingError                   => { println!("Error while parsing"); println!("{:?}", additional_info); std::process::exit(1)}
        ErrorCodes::ErrorNoEntryPoint                   => { println!("No main function"); println!("{:?}", additional_info); std::process::exit(1)}
        ErrorCodes::ErrorInvalidExpretion               => { println!("error while parsing expresion"); println!("{:?}", additional_info); std::process::exit(1)}
        ErrorCodes::ErrorAsertionFailed                 => { println!("Some Assert statment failed"); println!("{:?}", additional_info); std::process::exit(1)}
        ErrorCodes::ErrorInvalidInLifetimeStatment      => { println!("Invalid statment in lifetime"); println!("{:?}", additional_info); std::process::exit(1)}
    }
}
