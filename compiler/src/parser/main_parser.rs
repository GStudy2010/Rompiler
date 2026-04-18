use crate::{lexer::main_lexer::LexTokens, utils::errorutils::{ErrorCodes, error_print}};
pub enum ParseTokens {
    LifeTime(Option<Box<ParseTokens>>),
}
pub struct Parser {
    lex_text: Vec<LexTokens>,
    pub parse_tokens: Vec<ParseTokens>,
    current_token: i32,
}
impl Parser {
    pub fn new(lex_tokens: Vec<LexTokens>) -> Parser {
        Parser { lex_text: lex_tokens, parse_tokens: Vec::new(), current_token: 0 }
    }
    pub fn parse(&mut self) {
        for lex_token in &self.lex_text {
            match lex_token {
                LexTokens::LBracket => {self.parse_tokens.push(self.parse_scope());},
                LexTokens::RBracket => error_print(ErrorCodes::ErrorParsingError, Some(&format!("'}}' unexpected before a '{{'"))),
            }
        }
    }
    fn next_token(&mut self) {
        self.current_token += 1;
    }
    fn get_token(&self) -> LexTokens {
        return self.lex_text[self.current_token];
    }
    fn parse_scope(&self) -> ParseTokens {
    }
}
