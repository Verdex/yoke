
use crate::data::{LMeta, Lexeme};

pub struct LexGrouper<T> {
    input : T,
    pattern : Vec<Pattern>,
    label : String,
    match_buffer : Vec<Lexeme>,
}

impl<T : Iterator<Item = Lexeme>> Iterator for LexGrouper<T> {
    type Item = Lexeme;

    fn next(&mut self) -> Option<Self::Item> {
        match self.input.next() {
            None => { },
            Some(l) => { self.match_buffer.push(l); },
        }
        if pattern_match(&self.pattern, &self.match_buffer) {
            let ls = std::mem::replace(&mut self.match_buffer, vec![]);
            let start = ls.first().unwrap().meta().start;
            let end = ls.last().unwrap().meta().end;
            Some(Lexeme::Group(LMeta::multi(start, end), self.label.clone(), ls))
        }
        else {
            todo!()
        }
    }
}

fn pattern_match(pattern : &[Pattern], data : &[Lexeme]) -> bool {
    todo!()
}

#[derive(Debug)]
pub enum Pattern {
    Wild,
    Exact(Lexeme),
    Pred(fn(Lexeme) -> bool),
}

pub fn group<T : Iterator<Item = Lexeme>>(label : String, pattern : Vec<Pattern>, input : T) -> LexGrouper<T> { 
    // TODO don't allow empty patterns
    LexGrouper { input, pattern, label, match_buffer: vec![] }
}