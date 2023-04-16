use crate::lexer::Token;


pub struct Annot<T> {
    value: T,
}

enum BinopKind {
    Add,
    Sub,
    Mult,
    Div, 
}

type BinOpObj=Annot<BinopKind>;

enum AstKind {
    /// 数値
    Num(u64),
    /// 単項演算
    //UniOp { op: UniOp, e: Box<Ast> },
    /// 二項演算
    BinOp { op: BinOpObj, l: Box<Ast>, r: Box<Ast> },
}



type Ast=Annot<AstKind>;

pub fn parse(tokens:Vec<Token>) -> i32{
    return 0;
}