
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
        loop {
            match self.input.next() {
                None if self.match_buffer.len() == 0 => { break None; },
                None if self.match_buffer.len() != self.pattern.len() => { 
                    let ret = self.match_buffer.remove(0);
                    break Some(ret); 
                },
                None if !pattern_match(&self.pattern, &self.match_buffer) => { 
                    let ret = self.match_buffer.remove(0);
                    break Some(ret); 
                },
                Some(l) => { self.match_buffer.push(l); },
                _ => unreachable!(),
            }
            if self.match_buffer.len() < self.pattern.len() {
                continue;
            }
            else if self.match_buffer.len() > self.pattern.len() {
                break Some(self.match_buffer.remove(0));
            }
            else if pattern_match(&self.pattern, &self.match_buffer) {
                let ls = std::mem::replace(&mut self.match_buffer, vec![]);
                let start = ls.first().unwrap().meta().start;
                let end = ls.last().unwrap().meta().end;
                break Some(Lexeme::Group(LMeta::multi(start, end), self.label.clone(), ls));
            }
        }
    }
}

fn pattern_match(pattern : &[Pattern], data : &[Lexeme]) -> bool {
    let pds = pattern.iter().zip(data.iter());
    for pd in pds {
        match pd {
            (Pattern::Wild, _) => { },
            (Pattern::Pred(f), d) if f(d) => { },
            (Pattern::Exact(l), d) if l == d => { },
            _ => { return false; },
        }
    }
    true
}

#[derive(Debug)]
pub enum Pattern {
    Wild,
    Exact(Lexeme),
    Pred(fn(&Lexeme) -> bool),
}

pub fn group<T : Iterator<Item = Lexeme>>(label : String, pattern : Vec<Pattern>, input : T) -> LexGrouper<T> { 
    // TODO don't allow empty patterns
    LexGrouper { input, pattern, label, match_buffer: vec![] }
}