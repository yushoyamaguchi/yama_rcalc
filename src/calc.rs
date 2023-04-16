
use crossterm::{execute, style::Print};

use crate::lexer::Lexer;



pub struct Calc;


impl Calc {
    pub fn new() -> Self {
        Self
    }

    pub fn run_expr(&mut self, expr: &str)  {
        let mut lexer_obj=Lexer::new();
        Lexer::lex(&mut lexer_obj,&expr);
        execute!(std::io::stdout(),Print("num of Vec items="),Print(lexer_obj.Tokens.len()), Print("\r\n")).ok();
    }
}
