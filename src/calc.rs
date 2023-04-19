

use crate::{lexer::Lexer, parser::parse};



pub struct Calc;


impl Calc {
    pub fn new() -> Self {
        Self
    }

    pub fn run_expr(&mut self, expr: &str)  {
        let mut lexer_obj=Lexer::new();
        Lexer::lex(&mut lexer_obj,&expr);
        parse(&lexer_obj.Tokens);
    }
}
