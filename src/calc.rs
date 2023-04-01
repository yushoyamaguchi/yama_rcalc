
use crate::lexer::Lexer;



pub struct Calc;


impl Calc {
    pub fn new() -> Self {
        Self
    }

    pub fn run_expr(&mut self, expr: &str)  {
        let tokens=Lexer::lex(&mut Lexer::new(),&expr);
        //let len_str=length.to_string();
        //execute!(std::io::stdout(),Print(len_str), Print("\r\n")).ok();
    }
}
