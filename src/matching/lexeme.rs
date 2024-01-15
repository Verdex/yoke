
use crate::data::{LMeta, Lexeme};

pub struct LexGrouper<T, const N : usize> {
    input : T,
    pattern : [Pattern; N],
    label : String,
    match_buffer : Vec<Lexeme>,
}

impl<T : Iterator<Item = Lexeme>, const N : usize> Iterator for LexGrouper<T, N> {
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
            (Pattern::Exact(l), d) if l.lmatch(d) => { },
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

pub fn grouper<T : Iterator<Item = Lexeme>, S : AsRef<str>, const N : usize>(label : S, pattern : [Pattern; N], input : T) -> LexGrouper<T, N> { 
    LexGrouper { input, pattern, label: label.as_ref().to_string(), match_buffer: vec![] }
}

pub fn r_paren() -> Lexeme { Lexeme::RParen(LMeta::new()) } 
pub fn l_paren() -> Lexeme { Lexeme::LParen(LMeta::new()) } 
pub fn r_angle() -> Lexeme { Lexeme::RAngle(LMeta::new()) } 
pub fn l_angle() -> Lexeme { Lexeme::LAngle(LMeta::new()) } 
pub fn r_curl() -> Lexeme { Lexeme::RCurl(LMeta::new()) } 
pub fn l_curl() -> Lexeme { Lexeme::LCurl(LMeta::new()) } 
pub fn r_square() -> Lexeme { Lexeme::RSquare(LMeta::new()) } 
pub fn l_square() -> Lexeme { Lexeme::LSquare(LMeta::new()) } 
pub fn punct(c : char) -> Lexeme { Lexeme::Punct(LMeta::new(), c) }
pub fn group<S : AsRef<str>>(label : S, ls : Vec<Lexeme>) -> Lexeme { Lexeme::Group(LMeta::new(), label.as_ref().to_string(), ls) }
pub fn string<S : AsRef<str>>(s : S) -> Lexeme { Lexeme::String(LMeta::new(), s.as_ref().to_string()) }
pub fn number<S : AsRef<str>>(s : S) -> Lexeme { Lexeme::Number(LMeta::new(), s.as_ref().to_string()) }
pub fn symbol<S : AsRef<str>>(s : S) -> Lexeme { Lexeme::Symbol(LMeta::new(), s.as_ref().to_string()) }

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::Lexeme;
    use crate::parsing::lexer;

    #[test]
    fn should_group_with_zero_length_input() {
        let input = "";
        let tokens = lexer::lex(&input).unwrap();
        let output = grouper("label", [Pattern::Wild], tokens.into_iter()).collect::<Vec<_>>();
        assert_eq!(output.len(), 0);
    }

    #[test]
    fn should_group_with_zero_length_pattern() {
        let input = "1 2 3";
        let tokens = lexer::lex(&input).unwrap();
        let output = grouper("label", [], tokens.into_iter()).collect::<Vec<_>>();
        assert_eq!(output.len(), 3);
        assert!(matches!(output[0], Lexeme::Number(_, _)));
        assert!(matches!(output[1], Lexeme::Number(_, _)));
        assert!(matches!(output[2], Lexeme::Number(_, _)));
    }

    #[test]
    fn should_group_with_wild_pattern() {
        let input = "1 2 3";
        let tokens = lexer::lex(&input).unwrap();
        let output = grouper("label", [Pattern::Wild, Pattern::Wild], tokens.into_iter()).collect::<Vec<_>>();
        assert_eq!(output.len(), 2);
        assert!(matches!(output[0], Lexeme::Group(_, _, _)));
        assert!(matches!(output[1], Lexeme::Number(_, _)));

        if let Lexeme::Group(meta, label, ls) = &output[0] {
            assert_eq!(meta.start, 0);
            assert_eq!(meta.end, 2);
            assert_eq!(label, "label");
            assert_eq!(ls.len(), 2);
        }

        if let Lexeme::Number(_, n) = &output[1] { 
            assert_eq!(n, "3");
        }
    }

    #[test]
    fn should_group_with_exact_pattern() {
        let input = "1 2 3 4";
        let tokens = lexer::lex(&input).unwrap();
        let output = grouper("label", [Pattern::Exact(number("1")), Pattern::Wild], tokens.into_iter()).collect::<Vec<_>>();
        assert_eq!(output.len(), 3);
        assert!(matches!(output[0], Lexeme::Group(_, _, _)));
        assert!(matches!(output[1], Lexeme::Number(_, _)));
        assert!(matches!(output[2], Lexeme::Number(_, _)));

        if let Lexeme::Group(meta, label, ls) = &output[0] {
            assert_eq!(meta.start, 0);
            assert_eq!(meta.end, 2);
            assert_eq!(label, "label");
            assert_eq!(ls.len(), 2);
        }

        if let Lexeme::Number(_, n) = &output[1] { 
            assert_eq!(n, "3");
        }

        if let Lexeme::Number(_, n) = &output[2] { 
            assert_eq!(n, "4");
        }
    }

    #[test]
    fn should_group_with_pred_pattern() {
        fn odd(l : &Lexeme) -> bool {
            match l {
                Lexeme::Number(_, x) => x.parse::<u8>().unwrap() % 2 == 1,
                _ => false, 
            }
        }

        let input = "1 2 3";
        let tokens = lexer::lex(&input).unwrap();
        let output = grouper("label", [Pattern::Pred(odd), Pattern::Wild], tokens.into_iter()).collect::<Vec<_>>();
        assert_eq!(output.len(), 2);
        assert!(matches!(output[0], Lexeme::Group(_, _, _)));
        assert!(matches!(output[1], Lexeme::Number(_, _)));

        if let Lexeme::Group(meta, label, ls) = &output[0] {
            assert_eq!(meta.start, 0);
            assert_eq!(meta.end, 2);
            assert_eq!(label, "label");
            assert_eq!(ls.len(), 2);
        }

        if let Lexeme::Number(_, n) = &output[1] { 
            assert_eq!(n, "3");
        }
    }
}
