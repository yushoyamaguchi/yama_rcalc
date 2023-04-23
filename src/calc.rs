

use crate::{lexer::Lexer, parser::Parser};



pub struct Calc;


impl Calc {
    pub fn new() -> Self {
        Self
    }

    pub fn run_expr(&mut self, expr: &str)  {
        let mut lexer_obj=Lexer::new();
        Lexer::lex(&mut lexer_obj,&expr);
        let mut parser_obj=Parser::new();
        Parser::parse(&mut parser_obj,&lexer_obj.Tokens);
    }
}
