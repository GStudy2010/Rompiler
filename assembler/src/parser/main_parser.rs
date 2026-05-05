use std::char;
use crate::utils::errorutils::{ErrorCodes, error_print};

struct BinaryData {
    binary: Vec<char>,
}

pub struct Parser {
    is_comment_line: bool,
    assembly_text: Vec<String>,
    ready_binary: BinaryData,
}

impl Parser {
    pub fn new(text: Vec<String>) -> Parser {
        Parser { is_comment_line: false, assembly_text: text, ready_binary: BinaryData { binary: Vec::new() } }
    }

    pub fn parse(&mut self) -> Vec<char> {
        for i in self.assembly_text.clone() {
            self.parse_statment(i);
        }
        self.ready_binary.binary.clone()
    }

    fn parse_statment(&mut self, line: String) {
        // Skip comments and empty lines
        let line = line.trim();
        if line.starts_with("/*") {
            self.is_comment_line = true;
        }
        if line.starts_with("*/") {
            self.is_comment_line = false;
        }
        if line.is_empty() || line.starts_with("*/") || line.starts_with("//") || self.is_comment_line {
            return;
        }

        // Strip trailing '.'
        let line = line.trim_end_matches('.');

        // Skip labels like "_start:"
        if line.ends_with(':') {
            for _ in 0..6 {
                self.push_opcode('f');
            }
            return;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.is_empty() { return; }

        match tokens[0] {
            "MOV" => self.parse_mov(tokens.get(1), false),
            "MOVR" => self.parse_mov(tokens.get(1), true),
            "NOT" => {
                self.push_opcode('2');
                if let Some(reg) = tokens.get(1) {
                    self.push_register(reg);
                }
                self.push_register("NREG"); // no second arg
            }
            "OR"  => self.parse_binary_op('3', &tokens),
            "XOR" => self.parse_binary_op('4', &tokens),
            "AND" => self.parse_binary_op('5', &tokens),
            "SYSC" => {
                self.push_opcode('6');
                self.push_register("NREG");
                self.push_register("NREG");
            }
            s => error_print(ErrorCodes::ErrorUnexpected, Some(&format!("Invalid syntax: {}", s))),
        }
    }

    // Parses: MOV A->B or MOV 0->RARX
    fn parse_mov(&mut self, arg: Option<&&str>, is_movr: bool) {
        let opcode = if is_movr { '1' } else { '0' };
        self.push_opcode(opcode);

        match arg {
            Some(expr) => {
                let parts: Vec<&str> = expr.split("->").collect();
                if parts.len() != 2 {
                    error_print(ErrorCodes::ErrorUnexpected, Some(&"Invalid MOV syntax".to_string()));
                }
                self.push_register_or_immediate(parts[0]);
                self.push_register(parts[1]);
            }
            None => error_print(ErrorCodes::ErrorUnexpected, Some(&"MOV missing argument".to_string())),
        }
    }

    // Parses: OR A, B / XOR A, B / AND A, B
    fn parse_binary_op(&mut self, opcode: char, tokens: &[&str]) {
        self.push_opcode(opcode);
        // tokens[1] is "A," and tokens[2] is "B"
        let arg1 = tokens.get(1).map(|s| s.trim_end_matches(','));
        let arg2 = tokens.get(2);
        match (arg1, arg2) {
            (Some(a), Some(b)) => {
                self.push_register(a);
                self.push_register(b);
            }
            _ => error_print(ErrorCodes::ErrorUnexpected, Some(&"Binary op missing arguments".to_string())),
        }
    }

    fn push_opcode(&mut self, code: char) {
        self.ready_binary.binary.push(code);
    }

    // Handles both register names and raw immediates like '0', '1', '255'
    fn push_register_or_immediate(&mut self, token: &str) {
        if token.chars().all(|c| c.is_ascii_digit()) {
            // It's a numeric immediate — encode as 'i' followed by the value chars
            self.ready_binary.binary.push('i');
            for c in token.chars() {
                self.ready_binary.binary.push(c);
            }
            self.ready_binary.binary.push(' '); // delimiter
        } else {
            self.push_register(token);
        }
    }

    fn push_register(&mut self, reg: &str) {
        let code = match reg {
            "NREG" => '0',
            "A"    => '1',
            "B"    => '2',
            "C"    => '3',
            "D"    => '4',
            "RA"   => '5',
            "RB"   => '6',
            "RC"   => '7',
            "RD"   => '8',
            "RAR"  => '9',
            // Multi-digit register codes need special handling below
            "RBR"  | "RCR"  | "RDR"  |
            "RARX" | "RBRX" | "RCRX" | "RDRX" => {
                self.push_wide_register(reg);
                return;
            }
            _ => {
                error_print(ErrorCodes::ErrorUnexpected, Some(&format!("Unknown register: {}", reg)));
            }
        };
        self.ready_binary.binary.push(code);
    }

    // Registers with opcode > 9 need two chars
    fn push_wide_register(&mut self, reg: &str) {
        let code = match reg {
            "RBR"  => "10",
            "RCR"  => "11",
            "RDR"  => "12",
            "RARX" => "13",
            "RBRX" => "14",
            "RCRX" => "15",
            "RDRX" => "16",
            _ => {
                error_print(ErrorCodes::ErrorUnexpected, Some(&format!("Unknown register: {}", reg)));
            }
        };
        for c in code.chars() {
            self.ready_binary.binary.push(c);
        }
    }
}
