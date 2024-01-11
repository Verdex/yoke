

#[derive(Debug, Clone, PartialEq)]
pub struct LMeta {
    start : usize,   
    end : usize,
}

impl LMeta {
    pub fn single(loc : usize) -> Self {
        LMeta { start: loc, end: loc }
    }
    pub fn multi(start : usize, end : usize) -> Self {
        LMeta { start, end }
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
    Dollar(LMeta),
    Caret(LMeta),
    RArrow(LMeta),
    RDoubleArrow(LMeta),
    String(LMeta, String),
    Number(LMeta, String),
    Symbol(LMeta, String),
}

impl Lexeme {
    pub fn meta(&self) -> LMeta {
        use Lexeme::*;
        match self {
            RParen(m) => m.clone(),
            LParen(m) => m.clone(),
            RAngle(m) => m.clone(),
            LAngle(m) => m.clone(),
            RCurl(m) => m.clone(),
            LCurl(m) => m.clone(),
            RSquare(m) => m.clone(),
            LSquare(m) => m.clone(),
            Dot(m) => m.clone(),
            Comma(m) => m.clone(),
            SemiColon(m) => m.clone(),
            Equal(m) => m.clone(),
            Dollar(m) => m.clone(),
            Caret(m) => m.clone(),
            RArrow(m) => m.clone(),
            RDoubleArrow(m) => m.clone(),
            String(m, _) => m.clone(),
            Number(m, _) => m.clone(),
            Symbol(m, _) => m.clone(),
        }
    }
    pub fn value(&self) -> String {
        use Lexeme::*;
        match self {
            RParen(_) => ")".into(),
            LParen(_) => "(".into(),
            RAngle(_) => ">".into(),
            LAngle(_) => "<".into(),
            RCurl(_) => "}".into(),
            LCurl(_) => "{".into(),
            RSquare(_) => "]".into(),
            LSquare(_) => "[".into(),
            Dot(_) => ".".into(),
            Comma(_) => ",".into(),
            SemiColon(_) => ";".into(),
            Equal(_) => "=".into(),
            Dollar(_) => "$".into(),
            Caret(_) => "^".into(),
            RArrow(_) => "->".into(),
            RDoubleArrow(_) => "=>".into(),
            String(_, s) => s.clone(),
            Number(_, n) => n.clone(),
            Symbol(_, sym) => sym.clone(),
        }
    }
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
    EncounteredEndInString,
    UnexpectedEscapeInString(usize, char),
    UnexpectedToken(usize, char),
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LexError::EncounteredEndInString => write!(f, "Encountered end of file while lexing string."),
            LexError::UnexpectedEscapeInString(index, c) => write!(f, "Encountered unexpected escape in string: {}::{}", index, c),
            LexError::UnexpectedToken(index, c) => write!(f, "Encountered unexpected token {} at {}", c, index),
        }
    }
}

impl std::error::Error for LexError { }