use std::fmt;
use crate::{lexer::main_lexer::LexTokens, utils::errorutils::{ErrorCodes, error_print}};

pub enum TopLevelDecl {
    Mainfuncton(Mainfuncton),
}

pub enum InLifeTime {
    B(Block),
    S(Statment),
}

pub enum Block {
    Func(Function),
    LT(LifeTime),
}

pub enum Statment {
    SystemExit(SystemEx),
}

pub enum Expr {
    StrLit(StrLit),
    Number(Num)
}

pub struct SystemEx {
    value: Expr,
}

pub struct Mainfuncton {
    inside: Function,
}

pub struct Function {
    name: Expr,
    body: LifeTime,
}

pub struct StrLit {
    name: String,
}
pub struct Num {
    value: i32,
}

pub struct LifeTime {
    body: Vec<InLifeTime>,
}

pub struct Parser {
    lex_text: Vec<LexTokens>,
    pub parse_tokens: Option<TopLevelDecl>, // Option because it's empty until parse() runs
    current_token: usize,
}

impl Parser {
    pub fn new(lex_tokens: Vec<LexTokens>) -> Parser {
        Parser { lex_text: lex_tokens, parse_tokens: None, current_token: 0 }
    }

    pub fn parse(&mut self) {
        let decl = self.parse_top_level_decl();
        self.parse_tokens = Some(decl);
    }

    fn parse_top_level_decl(&mut self) -> TopLevelDecl {
        if self.get_token() == LexTokens::Fn {
            TopLevelDecl::Mainfuncton(Mainfuncton { inside: self.parse_function() })
        } else {
            error_print(ErrorCodes::ErrorNoEntryPoint, Some(&"Program has no function declaration on first line".to_string()));
        }
    }

    fn parse_function(&mut self) -> Function {
        self.next_token(); // skip 'fn', now on name
        let fname = self.parse_expr();
        self.next_token(); // move past name to '('
        self.assert_next_token(LexTokens::LParen);
        self.assert_next_token(LexTokens::RParen);
        self.assert_next_token(LexTokens::LBracket);
        let fbody = self.parse_lifetime();
        self.next_token(); // skip '}'
        Function { name: fname, body: fbody }
    }

    fn parse_lifetime(&mut self) -> LifeTime {
        let mut body: Vec<InLifeTime> = Vec::new();
        while self.current_token < self.lex_text.len() && self.get_token() != LexTokens::RBracket {
            body.push(self.parse_in_lifetime());
        }
        LifeTime { body }
    }

    fn parse_in_lifetime(&mut self) -> InLifeTime {
        match self.get_token() {
            LexTokens::Fn       => InLifeTime::B(Block::Func(self.parse_function())),
            LexTokens::LBracket => {
                self.next_token(); // skip '{'
                InLifeTime::B(Block::LT(self.parse_lifetime()))
            },
            _ => {
                let current_statment = self.parse_statment();
                match current_statment {
                    Statment::SystemExit(s) => {
                        InLifeTime::S(Statment::SystemExit(s))
                    }
                }
            }
        }
    }
    fn parse_statment(&mut self) -> Statment {
        match self.get_token() {
            LexTokens::SysExit => {
                self.next_token();
                self.assert_next_token(LexTokens::LParen);
                let value = self.parse_expr();
                self.next_token();
                self.assert_next_token(LexTokens::RParen);
                self.assert_next_token(LexTokens::SemiC);
                Statment::SystemExit(SystemEx { value })
            }
            _ => error_print(ErrorCodes::ErrorParsingError, Some(&("Not a statment").to_string())),
        }
    }
    fn parse_expr(&mut self) -> Expr {
        match self.get_token() {
            LexTokens::Str(s) => Expr::StrLit(StrLit { name: s }),
            LexTokens::Num(n) => Expr::Number(Num { value: n }),
            _ => error_print(ErrorCodes::ErrorInvalidExpretion, Some(&("Expression not handled").to_string())),
        }
    }

    // Checks current token matches, then advances
    fn assert_next_token(&mut self, assumed_token: LexTokens) {
        if assumed_token != self.get_token() {
            error_print(ErrorCodes::ErrorAsertionFailed, Some(&format!("Expected {:?} while parsing", assumed_token)));
        }
        self.next_token();
    }

    fn next_token(&mut self) {
        self.current_token += 1;
    }

    fn get_token(&self) -> LexTokens {
        self.lex_text[self.current_token].clone()
    }

    pub fn print(&self) {
        match &self.parse_tokens {
            Some(decl) => println!("{}", decl),
            None => println!("Nothing parsed yet"),
        }
    }
}

impl fmt::Display for TopLevelDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TopLevelDecl::Mainfuncton(m) => write!(f, "{}", m.inside),
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fn {}:{}", self.name, self.body)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::StrLit(s) => write!(f, "{}", s.name),
            Expr::Number(n) => write!(f, "{}", n.value),
        }
    }
}

impl fmt::Display for LifeTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{{")?;
        for item in &self.body {
            writeln!(f, "  {}", item)?;
        }
        write!(f, "}}")
    }
}

impl fmt::Display for InLifeTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InLifeTime::B(b) => write!(f, "{}", b),
            InLifeTime::S(s) => write!(f, "{}", s),
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Block::Func(func) => write!(f, "{}", func),
            Block::LT(lt)     => write!(f, "{}", lt),
        }
    }
}
impl fmt::Display for Statment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statment::SystemExit(s) => write!(f, "sysexit({});", s.value)
        }
    }
}
