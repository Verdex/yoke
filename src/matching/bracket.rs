
use crate::data::Bracket;

#[derive(Debug)]
pub enum Pattern {
    Wild,
    Exact(Bracket),
    Pred(fn(&Bracket) -> bool),
}

fn pattern_match(pattern : &[Pattern], data : &[Bracket]) -> bool {
    let pds = pattern.iter().zip(data.iter());
    for pd in pds {
        match pd {
            (Pattern::Wild, _) => { },
            (Pattern::Pred(f), d) if f(d) => { },
            (Pattern::Exact(l), d) if l.lmatch(d) => { },
            _ => { return false; },
        }
    }
    true
}

pub struct Rule<T> {
    pattern : Vec<Pattern>, 
    f : fn(&[Bracket]) -> T,
}

impl<T> Rule<T> {
    pub fn new(pattern : Vec<Pattern>, f : fn(&[Bracket]) -> T) -> Rule<T> {
        Rule { pattern, f }
    }
}

impl<T> From<(Vec<Pattern>, fn(&[Bracket]) -> T)> for Rule<T> {
    fn from(value : (Vec<Pattern>, fn(&[Bracket]) -> T)) ->  Self {
        Rule::new(value.0, value.1)
    }
}

#[derive(Debug)]
pub enum BracketProcessError {
}

impl std::fmt::Display for BracketProcessError {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
    }
}

impl std::error::Error for BracketProcessError { }

pub fn process<T, I : Iterator<Item = Bracket>>(rules : &[Rule<T>], input : I) -> Result<Vec<T>, BracketProcessError> {

    
       todo!() 
}
