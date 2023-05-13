use core::num;

use crossterm::{execute, style::Print};

use crate::{lexer::Lexer, parser::{Parser, Expr}};





pub struct Calc;

fn recur(expr:&mut Expr){
    execute!(std::io::stdout(),Print("Plus found (This is for AST access check)"), Print("\r\n")).ok();
    if expr.get_is_single()==false {
        let right_expr_ptr=expr.get_right();
        match right_expr_ptr{
            Some(right_expr)=>{
                recur(right_expr);
            }
            None=>{
                return;
            }
        }
        return;
    }
}

pub fn num_of_plus(parser: &mut Parser){
    let root_expr=&mut parser.root_expr;
    recur(root_expr);
}

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
        let parse_result=Parser::parse(&mut parser_obj,&lexer_obj.Tokens);
        match parse_result{
            Err(parse_error)=>{
                match parse_error.value{
                    crate::parser::ParseErrorKind::UnexpectedToken(token)=>{
                        execute!(std::io::stdout(),Print("There was an error during parsing: "), Print(token), Print("\r\n")).ok();
                        return;
                    }
                }
            }
            Ok(n)=>{
                execute!(std::io::stdout(),Print("Parsing was successful : Ansewer="),Print(n), Print("\r\n")).ok();
            }
        }
        num_of_plus(&mut parser_obj);
    }
}
