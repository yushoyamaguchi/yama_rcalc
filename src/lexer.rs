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

    pub fn minus()-> Self{
        Self::new(TokenKind::Minus)
    }

    pub fn astarisk()-> Self{
        Self::new(TokenKind::Asterisk)
    }

    pub fn slash()-> Self{
        Self::new(TokenKind::Slash)
    }
    
}

pub struct Lexer{
    pub Tokens:Vec<Token>,
}

impl Lexer{
    pub fn new()->Self{
        Self { Tokens: Vec::new() }
    }

    pub fn lex_number(&mut self,input: &[u8],pos: usize) ->  usize{
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
        self.Tokens.push(Token::number(num));
        execute!(std::io::stdout(),Print(num_str), Print("\r\n")).ok();
        return current_pos;
    }

    pub fn lex_plus(&mut self,input: &[u8],pos: usize) ->  usize{
        let mut current_pos=pos;
        current_pos+=1;
        self.Tokens.push(Token::plus());
        execute!(std::io::stdout(),Print("plus"), Print("\r\n")).ok();
        return current_pos;
    }

    pub fn lex_minus(&mut self,input: &[u8],pos: usize) ->  usize{
        let mut current_pos=pos;
        current_pos+=1;
        self.Tokens.push(Token::minus());
        execute!(std::io::stdout(),Print("minus"), Print("\r\n")).ok();
        return current_pos;
    }

    pub fn lex_astarisk(&mut self,input: &[u8],pos: usize) ->  usize{
        let mut current_pos=pos;
        current_pos+=1;
        self.Tokens.push(Token::astarisk());
        execute!(std::io::stdout(),Print("astarisk"), Print("\r\n")).ok();
        return current_pos;
    }

    pub fn lex_slash(&mut self,input: &[u8],pos: usize) ->  usize{
        let mut current_pos=pos;
        current_pos+=1;
        self.Tokens.push(Token::slash());
        execute!(std::io::stdout(),Print("slash"), Print("\r\n")).ok();
        return current_pos;
    }
    
    pub fn lex(&mut self,form1: &str) {
        let length=form1.len() as i32;
        let mut pos:usize=0;
        let input=form1.as_bytes();
        while pos<form1.len(){
            match input[pos]{
                b'0'..=b'9' => pos=Lexer::lex_number(self,input,pos),
                b'+' =>pos=Lexer::lex_plus(self, input, pos),
                b'-' =>pos=Lexer::lex_minus(self, input, pos),
                b'*' =>pos=Lexer::lex_astarisk(self, input, pos),
                b'/' =>pos=Lexer::lex_slash(self, input, pos),
                b' ' | b'\n' | b'\t' => pos+=1,
                b => pos+=1,
            };
        }
        execute!(std::io::stdout(),Print(form1), Print("\r\n")).ok();
    }
}
