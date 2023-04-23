use crossterm::{execute, style::Print};

use crate::lexer::{Token, TokenKind};


#[derive(Debug)]
pub struct Annot<T> {
    value: T,
}

impl<T> Annot<T> {
    fn new(value: T) -> Self {
        Self { value}
    }
}

enum TermopKind {
    Mult,
    Div,
}

type TermOpObj=Annot<TermopKind>;

enum ExpropKind {
    Add,
    Sub,
}

type ExprOpObj=Annot<ExpropKind>;

struct Factor{
    value:i32,
    is_num:bool,
    in_paren:Option<Box<Expr>>,
}

impl Factor{
    fn new(value:i32)->Factor{
        Factor{
            value:value,
            is_num:true,
            in_paren:None,
        }
    }
    fn new_paren()->Factor{
        Factor{
            value:0,
            is_num:false,
            in_paren:None,
        }
    }
}

struct Term{
    value:i32,
    left:Option<Box<Factor>>,
    right:Option<Box<Term>>,
    TermOp:Option<TermOpObj>,
    is_single:bool,
}

impl Term{
    fn new(left:Option<Box<Factor>>, right:Option<Box<Term>>,Op:Option<TermOpObj>)->Term{
        Term{
            value:0,
            left:left,
            right:right,
            TermOp:Op,
            is_single:true,
        }
    }
}

pub struct Expr{
    value:i32,
    left:Option<Box<Term>>,
    right:Option<Box<Expr>>,
    ExprOp:Option<ExprOpObj>,
    is_single:bool,
}

impl Expr{
    fn new(left:Option<Box<Term>>, right:Option<Box<Expr>>,Op:Option<ExprOpObj>)->Expr{
        Expr{
            value:0,
            left:left,
            right:right,
            ExprOp:Op,
            is_single:true,
        }
    }
}

pub enum ParseErrorKind<'a>{
    UnexpectedToken(&'a  TokenKind),
}

type ParseError<'a>=Annot<ParseErrorKind<'a>>;

impl<'b> ParseError<'b> {
    fn unexpected_token(t: &'b TokenKind) -> Self {
        Self::new(ParseErrorKind::UnexpectedToken(t))
    }
}

fn parse_factor<'a>(tokens:&'a Vec<Token>,pos_param:usize,parent:& mut Term,parent_is_term:bool)->Result<usize,ParseError<'a>>{
    let mut pos=pos_param;
    match tokens[pos].value{
        TokenKind::Number(n) =>{
            let mut myself=Factor::new(n);
            parent.left=Some(Box::new(myself));
            pos+=1;
            //pos=parse_term(tokens,pos,parent,true);
        }
        TokenKind::Lparen=>{
            //pos+=1;
            //let mut myself=Factor::new_paren();
            //pos=parse_expr(tokens,pos,&mut Some(&mut myself));
            //カッコ非対応ver
            return Err(ParseError::unexpected_token(& tokens[pos].value));
        }
        TokenKind::Plus|TokenKind::Minus|TokenKind::Asterisk|TokenKind::Slash|TokenKind::Rparen =>{
            //error
            return Err(ParseError::unexpected_token(& tokens[pos].value));
        }
    }

    return Ok(pos);
}


fn parse_term(tokens:&Vec<Token>,pos:usize,parent_expr:&mut Option<&mut Expr>,parent_is_expr:bool)->usize{
    let mut pos=pos;
    let mut myself=Term::new(None,None,None);
    let result=parse_factor(tokens,pos,&mut myself,true);
    match result {
        Ok(p)=>{
            pos=p;
        }
        Err(e)=>{
            //error
        }
    }


    return pos;
}

fn parse_expr(tokens:&Vec<Token>,pos:usize,parent_factor_paren:&mut Option<&mut Factor>)->usize{
    let mut pos=pos;
    let mut myself=Expr::new(None,None,None);
    pos=parse_term(tokens,pos,& mut Some(&mut myself),true);

    return pos+1;
}


pub struct Parser{
    pub root_expr:Expr,
}

impl Parser{
    pub fn new ()->Parser{
        Parser{
            root_expr:Expr::new(None,None,None),
        }
    }

    fn root_parse_expr(&mut self,tokens:&Vec<Token>,pos:usize)->usize{
        let mut pos=pos;
        pos=parse_term(tokens,pos,&mut Some(&mut self.root_expr),true);
    
        return pos;
    }

    pub fn parse(&mut self,tokens:&Vec<Token>) -> i32{
        execute!(std::io::stdout(),Print("num of Vec items="),Print(tokens.len()), Print("\r\n")).ok();
        //parse_expr(tokens,0);
        self.root_parse_expr(tokens,0);
        return 0;
    }
}

