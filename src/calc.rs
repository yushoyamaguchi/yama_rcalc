use crate::parser;
use crate::parser::parse;
use crate::lexer;
use tracing::debug;
use crate::lexer::Lexer;

use crossterm::style::{Print};
use crossterm::{execute};
use std::io::Write;

pub struct Calc;


impl Calc {
    pub fn new() -> Self {
        Self
    }

    pub fn run_expr(&mut self, expr: &str)  {
        let tokens=Lexer::lex(&Lexer::new(),&expr);
        //let len_str=length.to_string();
        //execute!(std::io::stdout(),Print(len_str), Print("\r\n")).ok();
    }
}
