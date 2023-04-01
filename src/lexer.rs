use crossterm::style::{Print};
use crossterm::{execute};



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

pub struct Lexer{
    Tokens:Vec<Token>,
}

impl Lexer{
    pub fn new()->Self{
        Self { Tokens: Vec::new() }
    }

    pub fn lex_number(input: &[u8],pos: usize) ->  usize{
        let mut num= 0;
        let mut current_pos=pos;
        while current_pos<input.len(){
            match input[current_pos]{
                b'0'..=b'9' => num=num*10+(input[current_pos] - b'0') as i32,
                b => break,
            };
            current_pos+=1;
        }
        let num_str=num.to_string();
        execute!(std::io::stdout(),Print(num_str), Print("\r\n")).ok();
        return current_pos;
    }
    
    pub fn lex(&self,form1: &str) {
        let length=form1.len() as i32;
        let mut pos:usize=0;
        let input=form1.as_bytes();
        while pos<form1.len(){
            match input[pos]{
                b'0'..=b'9' => pos=Lexer::lex_number(input,pos),
                b => pos+=1,
            };
        }
        execute!(std::io::stdout(),Print(form1), Print("\r\n")).ok();
    }
}
