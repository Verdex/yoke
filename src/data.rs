

#[derive(Debug, Clone, PartialEq)]
pub struct LMeta {
    pub start : usize,   
    pub end : usize,
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

    Punct(LMeta, char),
    Group(LMeta, Vec<Lexeme>),

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
            Punct(m, _) => m.clone(),
            Group(m, _) => m.clone(),
            String(m, _) => m.clone(),
            Number(m, _) => m.clone(),
            Symbol(m, _) => m.clone(),
        }
    }
    pub fn value(&self) -> String {
        match self {
            Lexeme::RParen(_) => ")".into(),
            Lexeme::LParen(_) => "(".into(),
            Lexeme::RAngle(_) => ">".into(),
            Lexeme::LAngle(_) => "<".into(),
            Lexeme::RCurl(_) => "}".into(),
            Lexeme::LCurl(_) => "{".into(),
            Lexeme:: RSquare(_) => "]".into(),
            Lexeme::LSquare(_) => "[".into(),
            Lexeme::Punct(_, c) => c.to_string(),
            Lexeme::Group(_, g) => g.iter().map(|x| x.value()).collect::<String>(),
            Lexeme::String(_, s) => s.clone(),
            Lexeme::Number(_, n) => n.clone(),
            Lexeme::Symbol(_, sym) => sym.clone(),
        }
    }
}

#[derive(Debug)]
pub enum Bracket {
    Paren(LMeta, Vec<Bracket>),
    Angle(LMeta, Vec<Bracket>),
    Curl(LMeta, Vec<Bracket>),
    Square(LMeta, Vec<Bracket>),
    Lex(Lexeme),
}

impl Bracket {
    pub fn meta(&self) -> LMeta {
        use Bracket::*;
        match self {
            Paren(m, _) => m.clone(),
            Angle(m, _) => m.clone(),
            Curl(m, _) => m.clone(),
            Square(m, _) => m.clone(),
            Lex(l) => l.meta(),
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    MissingEndBracket { initial: usize, terminal : usize, found : char, expected : char},
    EofInsteadOfEndBracket { initial: usize, expected : char },
    NotAllInputConsumed(usize),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::MissingEndBracket { initial, terminal, found, expected } => 
                write!(f, "Encountered incorrect bracket at {}.  Expected {}, but found {} matching {}", 
                    terminal, expected, found, initial),
            ParseError::EofInsteadOfEndBracket { initial, expected } => 
                write!(f, "Encountered end of file instead of bracket.  Expected {}, but found end of file matching {}",
                    expected, initial),
            ParseError::NotAllInputConsumed(index) => write!(f, "Not all input consumed during parsing: {}", index),
        }
    }
}

impl std::error::Error for ParseError { }

#[derive(Debug)]
pub enum LexError {
    EncounteredEndInString,
    UnexpectedEscapeInString(usize, char),
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LexError::EncounteredEndInString => write!(f, "Encountered end of file while lexing string."),
            LexError::UnexpectedEscapeInString(index, c) => write!(f, "Encountered unexpected escape in string: {}::{}", index, c),
        }
    }
}

impl std::error::Error for LexError { }