

#[derive(Debug, Clone, PartialEq)]
pub struct LMeta {
    pub start : usize,   
    pub end : usize,
}

impl LMeta {
    pub fn new() -> Self { 
        LMeta { start: 0, end: 0 }
    }
    pub fn single(loc : usize) -> Self {
        LMeta { start: loc, end: loc }
    }
    pub fn multi(start : usize, end : usize) -> Self {
        LMeta { start, end }
    }
}

#[derive(Debug, PartialEq, Clone)]
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
    Group(LMeta, String, Vec<Lexeme>),

    String(LMeta, String),
    Number(LMeta, String),
    Symbol(LMeta, String),
}

impl Lexeme {
    pub fn lmatch(&self, other: &Lexeme) -> bool {
        use Lexeme::*;
        match (self, other) {
            (RParen(_), RParen(_)) => true,
            (LParen(_), LParen(_)) => true,
            (RAngle(_), RAngle(_)) => true,
            (LAngle(_), LAngle(_)) => true,
            (RCurl(_), RCurl(_)) => true,
            (LCurl(_), LCurl(_)) => true,
            (RSquare(_), RSquare(_)) => true,
            (LSquare(_), LSquare(_)) => true,
            (Punct(_, c1), Punct(_, c2)) if c1 == c2 => true,
            (Group(_, label1, g1), Group(_, label2, g2)) 
                if 
                label1 == label2 
                && g1.iter().zip(g2.iter()).all(|(a, b)| a.lmatch(b)) => true,

            (String(_, s1), String(_, s2)) if s1 == s2 => true,
            (Number(_, n1), Number(_, n2)) if n1 == n2 => true,
            (Symbol(_, s1), Symbol(_, s2)) if s1 == s2 => true,
            _ => false,
        }
    }
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
            Group(m, _, _) => m.clone(),
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
            Lexeme::Group(_, _, g) => g.iter().map(|x| x.value()).collect::<String>(),
            Lexeme::String(_, s) => s.clone(),
            Lexeme::Number(_, n) => n.clone(),
            Lexeme::Symbol(_, sym) => sym.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Bracket {
    Paren(LMeta, Vec<Bracket>),
    Angle(LMeta, Vec<Bracket>),
    Curl(LMeta, Vec<Bracket>),
    Square(LMeta, Vec<Bracket>),
    Lex(Lexeme),
}

impl Bracket {
    pub fn lmatch(&self, other : &Bracket) -> bool {
        match (self, other) {
            (Bracket::Paren(_, xs), Bracket::Paren(_, ys)) => xs.iter().zip(ys.iter()).map(|(x, y)| x.lmatch(y)).all(|z| z),
            (Bracket::Angle(_, xs), Bracket::Angle(_, ys)) => xs.iter().zip(ys.iter()).map(|(x, y)| x.lmatch(y)).all(|z| z),
            (Bracket::Curl(_, xs), Bracket::Curl(_, ys)) => xs.iter().zip(ys.iter()).map(|(x, y)| x.lmatch(y)).all(|z| z),
            (Bracket::Square(_, xs), Bracket::Square(_, ys)) => xs.iter().zip(ys.iter()).map(|(x, y)| x.lmatch(y)).all(|z| z),
            (Bracket::Lex(x), Bracket::Lex(y)) => x.lmatch(y),
            _ => false,
        }
    }
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
