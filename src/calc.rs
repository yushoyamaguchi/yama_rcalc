use crossterm::{execute, style::Print};

use crate::{lexer::Lexer, parser::Parser};



pub struct Calc;


impl Calc {
    pub fn new() -> Self {
        Self
    }

    pub fn run_expr(&mut self, expr: &str)  {
        let mut lexer_obj=Lexer::new();
        let lex_result=Lexer::lex(&mut lexer_obj,&expr);
        match lex_result{
            Some(lex_error)=>{
                match lex_error.value{
                    crate::lexer::LexErrorKind::InvalidChar(c)=>{
                        execute!(std::io::stdout(),Print("There was an error during lexing: "), Print(c), Print("\r\n")).ok();
                        return;
                    }
                }
            }
            None=>{
                execute!(std::io::stdout(),Print("Lexing was successful"), Print("\r\n")).ok();
            }
        }
        let mut parser_obj=Parser::new();
        Parser::parse(&mut parser_obj,&lexer_obj.Tokens);
    }
}
