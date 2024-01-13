
use crate::data::Lexeme;

pub struct LexGrouper<T> {
    input : T,
    pattern : Vec<Pattern>,
    label : String,
}

impl<T : Iterator<Item = Lexeme>> Iterator for LexGrouper<T> {
    type Item = Lexeme;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[derive(Debug)]
pub enum Pattern {
    Wild,
    Exact(Lexeme),
    Pred(fn(Lexeme) -> bool),
}

pub fn group<T : Iterator<Item = Lexeme>>(label : String, pattern : Vec<Pattern>, input : T) -> LexGrouper<T> { 
    LexGrouper { input, pattern, label }
}