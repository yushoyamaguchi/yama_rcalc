use crossterm::style::{Print};
use crossterm::{execute};
use std::io::Write;



pub enum TokenKind {
    Number(i32),
    Plus,
    Minus,
    Asterisk,
    Slash
}

pub enum LexErrorKind{
    InvalidChar(char),
}

pub struct Annot<T> {
    value: T,
}

impl<T> Annot<T> {
    fn new(value: T) -> Self {
        Self { value}
    }
}

type LexError=Annot<LexErrorKind>;

impl LexError{
    fn invalid_char(c: char)->Self{
        Self::new(LexErrorKind::InvalidChar(c))
    }
}

type Token=Annot<TokenKind>;

impl Token{


    pub fn number(n:i32)-> Self{
        Self::new(TokenKind::Number(n))
    }

    pub fn plus()-> Self{
        Self::new(TokenKind::Plus)
    }

    pub fn astarisk()-> Self{
        Self::new(TokenKind::Asterisk)
    }

    pub fn slash()-> Self{
        Self::new(TokenKind::Slash)
    }
    
}

pub fn lex(form1: &str) -> Result<Vec<Token>, LexError>{
    let length=form1.len() as i32;
    let mut Tokens= Vec::new();
    let mut pos=0;
    while pos<form1.len(){
        pos+=1;
    }
    execute!(std::io::stdout(),Print(form1), Print("\r\n")).ok();
    return Ok(Tokens);
}