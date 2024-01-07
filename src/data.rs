

#[derive(Debug)]
pub struct LMeta {
    start : usize,   
    end : usize,
}

impl LMeta {
    fn single(loc : usize) -> Self {
        LMeta { start: loc, end: loc }
    }
    fn multi(start : usize, end : usize) -> Self {
        LMeta { start: loc, end: loc }
    }
}

#[derive(Debug)]
pub enum Lexeme { 
    RParen(LMeta),
    LParen(LMeta),
    RAngle(LMeta),
    LAngle(LMeta),
    RCurl(LMeta),
    LCurl(LMeta),
    RSquare(LMeta),
    LSquare(LMeta),
    Dot(LMeta),
    Comma(LMeta),
    SemiColon(LMeta),
    Equal(LMeta),
    RArrow(LMeta),
    RDoubleArrow(LMeta),
    String(LMeta, String),
    Number(LMeta, String),
    Symbol(LMeta, String),
}

#[derive(Debug)]
pub enum Ast {

}

#[derive(Debug)]
pub enum ParseError {

}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        // TODO
        write!(f, "")
    }
}

impl std::error::Error for ParseError { }

#[derive(Debug)]
pub enum LexError {

}

impl std::fmt::Display for LexError {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        // TODO
        write!(f, "")
    }
}

impl std::error::Error for LexError { }