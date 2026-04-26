use std::fmt;
use crate::utils::errorutils::{self, error_print};

#[derive(Clone, PartialEq, Debug)]
pub enum LexTokens {
    LBracket,
    RBracket,
    Fn,
    KeyWord,
    LParen,
    RParen,
    Str(String),
    Num(i32),
}

pub struct Lexer {
    origin_text: Vec<char>, // collect to Vec<char> so indexing works
    i: usize,
    pub lex_tokens: Vec<LexTokens>,
}

impl Lexer {
    pub fn new(text: String) -> Lexer {
        Lexer { origin_text: text.chars().collect(), i: 0, lex_tokens: Vec::new() }
    }

    pub fn lex(&mut self) {
        while self.i < self.origin_text.len() {
            let current = self.origin_text[self.i];
            match current {
                '{'  => { self.lex_tokens.push(LexTokens::LBracket); self.i += 1; },
                '}'  => { self.lex_tokens.push(LexTokens::RBracket); self.i += 1; },
                '('  => { self.lex_tokens.push(LexTokens::LParen); self.i += 1; },
                ')'  => { self.lex_tokens.push(LexTokens::RParen); self.i += 1; },
                '\n' | ' ' | '\t' | '\r' => { self.i += 1; }, // whitespace before catchall
                s if s.is_alphabetic() => {
                    let token = self.lex_word();
                    self.lex_tokens.push(token);
                },
                s if s.is_numeric() => {
                    let token = self.lex_number();
                    self.lex_tokens.push(token);
                },
                _ => {
                    error_print(
                        errorutils::ErrorCodes::ErrorInvalidCharLiteralInLexication,
                        Some(&format!("token: {:?}", current)),
                    );
                }
            }
        }
    }

    fn lex_number(&mut self) -> LexTokens {
        let mut num = String::new();
        while self.i < self.origin_text.len() && self.origin_text[self.i].is_numeric() {
            num.push(self.origin_text[self.i]);
            self.i += 1;
        }
        match num.parse::<i32>() {
            Ok(n) => LexTokens::Num(n),
            Err(_) => {
                error_print(
                    errorutils::ErrorCodes::ErrorUnexpected,
                    Some(&format!("This is probably the compiler's fault")),
                );
            }
        }
    }

    fn lex_word(&mut self) -> LexTokens {
        let mut word = String::new();
        while self.i < self.origin_text.len() && self.origin_text[self.i].is_alphanumeric() {
            word.push(self.origin_text[self.i]);
            self.i += 1;
        }
        match word.as_str() {
            "fn" => LexTokens::Fn,
            _ => LexTokens::Str(word),
        }
    }

    pub fn print(&self) {
        for token in &self.lex_tokens {
            println!("{}", token);
        }
    }
}

impl fmt::Display for LexTokens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexTokens::LBracket  => write!(f, "Left Bracket: '{{'"),
            LexTokens::RBracket  => write!(f, "Right Bracket: '}}'"),
            LexTokens::LParen    => write!(f, "Left Paren: '('"),
            LexTokens::RParen    => write!(f, "Right Paren: ')'"),
            LexTokens::Fn        => write!(f, "Keyword: 'fn'"),
            LexTokens::KeyWord   => write!(f, "Keyword"),
            LexTokens::Str(s)    => write!(f, "String: '{}'", s),
            LexTokens::Num(n)    => write!(f, "Number: '{}'", n),
        }
    }
}
