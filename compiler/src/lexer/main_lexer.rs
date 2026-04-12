use std::fmt;

use crate::utils::errorutils::{self, error_print};

pub enum LexTokens {
    LBracket,
    RBracket,
}
pub struct Lexer {
    origin_text: String,
    i: usize,
    pub lex_tokens: Vec<LexTokens>
}
impl Lexer {
    pub fn new(text: String) -> Lexer {
        Lexer { origin_text: text, i: 0, lex_tokens: Vec::new() }
    }
    pub fn lex(&mut self) {
        while self.i < self.origin_text.len() {
            let current = self.origin_text.chars().nth(self.i);
            match current {
                Some('{')  => {self.lex_tokens.push(LexTokens::LBracket); self.i += 1},
                Some('}')  => {self.lex_tokens.push(LexTokens::RBracket); self.i += 1},
                Some('\n') => {self.i += 1},
                None       => break,
                _          => {error_print(errorutils::ErrorCodes::ErrorInvalidCharLiteralInLexication, Some(&format!("token: {:?}", current.clone())));}
            }
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
         LexTokens::LBracket => write!(f, "Left Bracket: '{{'"),
         LexTokens::RBracket => write!(f, "Right Bracket: '}}'"),
     }
 }
}
