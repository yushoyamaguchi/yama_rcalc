use crossterm::{execute, style::Print};

use crate::lexer::{Token, TokenKind};


#[derive(Debug)]
pub struct Annot<T> {
    pub value: T,
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

pub enum ParseErrorKind{
    UnexpectedToken(u32),
}

type ParseError=Annot<ParseErrorKind>;

impl ParseError {
    fn unexpected_token(err_id: u32) -> Self {
        Self::new(ParseErrorKind::UnexpectedToken(err_id))
    }
}

fn parse_factor<>(tokens:& Vec<Token>,pos_param:usize,parent:& mut Option<&mut Term>,parent_is_term:bool)->Result<usize,ParseError>{
    let mut pos=pos_param;
    match tokens[pos].value{
        TokenKind::Number(n) =>{
            let mut myself=Factor::new(n);
            execute!(std::io::stdout(),Print("factor:num="), Print(n), Print("\r\n")).ok();
            if parent_is_term{
                parent.as_mut().unwrap().left=Some(Box::new(myself));
            }
            pos+=1;
            //pos=parse_term(tokens,pos,parent,true);
        }
        TokenKind::Lparen=>{
            //pos+=1;
            //let mut myself=Factor::new_paren();
            //pos=parse_expr(tokens,pos,&mut Some(&mut myself));
            //カッコ非対応ver
            return Err(ParseError::unexpected_token(1));
        }
        TokenKind::Plus|TokenKind::Minus|TokenKind::Asterisk|TokenKind::Slash|TokenKind::Rparen =>{
            //error
            return Err(ParseError::unexpected_token(1));
        }
    }

    return Ok(pos);
}


fn parse_term<>(tokens:& Vec<Token>,pos_param:usize,parent_expr:& mut Option<&mut Expr>,parent_term:& mut Option<&mut Term>,parent_is_expr:bool)->Result<usize,ParseError>{
    let mut pos=pos_param;
    execute!(std::io::stdout(),Print("term"), Print("\r\n")).ok();
    let mut myself=Term::new(None,None,None);
    let result=parse_factor(tokens,pos,&mut Some(&mut myself),true);
    match result {
        Ok(p)=>{
            pos=p;
        }
        Err(e)=>{
            return Err(e);
        }
    }
    if pos>=tokens.len(){
        if parent_is_expr {
            parent_expr.as_mut().unwrap().left=Some(Box::new(myself));
        }
        else{
            parent_term.as_mut().unwrap().right=Some(Box::new(myself));
        }
        return Ok(pos);
    }
    match tokens[pos].value{
        TokenKind::Asterisk =>{
            myself.is_single=false;
            myself.TermOp=Some(Annot::new(TermopKind::Mult));
            pos+=1;
            let mut none_expr:Option<&mut Expr>=Option::None;
            let mut myself_option=Option::Some(&mut myself);
            let result_right=parse_term(tokens, pos, &mut none_expr, &mut myself_option, false);
            match result_right {
                Ok(p)=>{
                    pos=p;
                }
                Err(e)=>{
                    return Err(e);
                }
            }
        }
        TokenKind::Slash=>{
            myself.is_single=false;
            myself.TermOp=Some(Annot::new(TermopKind::Div));
            pos+=1;
            let mut none_expr:Option<&mut Expr>=Option::None;
            let mut myself_option=Option::Some(&mut myself);
            let result_right=parse_term(tokens, pos, &mut none_expr, &mut myself_option, false);
            match result_right {
                Ok(p)=>{
                    pos=p;
                }
                Err(e)=>{
                    return Err(e);
                }
            }
        }
        TokenKind::Plus | TokenKind::Minus=>{
            if parent_is_expr {
                parent_expr.as_mut().unwrap().left=Some(Box::new(myself));
            }
            else{
                parent_term.as_mut().unwrap().right=Some(Box::new(myself));
            }
            return Ok(pos);
        }
        TokenKind::Rparen | TokenKind::Lparen =>{
            return Err(ParseError::unexpected_token(1));
        }
        TokenKind::Number(n) =>{
            return Err(ParseError::unexpected_token(1));
        }
    }

    if parent_is_expr {
        parent_expr.as_mut().unwrap().left=Some(Box::new(myself));
    }
    else{
        parent_term.as_mut().unwrap().right=Some(Box::new(myself));
    }

    return Ok(pos);
}

fn parse_expr(tokens:&Vec<Token>,pos_param:usize,parent_factor_paren:&mut Option<&mut Factor>,parent_expr:&mut Option<&mut Expr>,parent_is_factor:bool)->Result<usize,ParseError>{
    let mut pos=pos_param;
    execute!(std::io::stdout(),Print("expr"), Print("\r\n")).ok();
    let mut myself=Expr::new(None,None,None);
    let result=parse_term(tokens,pos,& mut Some(&mut myself),&mut None,true);
    match result {
        Ok(p)=>{
            pos=p;
        }
        Err(e)=>{
            return Err(e);
        } 
    }
    if pos>=tokens.len(){
        if parent_is_factor==false {
            parent_expr.as_mut().unwrap().right=Some(Box::new(myself));
        }
        return Ok(pos);
    }
    match tokens[pos].value{
        TokenKind::Plus =>{
            myself.is_single=false;
            myself.ExprOp=Some(Annot::new(ExpropKind::Add));
            pos+=1;
            let mut none_factor_paren:Option<&mut Factor>=Option::None;
            let mut myself_option=Option::Some(&mut myself);
            let result_right=parse_expr(tokens, pos, &mut none_factor_paren, &mut myself_option, false);
            match result_right {
                Ok(p)=>{
                    pos=p;
                }
                Err(e)=>{
                    return Err(e);
                } 
            }
        }
        TokenKind::Minus=>{
            myself.is_single=false;
            myself.ExprOp=Some(Annot::new(ExpropKind::Sub));
            pos+=1;
            let mut none_factor_paren:Option<&mut Factor>=Option::None;
            let mut myself_option=Option::Some(&mut myself);
            let result_right=parse_expr(tokens, pos, &mut none_factor_paren, &mut myself_option, false);
            match result_right {
                Ok(p)=>{
                    pos=p;
                }
                Err(e)=>{
                    return Err(e);
                } 
            }
        }
        TokenKind::Rparen | TokenKind::Lparen =>{
            if parent_is_factor==false {
                parent_expr.as_mut().unwrap().right=Some(Box::new(myself));
            }
            return Ok(pos);
        }
        TokenKind::Number(n) =>{
            return Err(ParseError::unexpected_token(1));
        }
        TokenKind::Asterisk | TokenKind::Slash =>{
            return Err(ParseError::unexpected_token(1));
        }
    }
    if parent_is_factor==false {
        parent_expr.as_mut().unwrap().right=Some(Box::new(myself));
    }
    return Ok(pos);
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

    fn root_parse_expr(&mut self,tokens:&Vec<Token>,pos_param:usize)->Result<usize,ParseError>{
        let mut pos=pos_param;
        execute!(std::io::stdout(),Print("root expr"), Print("\r\n")).ok();
        let result=parse_term(tokens,pos,&mut Some(&mut self.root_expr),&mut None,true);
        match result {
            Ok(p)=>{
                pos=p;
            }
            Err(e)=>{
                return Err(e);
            } 
        }
        if pos>=tokens.len(){
            return Ok(pos);
        }
        match tokens[pos].value{
            TokenKind::Plus =>{
                self.root_expr.is_single=false;
                self.root_expr.ExprOp=Some(Annot::new(ExpropKind::Add));
                pos+=1;
                let mut none_expr:Option<&mut Factor>=Option::None;
                let mut myself_option=Option::Some(&mut self.root_expr);
                let result_right=parse_expr(tokens, pos, &mut none_expr, &mut myself_option, false);
                match result_right {
                    Ok(p)=>{
                        pos=p;
                    }
                    Err(e)=>{
                        return Err(e);
                    }
                }
            }
            TokenKind::Minus=>{
                self.root_expr.is_single=false;
                self.root_expr.ExprOp=Some(Annot::new(ExpropKind::Sub));
                pos+=1;
                let mut none_expr:Option<&mut Factor>=Option::None;
                let mut myself_option=Option::Some(&mut self.root_expr);
                let result_right=parse_expr(tokens, pos, &mut none_expr, &mut myself_option, false);
                match result_right {
                    Ok(p)=>{
                        pos=p;
                    }
                    Err(e)=>{
                        return Err(e);
                    }
                }
            }
            TokenKind::Rparen | TokenKind::Lparen |TokenKind::Asterisk | TokenKind::Slash =>{
                return Err(ParseError::unexpected_token(1));
            }
            TokenKind::Number(n) =>{
                return Err(ParseError::unexpected_token(1));
            }
        }

    
        return Ok(pos);
    }

    pub fn parse(&mut self,tokens:&Vec<Token>) -> Option<ParseError>{
        execute!(std::io::stdout(),Print("num of Vec items="),Print(tokens.len()), Print("\r\n")).ok();
        //parse_expr(tokens,0);
        let result=self.root_parse_expr(tokens,0);
        match result {
            Ok(p)=>{
                return None;
            }
            Err(e)=>{
                return Some(e);
            }
        }
    }
}

