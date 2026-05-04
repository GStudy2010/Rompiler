use crate::{parser::main_parser::{Block, Expr, Function, InLifeTime, LifeTime, Mainfuncton, Statment, TopLevelDecl}, utils::errorutils::{ErrorCodes, error_print}};

pub struct AsmGenerator {
    source_tree: Option<TopLevelDecl>,
    indent: usize,
    pub source_code: Vec<String>,
}
impl AsmGenerator {
    pub fn new(source: Option<TopLevelDecl>) -> AsmGenerator {
        AsmGenerator { source_tree: source, indent: 0, source_code: Vec::new() }
    }
    pub fn generate(&mut self) {
        match self.source_tree.clone() { // This shit might make some troubles ( imma copy a 3 gig
                                         // file tree for funzies
            None => error_print(ErrorCodes::ErrorNoEntryPoint, Some(&("While generating assembly found out that file is empty").to_string())),
            Some(t) => {
                self.generate_from_top_level_decl(t.clone());
            }
        }
    }
    fn write_to_source(&mut self, line: String) {
        self.source_code.push(format!("{}{}", " ".repeat(self.indent), line));
    }
    // I should implement a table of registers to keep track of their values instead of xoring them
    // every time.

    // Also This whole file will need to be re-written as soon as I start implementing binary
    // expresions that colapse into values.
    fn generate_statment(&mut self, s: Statment) {
        match s {
            Statment::SystemExit(n) => {
                match n.value {
                    Expr::StrLit(_) => error_print(ErrorCodes::ErrorUnexpected, Some(&"The value is an string, found out in generation process".to_string())),
                    Expr::Number(nu) => {
                        if nu.value >= 256 {
                            error_print(ErrorCodes::ErrorToBigReturnValue,Some(&format!("return value is {} when the max is 255", n.value)));
                        }
                        self.write_to_source("MOV 0->RARX".to_string());
                        self.write_to_source(format!("MOV {}->RBRX", n.value));
                        self.write_to_source("SYSC".to_string());
                    }
                }

            }
        }
    }
    fn generate_function(&mut self, f: Function) {
        let name = match f.name {
            Expr::StrLit(s) => {
                if s.name == "main" {
                    "_start".to_string()
                } else {
                    s.name
                }
            },
            _ => error_print(ErrorCodes::ErrorInvalidExpretion, Some(&("Found int where String expected").to_string())),
        };
        self.write_to_source(format!("{}:", name));
        self.indent = 2;
        self.generate_scope(f.body);
        self.indent = 0;
    }
    fn generate_scope(&mut self, b: LifeTime) {
        for i in 0..b.body.len() {
            match b.body[i].clone() {
                InLifeTime::B(bl) => {
                    match bl {
                        Block::LT(l) => self.generate_scope(l),
                        Block::Func(f) => self.generate_function(f),
                    }
                }
                InLifeTime::S(st) => self.generate_statment(st),
            }
        }
    }
    fn generate_main_function(&mut self, m: Mainfuncton) {
        let func = m.inside;
        self.generate_function(func);
    }
    fn generate_from_top_level_decl(&mut self, t: TopLevelDecl) {
        match t {
            TopLevelDecl::Mainfunction(m) => self.generate_main_function(m),
        }
    }
    pub fn print(&self) {
        for i in &self.source_code {
            println!("{}", i);
        }
    }
}
