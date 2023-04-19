use crossterm::{execute, style::Print};

use crate::lexer::Token;


pub struct Annot<T> {
    value: T,
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
    value:u64,
    is_num:bool,
    in_paren:Box<Expr>,
}

struct Term{
    left:Box<Factor>,
    right:Box<Term>,
    TermOp:TermOpObj,
    is_single:bool,
}

struct Expr{
    left:Box<Term>,
    right:Box<Expr>,
    ExprOp:ExprOpObj,
    is_single:bool,
}

impl Expr{

}



fn parse_term(tokens:&Vec<Token>,pos:usize){
}

fn parse_expr(tokens:&Vec<Token>,pos:usize)->usize{
    let mut pos=pos;
    let mut left=parse_term(tokens,pos);

    return pos+1;
}

pub fn parse(tokens:&Vec<Token>) -> i32{
    execute!(std::io::stdout(),Print("num of Vec items="),Print(tokens.len()), Print("\r\n")).ok();
    parse_expr(tokens,0);
    return 0;
}